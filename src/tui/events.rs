use crate::tui::app::App;
use crate::tui::keybindings::{KeyAction, KeyBindings};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::time::Duration;

use super::app::fuzzy_match;
use super::widgets::{Modal, ModalType};

pub struct EventHandler {
    bindings: KeyBindings,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            bindings: KeyBindings::new(),
        }
    }

    pub fn next_event(&mut self, app: &mut App) -> Option<KeyEvent> {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                // Handle modal-specific keys first
                if app.modal.is_some() {
                    match key.code {
                        KeyCode::Enter => {
                            app.confirm_modal();
                            return Some(key);
                        }
                        KeyCode::Esc => {
                            app.close_modal();
                            return Some(key);
                        }
                        _ => return Some(key),
                    }
                }

                if let Some(action) = self.bindings.match_action(key) {
                    self.handle_action(action, app);
                }
                return Some(key);
            }
        }
        None
    }

    fn handle_action(&self, action: KeyAction, app: &mut App) {
        match action {
            KeyAction::Quit => app.quit(),
            KeyAction::MoveUp => app.move_selection_up(),
            KeyAction::MoveDown => app.move_selection_down(),
            KeyAction::ToggleHelp => app.toggle_help(),
            KeyAction::SearchChar(c) => {
                app.update_search(c);
            }
            KeyAction::Backspace => {
                app.search_input.pop();
                let search = app.search_input.to_lowercase();
                app.filtered_passwords = app
                    .all_passwords
                    .iter()
                    .filter(|p| {
                        let name = p.name.to_lowercase();
                        let id = p.id.to_lowercase();
                        fuzzy_match(&search, &name) || fuzzy_match(&search, &id)
                    })
                    .cloned()
                    .collect();
                app.selected_index = 0;
            }
            KeyAction::CopyPassword => {
                app.copy_password();
            }
            KeyAction::EditEntry => {
                if let Some(entry) = app.selected_password() {
                    app.open_modal(Modal::new(
                        ModalType::Edit,
                        "Edit Entry".into(),
                        format!("Edit entry: {}", entry.name),
                        Some(entry.clone()),
                    ));
                }
            }
            KeyAction::DeleteEntry => {
                if app.multi_selected.is_empty() {
                    if let Some(entry) = app.selected_password() {
                        app.open_modal(Modal::new(
                            ModalType::Delete,
                            "Confirm Delete".into(),
                            format!("Are you sure you want to delete {}?", entry.name),
                            Some(entry.clone()),
                        ));
                    }
                } else {
                    app.delete_selected_entries();
                }
            }
            KeyAction::CreateEntry => {
                app.open_modal(Modal::new(
                    ModalType::Create,
                    "Create Entry".into(),
                    "Create a new password entry".into(),
                    None,
                ));
            }
            KeyAction::MultiSelect => {
                app.toggle_multi_select();
            }
            KeyAction::CloseModal => {
                if app.modal.is_some() {
                    app.close_modal();
                } else {
                    app.quit();
                }
            }
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
