use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: String,
    pub id: String,
    pub password: String,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
pub struct EncryptedFile {
    pub salt: String,
    pub nonce: String,
    pub encrypted_data: String,
}