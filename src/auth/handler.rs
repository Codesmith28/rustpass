
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use crate::models::types::EncryptionKey;


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

