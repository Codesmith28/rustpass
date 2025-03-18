use crate::data::data::save_passwords;
use crate::models::data::{ Metadata, PasswordEntry };
use crate::utils::fuzzy_finder::fuzzy_match;
use crate::utils::verify_passwords::verify_password;
use crate::PASSWORD_FILE_PATH;
// Import Notification from widgets (adjust the module path as needed)
use crate::tui::layout::restore_terminal;
use crate::tui::widgets::{ modal::Modal, modal::ModalType, notification::Notification };

use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use log::debug;
use ratatui::style::Color;
use std::time::Instant;
use arboard::Clipboard;

use super::widgets::modal::{ ConfirmationType, InputType };


// struct for the app:
pub struct App {
    pub running: bool,
    pub search_input: String,
    pub all_passwords: Vec<PasswordEntry>,
    pub filtered_passwords: Vec<PasswordEntry>,
    pub selected_index: usize,
    pub show_help: bool,
    pub multi_selected: Vec<String>,
    pub notification: Option<Notification>,
    pub modal: Option<Modal>,
    pub encryption_key: [u8; 32], // Add this
    pub salt: Vec<u8>, // Add this
}

// implementation for the app:
impl App {
    pub fn new(passwords: Vec<PasswordEntry>, encryption_key: [u8; 32], salt: Vec<u8>) -> Self {
        Self {
            running: true,
            search_input: String::new(),
            all_passwords: passwords.clone(),
            filtered_passwords: passwords,
            selected_index: 0,
            show_help: false,
            multi_selected: Vec::new(),
            notification: None,
            modal: None,
            encryption_key,
            salt,
        }
    }

    // update the search input:
    pub fn update_search(&mut self, c: char) {
        self.search_input.push(c);
        self.filter_passwords();
    }

    // filter the passwords:
    pub fn filter_passwords(&mut self) {
        let search = self.search_input.to_lowercase();
        self.filtered_passwords = self.all_passwords
            .iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                let id = p.id.to_lowercase();
                fuzzy_match(&search, &name) || fuzzy_match(&search, &id)
            })
            .cloned()
            .collect();
        self.selected_index = 0;
    }

    // move the selection up:
    pub fn move_selection_up(&mut self) {
        let len = self.filtered_passwords.len();
        if len > 0 {
            self.selected_index = if self.selected_index == 0 {
                len - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    // move the selection down:
    pub fn move_selection_down(&mut self) {
        let len = self.filtered_passwords.len();
        if len > 0 {
            self.selected_index = if self.selected_index >= len - 1 {
                0
            } else {
                self.selected_index + 1
            };
        }
    }

    // toggle the help:
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    // get the selected password:
    pub fn selected_password(&self) -> Option<&PasswordEntry> {
        if self.filtered_passwords.is_empty() {
            None
        } else {
            Some(&self.filtered_passwords[self.selected_index])
        }
    }

    // quit the app:
    pub fn quit(&mut self) {
        self.running = false;
        let _ = restore_terminal();
    }

    // Copies the password of the current selection to clipboard:
    pub fn copy_password(&mut self) {
        if let Some(entry) = self.selected_password() {
            let mut clipboard = match Clipboard::new() {
                Ok(ctx) => ctx,
                Err(e) => {
                    self.notification = Some(Notification {
                        header: "Error".into(),
                        message: format!("Could not access clipboard: {}", e),
                        color: Color::Red,
                        created: Instant::now(),
                    });
                    return;
                }
            };
            let selected_password = entry.password.clone();
            debug!("Selected password: {}", selected_password);
            if let Err(e) = clipboard.set_text(selected_password) {
                self.notification = Some(Notification {
                    header: "Error".into(),
                    message: format!("Failed to copy to clipboard: {}", e),
                    color: Color::Red,
                    created: Instant::now(),
                });
                return;
            }

            self.notification = Some(Notification {
                header: "Copied".into(),
                message: format!("{} password copied!", entry.name),
                color: Color::Green,
                created: Instant::now(),
            });
        }
    }

    // Toggles multi-selection for the current entry and moves to the next one:
    pub fn toggle_multi_select(&mut self) {
        if let Some(entry) = self.filtered_passwords.get(self.selected_index) {
            let entry_id = entry.id.clone();
            if let Some(pos) = self.multi_selected.iter().position(|x| x == &entry_id) {
                // If already selected, unselect it
                self.multi_selected.remove(pos);
            } else {
                // If not selected, select it
                self.multi_selected.push(entry_id);
            }
            self.move_selection_down();
        }
    }

    pub fn open_modal(&mut self, modal: Modal) {
        self.modal = Some(modal);
    }

    pub fn close_modal(&mut self) {
        self.modal = None;
    }

    pub fn handle_modal_input(&mut self, key: KeyEvent) {
        if let Some(modal) = &mut self.modal {
            match key.code {
                KeyCode::Tab => {
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        modal.prev_field();
                    } else {
                        modal.next_field();
                    }
                }
                KeyCode::Char(c) => modal.handle_input(c),
                KeyCode::Backspace => modal.handle_backspace(),
                _ => {}
            }
        }
    }

    // confirm the modal:
    pub fn confirm_modal(&mut self) {
        if let Some(modal) = self.modal.take() {
            match modal.typ {
                // handle the delete confirmation:
                ModalType::Confirm(ConfirmationType::Delete) => {
                    if let Some(entry) = modal.entry {
                        self.all_passwords.retain(|p| p.id != entry.id);
                        self.filter_passwords();
                        self.notification = Some(Notification {
                            header: "Deleted".into(),
                            message: format!("{} deleted!", entry.name),
                            color: Color::Red,
                            created: Instant::now(),
                        });
                        // Save after deletion
                        if
                            let Err(e) = save_passwords(
                                PASSWORD_FILE_PATH,
                                &self.all_passwords,
                                &self.encryption_key,
                                &self.salt
                            )
                        {
                            log::error!("Failed to save passwords: {}", e);
                            self.notification = Some(Notification {
                                header: "Error".into(),
                                message: "Failed to save changes".into(),
                                color: Color::Red,
                                created: Instant::now(),
                            });
                        }
                    }
                }

                // handle the bulk delete confirmation:
                ModalType::Confirm(ConfirmationType::BulkDelete) => {
                    // Save IDs before clearing the selection
                    let selected_ids = self.multi_selected.clone();

                    // Remove all selected entries
                    self.all_passwords.retain(|p| !selected_ids.contains(&p.id));

                    // Clear the selection and update display
                    self.multi_selected.clear();
                    self.filter_passwords();

                    // Show notification
                    self.notification = Some(Notification {
                        header: "Deleted".into(),
                        message: "Selected entries deleted!".into(),
                        color: Color::Red,
                        created: Instant::now(),
                    });

                    // Save changes
                    if
                        let Err(e) = save_passwords(
                            PASSWORD_FILE_PATH,
                            &self.all_passwords,
                            &self.encryption_key,
                            &self.salt
                        )
                    {
                        log::error!("Failed to save passwords: {}", e);
                        self.notification = Some(Notification {
                            header: "Error".into(),
                            message: "Failed to save changes".into(),
                            color: Color::Red,
                            created: Instant::now(),
                        });
                    }
                }

                ModalType::Input(input_type) => {
                    let name = modal.input_fields[0].value.clone();
                    let id = modal.input_fields[1].value.clone();
                    let password = modal.input_fields[2].value.clone();
                    let url = Some(modal.input_fields[3].value.clone());
                    let notes = Some(modal.input_fields[4].value.clone());

                    match input_type {
                        InputType::Create => {
                            let entry = PasswordEntry {
                                name: name.clone(),
                                id: id.clone(),
                                password,
                                metadata: Metadata { url, notes },
                            };

                            self.all_passwords.push(entry);
                            self.filter_passwords();
                            self.notification = Some(Notification {
                                header: "Created".into(),
                                message: format!("{} created!", name),
                                color: Color::Green,
                                created: Instant::now(),
                            });
                        }
                        InputType::Edit => {
                            if let Some(entry) = modal.entry {
                                if
                                    let Some(existing_entry) = self.all_passwords
                                        .iter_mut()
                                        .find(|p| p.id == entry.id)
                                {
                                    existing_entry.name = name.clone();
                                    existing_entry.id = id;
                                    existing_entry.password = password;
                                    existing_entry.metadata.url = url;
                                    existing_entry.metadata.notes = notes;
                                }

                                if !verify_password(&entry, self) {
                                    return;
                                }

                                self.filter_passwords();
                                self.notification = Some(Notification {
                                    header: "Updated".into(),
                                    message: format!("{} updated!", name),
                                    color: Color::Yellow,
                                    created: Instant::now(),
                                });
                            }
                        }
                    }

                    // Save after create or edit
                    if
                        let Err(e) = save_passwords(
                            PASSWORD_FILE_PATH,
                            &self.all_passwords,
                            &self.encryption_key,
                            &self.salt
                        )
                    {
                        log::error!("Failed to save passwords: {}", e);
                        self.notification = Some(Notification {
                            header: "Error".into(),
                            message: "Failed to save changes".into(),
                            color: Color::Red,
                            created: Instant::now(),
                        });
                    }
                }
            }
        }
        self.close_modal();
    }
}
