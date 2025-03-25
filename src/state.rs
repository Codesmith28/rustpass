use crate::models::structs::PasswordEntry;
use crate::daemon::client::DaemonClient;
use std::io;
use std::sync::{ Arc, Mutex };

#[derive(Clone)]
pub struct AppState {
    pub unlocked: bool,
    pub passwords: Vec<PasswordEntry>,
    pub encryption_key: [u8; 32],
    pub salt: Vec<u8>,
}

pub struct StateManager {
    state: Arc<Mutex<Option<AppState>>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(None)),
        }
    }

    pub fn is_unlocked(&self) -> bool {
        // First check if we've already unlocked the state locally
        if let Ok(state) = self.state.lock() {
            if state.is_some() && state.as_ref().unwrap().unlocked {
                // Even if locally unlocked, we should check if daemon has locked due to session changes
                if DaemonClient::is_running() {
                    // Fast check with timeout
                    let (tx, rx) = std::sync::mpsc::channel();
                    let _handle = std::thread::spawn(move || {
                        let result = DaemonClient::get_state()
                            .map(|s| s.unlocked)
                            .unwrap_or(true);
                        let _ = tx.send(result);
                    });

                    // If daemon reports locked (e.g., due to session inactivity), update local state
                    if
                        let Ok(daemon_unlocked) = rx.recv_timeout(
                            std::time::Duration::from_millis(100)
                        )
                    {
                        if !daemon_unlocked {
                            // Daemon is locked but we're unlocked - need to sync
                            let _ = self.lock();
                            return false;
                        }
                    }
                    return true;
                }
                return true; // No daemon, use local state
            }
        }

        // Quick check if daemon socket exists before attempting connection
        let socket_path = crate::daemon::ipc::get_socket_path();
        if !socket_path.exists() {
            // Socket doesn't exist, daemon is definitely not running
            return false;
        }

        // Then check with the daemon - but use a fast timeout
        let (tx, rx) = std::sync::mpsc::channel();
        let _handle = std::thread::spawn(move || {
            let result =
                DaemonClient::is_running() &&
                DaemonClient::get_state()
                    .map(|s| s.unlocked)
                    .unwrap_or(false);
            let _ = tx.send(result);
        });

        // Wait with timeout for the result
        if let Ok(state) = rx.recv_timeout(std::time::Duration::from_millis(100)) {
            if state {
                // If daemon has unlocked state, sync it locally
                let _ = self.sync_from_daemon();
                return true;
            }
        }

        false
    }

    pub fn sync_from_daemon(&self) -> io::Result<()> {
        if !DaemonClient::is_running() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Daemon not running"));
        }

        let daemon_state = DaemonClient::get_state()?;

        if
            daemon_state.unlocked &&
            daemon_state.encryption_key.is_some() &&
            daemon_state.salt.is_some()
        {
            // We need to load passwords using the daemon's key
            let key_vec = daemon_state.encryption_key.unwrap();
            let salt = daemon_state.salt.unwrap();

            // Convert Vec<u8> to [u8; 32]
            let mut key = [0u8; 32];
            if key_vec.len() == 32 {
                key.copy_from_slice(&key_vec);
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid key length"));
            }

            // Load the passwords from the password file using the data module
            // We need to implement this function
            let passwords = crate::data::data::load_passwords_with_key(
                crate::PASSWORD_FILE_PATH,
                &key,
                &salt
            )?;

            // Update local state
            let app_state = AppState {
                unlocked: true,
                passwords,
                encryption_key: key,
                salt,
            };

            match self.state.lock() {
                Ok(mut state) => {
                    *state = Some(app_state);
                    Ok(())
                }
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Daemon state not unlocked"))
        }
    }

    pub fn unlock(
        &self,
        passwords: Vec<PasswordEntry>,
        key: [u8; 32],
        salt: Vec<u8>,
        password_opt: Option<&str>
    ) -> io::Result<()> {
        let app_state = AppState {
            unlocked: true,
            passwords,
            encryption_key: key,
            salt: salt.clone(),
        };

        // Update local state
        match self.state.lock() {
            Ok(mut state) => {
                *state = Some(app_state);

                // Also update daemon if it's running and we have the password
                if DaemonClient::is_running() && password_opt.is_some() {
                    let _ = DaemonClient::unlock(password_opt.unwrap());
                }

                Ok(())
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn get_state(&self) -> io::Result<AppState> {
        // Try to sync from daemon first if we don't have local state
        if self.state.lock().unwrap().is_none() {
            let _ = self.sync_from_daemon();
        }

        match self.state.lock() {
            Ok(state) => {
                if let Some(state) = state.as_ref() {
                    Ok(state.clone())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "State not initialized"))
                }
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn lock(&self) -> io::Result<()> {
        // Lock local state
        match self.state.lock() {
            Ok(mut state) => {
                *state = None;

                // Also lock daemon if it's running
                if DaemonClient::is_running() {
                    let _ = DaemonClient::lock();
                }

                Ok(())
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }
}

// Create a singleton instance of StateManager
lazy_static::lazy_static! {
    pub static ref STATE_MANAGER: StateManager = StateManager::new();
}
