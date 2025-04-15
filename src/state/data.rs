use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::models::types::{DecryptedDataResult, EncryptedDataResult};

// State file structure
#[derive(Serialize, Deserialize)]
pub struct AppStateFile {
    pub unlocked: bool,
}

// Encrypted state file format
#[derive(Serialize, Deserialize)]
pub struct EncryptedStateFile {
    pub nonce: String,
    pub encrypted_data: String,
}

// Get state file path
pub fn get_state_file_path() -> PathBuf {
    let base_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("rustpass");
    fs::create_dir_all(&base_dir).expect("Failed to create state directory");
    base_dir.join("state.enc")
}

// Derive fixed key for state encryption
fn derive_state_key() -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update("rustpass_state_key_v1");
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

// Encrypt state data
fn encrypt_state(data: &[u8]) -> EncryptedDataResult {
    let key = derive_state_key();
    let cipher = Aes256Gcm::new(&key.into());
    let mut nonce = [0u8; 12];
    for byte in &mut nonce {
        *byte = rand::random();
    }
    let ciphertext = cipher
        .encrypt(&nonce.into(), data)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    Ok((nonce.to_vec(), ciphertext))
}

// Decrypt state data
fn decrypt_state(ciphertext: &[u8], nonce: &[u8]) -> DecryptedDataResult {
    let key = derive_state_key();
    let cipher = Aes256Gcm::new(&key.into());
    cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}

// Save state to file
pub fn save_state(unlocked: bool) -> io::Result<()> {
    let state = AppStateFile { unlocked };
    let data = serde_json::to_vec(&state).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Serialization failed: {}", e))
    })?;

    let (nonce, encrypted_data) = encrypt_state(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Encryption failed: {}", e)))?;

    let encrypted_file = EncryptedStateFile {
        nonce: STANDARD.encode(&nonce),
        encrypted_data: STANDARD.encode(&encrypted_data),
    };

    let path = get_state_file_path();
    let mut file = File::create(&path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to create file: {}", e),
        )
    })?;
    file.write_all(serde_json::to_string(&encrypted_file).unwrap().as_bytes())
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to write file: {}", e))
        })?;

    // Set file permissions to 600 (owner read/write only)
    #[cfg(unix)]
    {
        let perms = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&path, perms).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to set permissions: {}", e),
            )
        })?;
    }

    Ok(())
}

// Load state from file
pub fn load_state() -> io::Result<bool> {
    let path = get_state_file_path();
    if !path.exists() {
        return Ok(false); // Default to locked if file doesn't exist
    }

    let mut file = File::open(&path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to open file: {}", e),
        )
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to read file: {}", e),
        )
    })?;

    let encrypted: EncryptedStateFile = serde_json::from_str(&contents).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid file format: {}", e),
        )
    })?;

    let nonce = STANDARD
        .decode(&encrypted.nonce)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid nonce: {}", e)))?;

    let encrypted_data = STANDARD.decode(&encrypted.encrypted_data).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid encrypted data: {}", e),
        )
    })?;

    let decrypted = decrypt_state(&encrypted_data, &nonce).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Decryption failed: {}", e),
        )
    })?;

    let state: AppStateFile = serde_json::from_slice(&decrypted).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse state: {}", e),
        )
    })?;

    Ok(state.unlocked)
}
