use rustpass::encryption::{
    decrypt::{decode_codesmith28, fibbil_unhash},
    encrypt::{codesmith28, fibbil_hash},
};

#[test]
fn test_fibbil_encryption_decryption() {
    let test_cases = vec![
        "Hello, World!",
        "",
        "12345",
        "!@#$%^&*()",
        "This is a longer test string with various characters: 123!@",
    ];

    for original in test_cases {
        let encrypted = fibbil_hash(original);
        let decrypted = fibbil_unhash(&encrypted);
        assert_eq!(
            decrypted, original,
            "Fibbil encryption/decryption failed for: {}",
            original
        );
    }
}

#[test]
fn test_codesmith28_encryption_decryption() {
    let test_cases = vec!["rustlang", "", "12345", "aaaaaa", "This is a test", "!@#$%"];

    for original in test_cases {
        let encrypted = codesmith28(original);
        let decrypted = decode_codesmith28(&encrypted);
        assert_eq!(
            decrypted, original,
            "Codesmith28 encryption/decryption failed for: {}",
            original
        );
    }
}

#[test]
fn test_codesmith28_invalid_input() {
    assert_eq!(decode_codesmith28(""), "");
    assert_eq!(decode_codesmith28("invalid"), "");
    assert_eq!(decode_codesmith28("no|separator"), "");
    assert_eq!(decode_codesmith28("a|12"), "");
}
