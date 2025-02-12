use crate::models::data::PasswordEntry;
use serde_json::Result;
use std::fs;

pub fn load_passwords(file_path: &str) -> Result<Vec<PasswordEntry>> {
    if !std::path::Path::new(file_path).exists() {
        fs::write(file_path, "[]").expect("Failed to create default JSON file");
    }

    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let passwords: Vec<PasswordEntry> = serde_json::from_str(&data)?;
    // DEBUG: Print passwords
    log::debug!("{:#?}", passwords);
    Ok(passwords)
}
