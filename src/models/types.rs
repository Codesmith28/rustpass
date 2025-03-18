use crate::models::structs::PasswordEntry;

/// A vector of password entries loaded from the file.
pub type PasswordEntries = Vec<PasswordEntry>;

/// The encryption key derived from the master password.
pub type EncryptionKey = [u8; 32];

/// A salt used in Argon2 key derivation.
pub type Salt = Vec<u8>;

/// Nonce used in AES-GCM encryption (12 bytes).
pub type Nonce = Vec<u8>;

/// Ciphertext after encryption.
pub type CipherText = Vec<u8>;

/// The result when loading or creating a password file:
/// - Password entries
/// - The derived encryption key
/// - The salt used for key derivation
pub type PasswordDataPayload = (PasswordEntries, EncryptionKey, Salt);

/// General success/error type when working with password files.
pub type PasswordDataResult = Result<PasswordDataPayload, String>;

/// The result of encrypting data (Nonce, Ciphertext).
pub type EncryptedDataResult = Result<(Nonce, CipherText), String>;

/// The result of decrypting data (Decrypted plain bytes).
pub type DecryptedDataResult = Result<Vec<u8>, String>;

/// A generic operation result that returns nothing on success and a `String` error on failure.
pub type OperationResult = Result<(), String>;
