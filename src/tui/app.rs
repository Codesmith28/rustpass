use crate::models::data::{Metadata, PasswordEntry};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::Color;
use std::time::Instant;

// Import Notification from widgets (adjust the module path as needed)
use crate::tui::layout::restore_terminal;
use crate::tui::widgets::{modal::Modal, modal::ModalType, notification::Notification};

use super::data::save_passwords;
use super::widgets::modal::{ConfirmationType, InputType};

use arboard::Clipboard;

pub fn fuzzy_match(query: &str, target: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let mut query_chars = query.chars();
    let mut current = query_chars.next().unwrap();
    for c in target.chars() {
        if c == current {
            if let Some(next) = query_chars.next() {
                current = next;
            } else {
                return true;
            }
        }
    }
    false
}

pub struct App {
    pub running: bool,
    pub search_input: String,
    pub all_passwords: Vec<PasswordEntry>,
    pub filtered_passwords: Vec<PasswordEntry>,
    pub selected_index: usize,
    pub show_help: bool,
    pub multi_selected: Vec<usize>, // holds indices of multi-selected entries
    pub notification: Option<Notification>, // new field for notifications
    pub modal: Option<Modal>,
}

impl App {
    pub fn new(passwords: Vec<PasswordEntry>) -> Self {
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
        }
    }

    pub fn update_search(&mut self, c: char) {
        self.search_input.push(c);
        self.filter_passwords();
    }

    pub fn filter_passwords(&mut self) {
        let search = self.search_input.to_lowercase();
        self.filtered_passwords = self
            .all_passwords
            .iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                let id = p.id.to_lowercase();
                fuzzy_match(&search, &name) || fuzzy_match(&search, &id)
            })
            .cloned()
            .collect();
        self.selected_index = 0;
        self.multi_selected.clear();
    }

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

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn selected_password(&self) -> Option<&PasswordEntry> {
        if self.filtered_passwords.is_empty() {
            None
        } else {
            Some(&self.filtered_passwords[self.selected_index])
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
        let _ = restore_terminal();
    }

    // Copies the password of the current selection to clipboard.
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

            if let Err(e) = clipboard.set_text(entry.password.clone()) {
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

    // Toggles multi-selection for the current entry and moves to the next one.
    pub fn toggle_multi_select(&mut self) {
        if self.filtered_passwords.get(self.selected_index).is_some() {
            if !self.multi_selected.contains(&self.selected_index) {
                self.multi_selected.push(self.selected_index);
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

    pub fn confirm_modal(&mut self) {
        if let Some(modal) = self.modal.take() {
            match modal.typ {
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
                        if let Err(e) = save_passwords("./passwords.json", &self.all_passwords) {
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
                ModalType::Confirm(ConfirmationType::BulkDelete) => {
                    // Remove entries in multi_selected (assuming indices are sorted in ascending order)
                    self.multi_selected.sort();
                    self.multi_selected.reverse();
                    for idx in &self.multi_selected {
                        if let Some(entry) = self.filtered_passwords.get(*idx) {
                            self.all_passwords.retain(|p| p.id != entry.id);
                        }
                    }
                    self.filter_passwords();
                    self.notification = Some(Notification {
                        header: "Deleted".into(),
                        message: "Selected entries deleted!".into(),
                        color: Color::Red,
                        created: Instant::now(),
                    });
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
                                if let Some(existing_entry) =
                                    self.all_passwords.iter_mut().find(|p| p.id == entry.id)
                                {
                                    existing_entry.name = name.clone();
                                    existing_entry.id = id;
                                    existing_entry.password = password;
                                    existing_entry.metadata.url = url;
                                    existing_entry.metadata.notes = notes;
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
                    if let Err(e) = save_passwords("./passwords.json", &self.all_passwords) {
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
