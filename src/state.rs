use crate::models::structs::PasswordEntry;
use std::io;
use std::sync::{Arc, Mutex};

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
        match self.state.lock() {
            Ok(state) => state.is_some() && state.as_ref().unwrap().unlocked,
            Err(_) => false,
        }
    }

    pub fn unlock(&self, passwords: Vec<PasswordEntry>, key: [u8; 32], salt: Vec<u8>) -> io::Result<()> {
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

    pub fn get_state(&self) -> io::Result<AppState> {
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
        match self.state.lock() {
            Ok(mut state) => {
                *state = None;
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