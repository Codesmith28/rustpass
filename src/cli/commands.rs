use crate::data::data::{load_passwords, save_passwords};
use crate::models::structs::{Metadata, PasswordEntry};
use crate::state::key::save_key;
use crate::state::manager::STATE_MANAGER;
use crate::PASSWORD_FILE_PATH;
use rpassword::read_password;
use std::io::{self, Write};

pub fn execute_unlock(password_opt: Option<String>) -> io::Result<()> {
    let password = match password_opt {
        Some(p) => p,
        None => {
            print!("Enter master password: ");
            std::io::stdout().flush()?;
            read_password()?
        }
    };

    // Unlock with provided password
    match load_passwords(PASSWORD_FILE_PATH, &password) {
        Ok((passwords, key, salt)) => {
            // Save the key before unlocking to ensure it's available
            save_key(&password)?;
            STATE_MANAGER.unlock(passwords, key, salt, Some(&password))?;
            println!("Password store unlocked");
            Ok(())
        }
        Err(e) => {
            println!("Failed to unlock: {}", e);
            Err(io::Error::new(io::ErrorKind::InvalidInput, e))
        }
    }
}

pub fn execute_lock() -> io::Result<()> {
    STATE_MANAGER.lock()?;
    println!("App locked successfully");
    Ok(())
}

pub fn execute_add(name: String, password: String) -> io::Result<()> {
    STATE_MANAGER.ensure_unlocked()?;

    let mut state = STATE_MANAGER.get_state()?;

    // Create a new password entry
    let new_entry = PasswordEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        password,
        metadata: Metadata::default(),
    };

    state.passwords.push(new_entry);

    // Save the updated passwords
    save_passwords(
        PASSWORD_FILE_PATH,
        &state.passwords,
        &state.encryption_key,
        &state.salt,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Update the state
    STATE_MANAGER.unlock(state.passwords, state.encryption_key, state.salt, None)?;

    println!("Password added successfully");
    Ok(())
}

pub fn execute_list() -> io::Result<()> {
    STATE_MANAGER.ensure_unlocked()?;

    let state = STATE_MANAGER.get_state()?;

    if state.passwords.is_empty() {
        println!("No passwords found");
        return Ok(());
    }

    println!("Passwords:");
    for (i, entry) in state.passwords.iter().enumerate() {
        println!("{}: {}", i + 1, entry.name);
    }

    Ok(())
}

pub fn execute_remove(name: String) -> io::Result<()> {
    STATE_MANAGER.ensure_unlocked()?;

    let mut state = STATE_MANAGER.get_state()?;

    let initial_count = state.passwords.len();
    state.passwords.retain(|entry| entry.name != name);

    if state.passwords.len() == initial_count {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Password with name '{}' not found", name),
        ));
    }

    // Save the updated passwords
    save_passwords(
        PASSWORD_FILE_PATH,
        &state.passwords,
        &state.encryption_key,
        &state.salt,
    )
    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Update the state
    STATE_MANAGER.unlock(state.passwords, state.encryption_key, state.salt, None)?;

    println!("Password removed successfully");
    Ok(())
}

pub fn execute_help() -> io::Result<()> {
    println!("RustPass - Password Manager");
    println!("Usage: rsp <command> [options]");
    println!("");
    println!("Commands:");
    println!("  add <name> <username> <password>   Add a new password entry");
    println!("  list                               List all password entries");
    println!("  remove <name>                      Remove a password entry");
    println!("  unlock [password]                  Unlock the password database");
    println!("  lock                               Lock the password database");
    println!("  tui                                Launch the terminal UI");
    println!("  help                               Show this help message");
    Ok(())
}
