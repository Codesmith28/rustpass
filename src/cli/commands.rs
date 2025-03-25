use crate::models::structs::{Metadata, PasswordEntry};
use crate::state::STATE_MANAGER;
use crate::data::data::{load_passwords, save_passwords};
use crate::PASSWORD_FILE_PATH;
use std::io;
use rpassword::read_password;

pub fn execute_unlock(password: Option<String>) -> io::Result<()> {
    if STATE_MANAGER.is_unlocked() {
        println!("App is already unlocked");
        return Ok(());
    }

    let password = match password {
        Some(pwd) => pwd,
        None => {
            print!("Enter master password: ");
            io::Write::flush(&mut io::stdout())?;
            read_password()?
        }
    };

    match load_passwords(PASSWORD_FILE_PATH, &password) {
        Ok((passwords, key, salt)) => {
            STATE_MANAGER.unlock(passwords, key, salt)?;
            println!("App unlocked successfully");
            Ok(())
        }
        Err(e) => {
            Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid password: {}", e)))
        }
    }
}

pub fn execute_lock() -> io::Result<()> {
    STATE_MANAGER.lock()?;
    println!("App locked successfully");
    Ok(())
}

pub fn execute_add(name: String, password: String) -> io::Result<()> {
    if !STATE_MANAGER.is_unlocked() {
        execute_unlock(None)?;
    }

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
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    // Update the state
    STATE_MANAGER.unlock(state.passwords, state.encryption_key, state.salt)?;
    
    println!("Password added successfully");
    Ok(())
}

pub fn execute_list() -> io::Result<()> {
    if !STATE_MANAGER.is_unlocked() {
        execute_unlock(None)?;
    }

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
    if !STATE_MANAGER.is_unlocked() {
        execute_unlock(None)?;
    }

    let mut state = STATE_MANAGER.get_state()?;
    
    let initial_count = state.passwords.len();
    state.passwords.retain(|entry| entry.name != name);
    
    if state.passwords.len() == initial_count {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Password with name '{}' not found", name)));
    }
    
    // Save the updated passwords
    save_passwords(
        PASSWORD_FILE_PATH,
        &state.passwords,
        &state.encryption_key,
        &state.salt,
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    // Update the state
    STATE_MANAGER.unlock(state.passwords, state.encryption_key, state.salt)?;
    
    println!("Password removed successfully");
    Ok(())
}

pub fn execute_help() -> io::Result<()> {
    println!("Rustpass - A simple password manager");
    println!();
    println!("Usage:");
    println!("  rsp [COMMAND]");
    println!();
    println!("Commands:");
    println!("  add <name> <username> <password>  Add a new password");
    println!("  list                              List all passwords");
    println!("  remove <name>                     Remove a password");
    println!("  unlock                            Unlock the password manager");
    println!("  lock                              Lock the password manager");
    println!("  help                              Show this help");
    println!("  tui                               Open the TUI");
    Ok(())
} 