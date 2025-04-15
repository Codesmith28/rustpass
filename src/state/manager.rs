use crate::data::data::{load_passwords, save_passwords};
use crate::models::structs::PasswordEntry;
use crate::state::data::{load_state, save_state};
use crate::state::key::{load_key, save_key, delete_key};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use rpassword::read_password;

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
                return true;
            }
        }

        // Then check the state file
        match load_state() {
            Ok(unlocked) => unlocked,
            Err(e) => {
                log::warn!("Failed to load state file: {}. Assuming locked.", e);
                false
            }
        }
    }

    pub fn unlock(
        &self,
        passwords: Vec<PasswordEntry>,
        key: [u8; 32],
        salt: Vec<u8>,
        password_opt: Option<&str>,
    ) -> io::Result<()> {
        let password = match password_opt {
            Some(p) => p.to_string(),
            None => {
                print!("Enter master password: ");
                std::io::stdout().flush()?;
                read_password()?
            }
        };

        // Verify password by attempting to load passwords
        let (loaded_passwords, loaded_key, loaded_salt) =
            load_passwords(crate::PASSWORD_FILE_PATH, &password).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid password: {}", e),
                )
            })?;

        let app_state = AppState {
            unlocked: true,
            passwords: loaded_passwords,
            encryption_key: loaded_key,
            salt: loaded_salt,
        };

        // Update local state
        match self.state.lock() {
            Ok(mut state) => {
                *state = Some(app_state);
                // Update state file
                save_state(true)?;
                // Save master password to key file
                save_key(&password)?;
                Ok(())
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn get_state(&self) -> io::Result<AppState> {
        let mut state_guard = self
            .state
            .lock()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // If in-memory state exists and is unlocked, return it
        if let Some(state) = state_guard.as_ref() {
            if state.unlocked {
                return Ok(state.clone());
            }
        }

        // Check state file
        if load_state()? {
            // State file indicates unlocked, use cached key
            let password = load_key().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Failed to load key: {}. Please unlock again.", e),
                )
            })?;

            let (passwords, key, salt) =
                load_passwords(crate::PASSWORD_FILE_PATH, &password).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Invalid password: {}", e),
                    )
                })?;

            let app_state = AppState {
                unlocked: true,
                passwords,
                encryption_key: key,
                salt,
            };
            *state_guard = Some(app_state.clone());
            Ok(app_state)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "App is locked"))
        }
    }

    pub fn lock(&self) -> io::Result<()> {
        // Prompt for password to verify
        print!("Enter master password to lock: ");
        std::io::stdout().flush()?;
        let password = read_password()?;

        // Verify password
        let _ = load_passwords(crate::PASSWORD_FILE_PATH, &password).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid password: {}", e),
            )
        })?;

        // Lock local state
        match self.state.lock() {
            Ok(mut state) => {
                *state = None;
                // Update state file
                save_state(false)?;
                // Delete key file
                delete_key()?;
                Ok(())
            }
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        }
    }

    pub fn ensure_unlocked(&self) -> io::Result<()> {
        if self.is_unlocked() {
            // Fetch state to ensure it's initialized
            let _ = self.get_state()?;
            return Ok(());
        }

        print!("Enter master password: ");
        std::io::stdout().flush()?;
        let password = read_password()?;

        let (passwords, key, salt) =
            load_passwords(crate::PASSWORD_FILE_PATH, &password).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid password: {}", e),
                )
            })?;

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
    }
}

// Create a singleton instance of StateManager
lazy_static::lazy_static! {
    pub static ref STATE_MANAGER: StateManager = StateManager::new();
}