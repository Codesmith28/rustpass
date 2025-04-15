use std::io;
use interprocess::local_socket::LocalSocketStream;
use serde_json::{ from_reader, to_writer };
use std::fs;
use std::path::PathBuf;
use dirs;

use super::ipc::{ DaemonCommand, DaemonResponse, DaemonState, get_socket_path, get_state_path };

pub struct DaemonClient;

impl DaemonClient {
    pub fn get_state() -> io::Result<DaemonState> {
        let socket_path = get_socket_path();

        // Fail fast if socket doesn't exist
        if !socket_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Daemon socket not found"));
        }

        // Connect with error handling
        let mut connection = match LocalSocketStream::connect(socket_path.to_str().unwrap()) {
            Ok(conn) => conn,
            Err(e) => {
                return Err(
                    io::Error::new(
                        io::ErrorKind::ConnectionRefused,
                        format!("Daemon connection failed: {}", e)
                    )
                );
            }
        };

        to_writer(&mut connection, &DaemonCommand::GetState).map_err(|e|
            io::Error::new(io::ErrorKind::Other, e)
        )?;

        let response: DaemonResponse = from_reader(&mut connection).map_err(|e|
            io::Error::new(io::ErrorKind::Other, e)
        )?;

        match response {
            DaemonResponse::StateInfo(state) => Ok(state),
            DaemonResponse::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unexpected response")),
        }
    }

    pub fn unlock(password: &str) -> io::Result<()> {
        let socket_path = get_socket_path();
        let mut connection = LocalSocketStream::connect(socket_path)?;

        to_writer(
            &mut connection,
            &(DaemonCommand::Unlock { password: password.to_string() })
        ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let response: DaemonResponse = from_reader(&mut connection).map_err(|e|
            io::Error::new(io::ErrorKind::Other, e)
        )?;

        match response {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unexpected response")),
        }
    }

    pub fn lock() -> io::Result<()> {
        let socket_path = get_socket_path();
        let mut connection = LocalSocketStream::connect(socket_path)?;

        to_writer(&mut connection, &DaemonCommand::Lock).map_err(|e|
            io::Error::new(io::ErrorKind::Other, e)
        )?;

        let response: DaemonResponse = from_reader(&mut connection).map_err(|e|
            io::Error::new(io::ErrorKind::Other, e)
        )?;

        match response {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Unexpected response")),
        }
    }

    pub fn is_running() -> bool {
        let socket_path = get_socket_path();

        // Quick file check first
        if !socket_path.exists() {
            return false;
        }

        // Try to connect with very short timeout
        match LocalSocketStream::connect(socket_path.to_str().unwrap()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_debug_info() -> io::Result<String> {
        let mut info = String::new();

        // Check socket path
        let socket_path = get_socket_path();
        info.push_str(&format!("Socket path: {:?}\n", socket_path));
        info.push_str(&format!("Socket exists: {}\n", socket_path.exists()));

        // Check state file
        let state_path = get_state_path();
        info.push_str(&format!("State path: {:?}\n", state_path));
        info.push_str(&format!("State file exists: {}\n", state_path.exists()));

        // Check PID file
        let pid_path = dirs
            ::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("rustpass/daemon.pid");
        info.push_str(&format!("PID file: {:?}\n", pid_path));

        if pid_path.exists() {
            if let Ok(pid_data) = fs::read_to_string(pid_path) {
                info.push_str(&format!("PID: {}\n", pid_data.trim()));

                // Check if process is running
                #[cfg(unix)]
                {
                    let pid = pid_data.trim().parse::<u32>().unwrap_or(0);
                    let proc_exists = std::path::Path::new(&format!("/proc/{}", pid)).exists();
                    info.push_str(&format!("Process exists: {}\n", proc_exists));
                }
            }
        } else {
            info.push_str("PID file does not exist\n");
        }

        Ok(info)
    }
}
