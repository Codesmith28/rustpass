use crate::tui::app::App;
use crate::tui::keybindings::{AppEvent, KeyBindings};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::time::Duration;

use super::app::fuzzy_match;
use super::widgets::{modal::ConfirmationType, modal::InputType, modal::Modal};

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
                        _ => {
                            app.handle_modal_input(key);
                            return Some(key);
                        }
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

    fn handle_action(&self, action: AppEvent, app: &mut App) {
        match action {
            AppEvent::Quit => app.quit(),
            AppEvent::MoveUp => app.move_selection_up(),
            AppEvent::MoveDown => app.move_selection_down(),
            AppEvent::ToggleHelp => app.toggle_help(),
            AppEvent::SearchChar(c) => {
                app.update_search(c);
            }
            AppEvent::Backspace => {
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
            AppEvent::CopyPassword => {
                app.copy_password();
            }
            AppEvent::EditEntry => {
                if let Some(entry) = app.selected_password() {
                    app.open_modal(Modal::new_input(
                        InputType::Edit,
                        " Edit Entry ".into(),
                        Some(entry.clone()),
                    ));
                }
            }
            AppEvent::DeleteEntry => {
                if app.multi_selected.is_empty() {
                    if let Some(entry) = app.selected_password() {
                        app.open_modal(Modal::new_confirmation(
                            ConfirmationType::Delete,
                            " Confirm Delete ".into(),
                            format!("Are you sure you want to delete {}?", entry.name),
                            Some(entry.clone()),
                        ));
                    }
                } else {
                    app.delete_selected_entries();
                }
            }
            AppEvent::CreateEntry => {
                app.open_modal(Modal::new_input(
                    InputType::Create,
                    " Create Entry ".into(),
                    None,
                ));
            }
            AppEvent::MultiSelect => {
                app.toggle_multi_select();
            }
            AppEvent::CloseModal => {
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
