use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};

use crate::models::types::DecryptedDataResult;

use super::encrypt::{FIBBONACI_NUMBERS, NUMERIC_KEY};

pub fn fibbil_unhash(encrypted: &str) -> String {
    let mut original = String::new();
    for (i, char) in encrypted.chars().enumerate() {
        let key_index = (i % NUMERIC_KEY.len()) as usize;
        let fib_index = (NUMERIC_KEY[key_index] - 1) as usize;
        // Use wrapping_sub to handle underflow safely
        let shifted = (((char as u32).wrapping_sub(FIBBONACI_NUMBERS[fib_index] as u32)) % 128) as u8;
        original.push(shifted as char);
    }
    original
}

pub fn decode_codesmith28(encrypted: &str) -> String {
    if encrypted.is_empty() {
        return String::new();
    }

    // Split at the separator
    let parts: Vec<&str> = encrypted.split('|').collect();
    if parts.len() != 2 {
        return String::new(); // Invalid format
    }

    let x = parts[0];
    let y = parts[1];
    
    if y.len() % 3 != 0 {
        return String::new(); // Invalid index format
    }

    // Convert y back to the original positions using fixed-width format
    let mut positions: Vec<(usize, char)> = y
        .chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .filter_map(|chunk| {
            let idx_str: String = chunk.iter().collect();
            idx_str.parse::<usize>().ok()
        })
        .zip(x.chars())
        .collect();
    
    // Sort by original index
    positions.sort_by_key(|&(i, _)| i);
    
    // Build the original string
    positions.into_iter().map(|(_, c)| c).collect()
}


// Decrypt data
pub fn decrypt_data(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8]) -> DecryptedDataResult {
    let cipher = Aes256Gcm::new(key.into());
    cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}