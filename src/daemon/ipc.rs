use std::fs::{ self, File };
use std::io::{ self, Read, Write };
use std::path::PathBuf;
use serde::{ Deserialize, Serialize };
use interprocess::local_socket::LocalSocketStream;

// Directory for IPC files
pub fn get_ipc_dir() -> PathBuf {
    // Use XDG_RUNTIME_DIR or equivalent on different platforms
    let base_dir = if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        dirs::runtime_dir().unwrap_or_else(|| {
            dirs::data_dir().expect("Could not find data directory")
        })
    } else {
        // Windows or other platforms
        dirs::data_local_dir().expect("Could not find local data directory")
    };

    let path = base_dir.join("rustpass");
    fs::create_dir_all(&path).expect("Failed to create IPC directory");

    // Make sure directory has correct permissions (0700)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o700);
        let _ = fs::set_permissions(&path, perms);
    }

    path
}

// File paths for IPC
pub fn get_socket_path() -> PathBuf {
    get_ipc_dir().join("daemon.sock")
}

pub fn get_state_path() -> PathBuf {
    get_ipc_dir().join("state.json")
}

// Don't serialize the encryption key directly, store it as Base64 string
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaemonState {
    pub unlocked: bool,
    // Use Vec<u8> instead of array for easier serialization
    pub encryption_key: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
}

impl Default for DaemonState {
    fn default() -> Self {
        Self {
            unlocked: false,
            encryption_key: None,
            salt: None,
        }
    }
}

// Save daemon state to file
pub fn save_daemon_state(state: &DaemonState) -> io::Result<()> {
    let path = get_state_path();
    let json = serde_json::to_string(state)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

// Load daemon state from file
pub fn load_daemon_state() -> io::Result<DaemonState> {
    let path = get_state_path();
    if !path.exists() {
        return Ok(DaemonState::default());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let state: DaemonState = serde_json
        ::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(state)
}

// Command enum for client-daemon communication
#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonCommand {
    Unlock {
        password: String,
    },
    Lock,
    GetState,
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DaemonResponse {
    Success,
    StateInfo(DaemonState),
    Error(String),
}

pub fn cleanup_stale_socket() -> io::Result<()> {
    let socket_path = get_socket_path();

    if socket_path.exists() {
        // Check if socket is actually in use
        match LocalSocketStream::connect(socket_path.to_str().unwrap()) {
            Ok(_) => {
                // Socket is working, leave it alone
            }
            Err(_) => {
                // Socket exists but connection failed - remove it
                log::info!("Removing stale socket file at {:?}", socket_path);
                fs::remove_file(&socket_path)?;
            }
        }
    }

    Ok(())
}
