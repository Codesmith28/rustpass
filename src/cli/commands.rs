use crate::models::structs::{ Metadata, PasswordEntry };
use crate::state::STATE_MANAGER;
use crate::data::data::{ load_passwords, save_passwords };
use crate::PASSWORD_FILE_PATH;
use std::io;
use rpassword::read_password;
use crate::daemon::client::DaemonClient;

pub fn execute_unlock(password_opt: Option<String>) -> io::Result<()> {
    if let Some(password) = password_opt {
        // If we have a password, use it to unlock
        match load_passwords(PASSWORD_FILE_PATH, &password) {
            Ok((passwords, key, salt)) => {
                // Pass password to StateManager so it can unlock the daemon too
                STATE_MANAGER.unlock(passwords, key, salt, Some(&password))?;
                println!("Password store unlocked");
                Ok(())
            }
            Err(e) => {
                println!("Failed to unlock: {}", e);
                Err(io::Error::new(io::ErrorKind::InvalidInput, e))
            }
        }
    } else {
        // No password provided, try to use daemon state if available
        if DaemonClient::is_running() {
            match DaemonClient::get_state() {
                Ok(state) if state.unlocked => {
                    // Daemon is unlocked, sync local state
                    match STATE_MANAGER.sync_from_daemon() {
                        Ok(_) => {
                            println!("Password store unlocked using daemon state");
                            Ok(())
                        }
                        Err(e) => {
                            println!("Failed to sync from daemon: {}", e);
                            Err(e)
                        }
                    }
                }
                _ => {
                    println!("Cannot unlock without password (daemon state is locked)");
                    Err(io::Error::new(io::ErrorKind::PermissionDenied, "Password required"))
                }
            }
        } else {
            println!("Cannot unlock without password (daemon not running)");
            Err(io::Error::new(io::ErrorKind::PermissionDenied, "Password required"))
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
        &state.salt
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Update the state
    STATE_MANAGER.unlock(state.passwords, state.encryption_key, state.salt, None)?;

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
        return Err(
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Password with name '{}' not found", name)
            )
        );
    }

    // Save the updated passwords
    save_passwords(
        PASSWORD_FILE_PATH,
        &state.passwords,
        &state.encryption_key,
        &state.salt
    ).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

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
    println!("  daemon start                       Start the daemon process");
    println!("  daemon stop                        Stop the daemon process");
    println!("  daemon status                      Check the status of the daemon");
    println!("  help                               Show this help message");
    Ok(())
}
