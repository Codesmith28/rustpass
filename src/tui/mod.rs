pub mod app;
pub mod events;
pub mod keybindings;
pub mod layout;
pub mod widgets;

use crossterm::event::{KeyCode, KeyEvent};
use log::error;
use rand::RngCore;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use crate::{
    auth::handler::derive_key,
    data::data::{create_password_file, load_passwords, save_passwords},
    models::structs::PasswordEntry,
    state::manager::STATE_MANAGER,
    tui::{
        app::App,
        events::EventHandler,
        layout::{restore_terminal, setup_terminal},
        widgets::ui::render_ui,
    },
    PASSWORD_FILE_PATH,
};

// Simple password input function
fn get_master_password(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<String> {
    let mut password = String::new();
    let mut events = EventHandler::new();
    // Create a dummy App instance for event handling during password input
    let mut dummy_app = App::new(Vec::new(), [0; 32], Vec::new());
    loop {
        terminal.draw(|f| {
            let size = f.area();
            // Clear the entire screen first
            f.render_widget(ratatui::widgets::Clear, f.area());

            f.render_widget(
                ratatui::widgets::Paragraph::new(format!(
                    "Enter master password: {}",
                    "*".repeat(password.len())
                ))
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL)),
                size,
            );
        })?;
        if let Some(event) = events.next_event(&mut dummy_app) {
            match event {
                KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                } => password.push(c),
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    password.pop();
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    break;
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "User cancelled"));
                }
                _ => {}
            }
        }
    }
    Ok(password)
}

pub fn run_tui() -> io::Result<()> {
    // Set up the terminal
    let mut terminal = setup_terminal()?;

    // Check if already unlocked
    let start_unlocked = STATE_MANAGER.is_unlocked();

    let (passwords, key, salt) = if start_unlocked {
        let state = STATE_MANAGER.get_state()?;
        (state.passwords, state.encryption_key, state.salt)
    } else {
        // Show loading message
        terminal.draw(|f| {
            f.render_widget(
                ratatui::widgets::Paragraph::new("Loading password manager..."),
                f.area(),
            );
        })?;

        // Implement unlock flow
        let result = (|| -> io::Result<(Vec<PasswordEntry>, [u8; 32], Vec<u8>)> {
            if std::path::Path::new(PASSWORD_FILE_PATH).exists() {
                let mut file = File::open(PASSWORD_FILE_PATH)?;
                let mut first_char = [0u8; 1];
                file.read_exact(&mut first_char)?;
                file.seek(SeekFrom::Start(0))?;

                if first_char[0] == b'[' {
                    let passwords: Vec<PasswordEntry> = serde_json::from_reader(&file)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                    terminal.draw(|f| {
                        let size = f.area();
                        f.render_widget(
                            ratatui::widgets::Paragraph::new(
                                "Existing unencrypted password file found. Please set a master password to encrypt it."
                            )
                            .block(
                                ratatui::widgets::Block::default()
                                    .borders(ratatui::widgets::Borders::ALL),
                            ),
                            size,
                        );
                    })?;

                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let password = get_master_password(&mut terminal)?;
                    let mut salt = vec![0u8; 16];
                    rand::rng().fill_bytes(&mut salt);
                    let key: [u8; 32] = derive_key(&password, &salt)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    save_passwords(PASSWORD_FILE_PATH, &passwords, &key, &salt)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    return Ok((passwords, key, salt));
                } else {
                    loop {
                        let password = get_master_password(&mut terminal)?;
                        match load_passwords(PASSWORD_FILE_PATH, &password) {
                            Ok((passwords, key, salt)) => {
                                return Ok((passwords, key, salt));
                            }
                            Err(e) => {
                                terminal.draw(|f| {
                                    let size = f.area();
                                    f.render_widget(
                                        ratatui::widgets::Paragraph::new(format!(
                                            "Error: Invalid password - {}",
                                            e
                                        ))
                                        .block(
                                            ratatui::widgets::Block::default()
                                                .borders(ratatui::widgets::Borders::ALL),
                                        ),
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
                        ratatui::widgets::Paragraph::new(
                            "No password file found. Setting up new file...",
                        )
                        .block(
                            ratatui::widgets::Block::default()
                                .borders(ratatui::widgets::Borders::ALL),
                        ),
                        size,
                    );
                })?;
                std::thread::sleep(std::time::Duration::from_secs(1));
                let password = get_master_password(&mut terminal)?;
                create_password_file(PASSWORD_FILE_PATH, &password).map_err(|e| {
                    error!("Failed to create password file: {}", e);
                    io::Error::new(io::ErrorKind::Other, e)
                })
            }
        })();

        match result {
            Ok(data) => data,
            Err(e) => {
                restore_terminal()?;
                return Err(e);
            }
        }
    };

    // Run the TUI with the retrieved passwords
    let mut app = App::new(passwords, key, salt);
    let mut events = EventHandler::new();

    // Clear the terminal completely before starting the main app loop
    terminal.clear()?;

    while app.running {
        terminal.draw(|f| render_ui(f, &app))?;
        if let Some(_key) = events.next_event(&mut app) {}
    }

    restore_terminal()?;
    Ok(())
}
