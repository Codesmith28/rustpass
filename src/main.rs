use crossterm::event::{KeyCode, KeyEvent};
use log::error;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use rustpass::models::data::PasswordEntry;
use rustpass::tui;
use rustpass::utils;
use rustpass::PASSWORD_FILE_PATH;
use std::io;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use tui::app::App;
use tui::events::EventHandler;
use tui::data::{load_passwords, create_password_file, save_passwords};
use tui::layout::{setup_terminal, restore_terminal};
use tui::widgets::ui::render_ui;
use utils::logger::init_logger;
use serde_json;
use rand::RngCore;

// Simple password input function (replace with a proper TUI modal if desired)
fn get_master_password(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<String> {
    let mut password = String::new();
    let mut events = EventHandler::new();
    // Create a dummy App instance for event handling during password input
    let mut dummy_app = App::new(Vec::new(), [0; 32], Vec::new());
    loop {
        terminal.draw(|f| {
            let size = f.area();
            f.render_widget(
                ratatui::widgets::Paragraph::new(format!("Enter master password: {}", "*".repeat(password.len())))
                    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
                size,
            );
        })?;
        if let Some(event) = events.next_event(&mut dummy_app) {
            match event {
                KeyEvent { code: KeyCode::Char(c), .. } => password.push(c),
                KeyEvent { code: KeyCode::Backspace, .. } => { password.pop(); },
                KeyEvent { code: KeyCode::Enter, .. } => break,
                KeyEvent { code: KeyCode::Esc, .. } => return Err(io::Error::new(io::ErrorKind::Interrupted, "User cancelled")),
                _ => {}
            }
        }
    }
    Ok(password) // Return the actual input instead of hardcoded value
}

fn main() -> io::Result<()> {
    init_logger();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        default_panic(panic_info);
    }));

    let mut terminal = setup_terminal()?;
    let file_path = PASSWORD_FILE_PATH;

    let result = (|| -> io::Result<(Vec<PasswordEntry>, [u8; 32], Vec<u8>)> {
        if std::path::Path::new(file_path).exists() {
            let mut file = File::open(file_path)?;
            let mut first_char = [0u8; 1];
            file.read_exact(&mut first_char)?;
            file.seek(SeekFrom::Start(0))?;

            if first_char[0] == b'[' {
                let passwords: Vec<PasswordEntry> = serde_json::from_reader(&file)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                terminal.draw(|f| {
                    let size = f.area();
                    f.render_widget(
                        ratatui::widgets::Paragraph::new("Existing unencrypted password file found. Please set a master password to encrypt it.")
                            .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
                        size,
                    );
                })?;
                std::thread::sleep(std::time::Duration::from_secs(2));
                let password = get_master_password(&mut terminal)?;
                let mut salt = [0u8; 16];
                rand::rng().fill_bytes(&mut salt); // Use thread_rng for consistency
                let key: [u8; 32] = tui::data::derive_key(&password, &salt)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                save_passwords(file_path, &passwords, &key, &salt)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok((passwords, key, salt.to_vec()))
            } else {
                loop {
                    let password = get_master_password(&mut terminal)?;
                    match load_passwords(file_path, &password) {
                        Ok((passwords, key, salt)) => return Ok((passwords, key, salt)),
                        Err(e) => {
                            terminal.draw(|f| {
                                let size = f.area();
                                f.render_widget(
                                    ratatui::widgets::Paragraph::new(format!("Error: Invalid password - {}", e))
                                        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
                                    size,
                                );
                            })?;
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        }
                    }
                }
            }
        } else {
            terminal.draw(|f| {
                let size = f.area();
                f.render_widget(
                    ratatui::widgets::Paragraph::new("No password file found. Setting up new file...")
                        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
                    size,
                );
            })?;
            std::thread::sleep(std::time::Duration::from_secs(1));
            let password = get_master_password(&mut terminal)?;
            create_password_file(file_path, &password).map_err(|e| {
                error!("Failed to create password file: {}", e);
                io::Error::new(io::ErrorKind::Other, e)
            })
        }
    })();

    let (passwords, key, salt) = match result {
        Ok(data) => data,
        Err(e) => {
            restore_terminal()?;
            return Err(e);
        }
    };

    let mut app = App::new(passwords, key, salt);
    let mut events = EventHandler::new();

    while app.running {
        terminal.draw(|f| render_ui(f, &app))?;
        if let Some(_key) = events.next_event(&mut app) {}
    }

    restore_terminal()?;
    Ok(())
}