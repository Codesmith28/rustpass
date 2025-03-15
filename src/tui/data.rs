use crate::models::data::{EncryptedFile, PasswordEntry};
use crate::models::types::{
    DecryptedDataResult, EncryptedDataResult, EncryptionKey, OperationResult, PasswordDataResult,
};
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json;
use std::fs::{set_permissions, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;

pub fn derive_key(password: &str, salt: &[u8]) -> Result<EncryptionKey, String> {
    let hashed = crate::encryption::encrypt::fibbil_hash(password);
    let mut key = [0u8; 32];
    let argon2 = Argon2::default();

    // Convert the raw salt bytes to a SaltString
    let salt_str = SaltString::encode_b64(salt).map_err(|e| format!("Invalid salt: {}", e))?;

    let hash = argon2
        .hash_password(hashed.as_bytes(), &salt_str)
        .map_err(|e| format!("Failed to derive key: {}", e))?;

    key.copy_from_slice(&hash.hash.unwrap().as_bytes()[..32]);

    Ok(key)
}

// Encrypt data
fn encrypt_data(data: &[u8], key: &[u8; 32]) -> EncryptedDataResult {
    let cipher = Aes256Gcm::new(key.into());
    let mut nonce = [0u8; 12];
    for byte in &mut nonce {
        *byte = rand::random();
    }
    let ciphertext = cipher
        .encrypt(&nonce.into(), data)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    Ok((nonce.to_vec(), ciphertext))
}

// Decrypt data
fn decrypt_data(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8]) -> DecryptedDataResult {
    let cipher = Aes256Gcm::new(key.into());
    cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}

pub fn load_passwords(file_path: &str, password: &str) -> PasswordDataResult {
    let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    //log::debug!("File contents: {}", contents);

    let encrypted: EncryptedFile =
        serde_json::from_str(&contents).map_err(|e| format!("Invalid file format: {}", e))?;
    //log::debug!("Encrypted salt: {}", encrypted.salt);

    let salt = STANDARD
        .decode(&encrypted.salt)
        .map_err(|e| format!("Invalid salt: {}", e))?;
    //log::debug!("Decoded salt: {:?}", salt);

    let nonce = STANDARD
        .decode(&encrypted.nonce)
        .map_err(|e| format!("Invalid nonce: {}", e))?;

    let encrypted_data = STANDARD
        .decode(&encrypted.encrypted_data)
        .map_err(|e| format!("Invalid encrypted data: {}", e))?;

    let key = derive_key(password, &salt)?;
    let decrypted = decrypt_data(&encrypted_data, &key, &nonce)?;
    let passwords: Vec<PasswordEntry> = serde_json::from_slice(&decrypted)
        .map_err(|e| format!("Failed to parse passwords: {}", e))?;

    Ok((passwords, key, salt))
}

pub fn save_passwords(
    file_path: &str,
    passwords: &[PasswordEntry],
    key: &[u8; 32],
    salt: &[u8],
) -> OperationResult {
    let data = serde_json::to_vec(passwords).map_err(|e| format!("Serialization failed: {}", e))?;
    let (nonce, encrypted_data) = encrypt_data(&data, key)?;

    let encrypted_file = EncryptedFile {
        salt: STANDARD.encode(salt),
        nonce: STANDARD.encode(&nonce),
        encrypted_data: STANDARD.encode(&encrypted_data),
    };

    let mut file = File::create(file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(serde_json::to_string(&encrypted_file).unwrap().as_bytes())
        .map_err(|e| format!("Failed to write file: {}", e))?;

    // Set file permissions to 600 (owner read/write only)
    let perms = std::fs::Permissions::from_mode(0o600);
    set_permissions(file_path, perms).map_err(|e| format!("Failed to set permissions: {}", e))?;

    Ok(())
}

pub fn create_password_file(file_path: &str, password: &str) -> PasswordDataResult {
    let mut salt = [0u8; 16];
    for byte in &mut salt {
        *byte = rand::random();
    }
    let key = derive_key(password, &salt)?;
    let passwords = Vec::new();
    save_passwords(file_path, &passwords, &key, &salt)?;
    Ok((passwords, key, salt.to_vec()))
}
