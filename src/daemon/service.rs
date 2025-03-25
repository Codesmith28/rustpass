use crate::data::data::load_passwords;
use crate::PASSWORD_FILE_PATH;
use crate::daemon::ipc::{ DaemonState, save_daemon_state, load_daemon_state };
use crate::daemon::session::{ SessionStatus, monitor_session_changes };

use log::{ info, error };
use std::io;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;
use interprocess::local_socket::{ LocalSocketListener, LocalSocketStream };
use serde_json::{ from_reader, to_writer };
use std::fs;
use std::path::PathBuf;
use daemonize;
use env_logger;

use super::ipc::{ DaemonCommand, DaemonResponse, get_socket_path };

pub struct DaemonService {
    state: Arc<Mutex<DaemonState>>,
    running: Arc<Mutex<bool>>,
}

impl DaemonService {
    pub fn new() -> Self {
        let state = match load_daemon_state() {
            Ok(state) => state,
            Err(e) => {
                error!("Failed to load daemon state: {}", e);
                DaemonState::default()
            }
        };

        Self {
            state: Arc::new(Mutex::new(state)),
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start(&self) -> io::Result<()> {
        info!("Initializing daemon service");
        let socket_path = get_socket_path();

        // Remove old socket if it exists
        if socket_path.exists() {
            info!("Removing existing socket file");
            let _ = fs::remove_file(&socket_path);
        }

        info!("Creating socket at {:?}", socket_path);
        let listener = match LocalSocketListener::bind(socket_path.to_str().unwrap()) {
            Ok(l) => {
                info!("Socket created successfully");
                l
            }
            Err(e) => {
                error!("Failed to bind to socket: {}", e);
                return Err(io::Error::new(io::ErrorKind::AddrInUse, e));
            }
        };

        // Set up session monitoring
        info!("Setting up session monitoring");
        let state_clone = self.state.clone();
        monitor_session_changes(move |status| {
            match status {
                SessionStatus::Active => {
                    info!("User session is active - maintaining current lock state");
                    // Don't auto-unlock on session active, just keep current state
                }
                SessionStatus::Inactive => {
                    info!("User session is inactive - locking daemon state");
                    let mut state = state_clone.lock().unwrap();

                    // Only lock if currently unlocked
                    if state.unlocked {
                        state.unlocked = false;
                        state.encryption_key = None;
                        state.salt = None;
                        match save_daemon_state(&state) {
                            Ok(_) => info!("Saved locked state to disk"),
                            Err(e) => error!("Failed to save state: {}", e),
                        }
                    }
                }
            }
        });

        let running_clone = self.running.clone();

        // Accept connections
        info!("Daemon ready, waiting for connections");
        while *self.running.lock().unwrap() {
            match listener.accept() {
                Ok(conn) => {
                    info!("New client connection accepted");
                    let state_clone = self.state.clone();
                    let running_clone = running_clone.clone();

                    thread::spawn(move || {
                        Self::handle_connection(conn, state_clone, running_clone);
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        info!("Daemon shutting down");
        Ok(())
    }

    fn handle_connection(
        mut conn: LocalSocketStream,
        state: Arc<Mutex<DaemonState>>,
        running: Arc<Mutex<bool>>
    ) {
        let cmd: Result<DaemonCommand, _> = from_reader(&mut conn);

        let response = match cmd {
            Ok(DaemonCommand::GetState) => {
                let state = state.lock().unwrap().clone();
                DaemonResponse::StateInfo(state)
            }
            Ok(DaemonCommand::Unlock { password }) => {
                match Self::perform_unlock(password, state.clone()) {
                    Ok(_) => DaemonResponse::Success,
                    Err(e) => DaemonResponse::Error(e.to_string()),
                }
            }
            Ok(DaemonCommand::Lock) => {
                let mut state = state.lock().unwrap();
                state.unlocked = false;
                state.encryption_key = None;
                state.salt = None;
                let _ = save_daemon_state(&state);
                DaemonResponse::Success
            }
            Ok(DaemonCommand::Exit) => {
                *running.lock().unwrap() = false;
                DaemonResponse::Success
            }
            Err(e) => {
                error!("Failed to parse command: {}", e);
                DaemonResponse::Error(format!("Invalid command: {}", e))
            }
        };

        let _ = to_writer(&mut conn, &response);
    }

    fn perform_unlock(password: String, state: Arc<Mutex<DaemonState>>) -> io::Result<()> {
        match load_passwords(PASSWORD_FILE_PATH, &password) {
            Ok((_, key, salt)) => {
                let mut state = state.lock().unwrap();
                state.unlocked = true;
                state.encryption_key = Some(key.to_vec());
                state.salt = Some(salt);
                save_daemon_state(&state)?;
                Ok(())
            }
            Err(e) => { Err(io::Error::new(io::ErrorKind::InvalidInput, e)) }
        }
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn start_daemon() -> io::Result<()> {
        use std::fs::OpenOptions;

        // Create system directories if they don't exist
        let log_dir = dirs
            ::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("rustpass");
        fs::create_dir_all(&log_dir)?;

        // Setup log file
        let log_file = log_dir.join("daemon.log");
        let stdout = OpenOptions::new().create(true).append(true).open(&log_file)?;

        // Setup PID file
        let pid_file = log_dir.join("daemon.pid");

        // Configure the daemon
        let daemonize = daemonize::Daemonize
            ::new()
            .pid_file(pid_file)
            .chown_pid_file(true)
            .working_directory(log_dir)
            .stdout(stdout.try_clone()?)
            .stderr(stdout);

        // Start the daemon
        match daemonize.start() {
            Ok(_) => {
                // Initialize logging
                env_logger::Builder
                    ::from_env(env_logger::Env::default())
                    .filter_level(log::LevelFilter::Info)
                    .format_timestamp_secs()
                    .init();

                info!("Daemon started successfully");

                // Start the service
                let service = DaemonService::new();
                service.start()
            }
            Err(e) => {
                eprintln!("Error starting daemon: {}", e);
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
}
