use crate::tui::app::App;
use crate::tui::keybindings::{AppEvent, KeyBindings};
use crate::utils::fuzzy_finder::fuzzy_match;

use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::style::Color;
use std::time::{Duration, Instant};

use super::widgets::notification::Notification;
use super::widgets::{modal::ConfirmationType, modal::InputType, modal::Modal};

// struct for the event handler:
pub struct EventHandler {
    bindings: KeyBindings,
}

// implementation for the event handler:
impl EventHandler {
    pub fn new() -> Self {
        Self {
            bindings: KeyBindings::new(),
        }
    }

    // get the next event:
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

    // handle the action:
    fn handle_action(&self, action: AppEvent, app: &mut App) {
        match action {
            AppEvent::Quit => app.quit(),
            AppEvent::MoveUp => app.move_selection_up(),
            AppEvent::MoveDown => app.move_selection_down(),
            AppEvent::ToggleHelp => app.toggle_help(),
            AppEvent::SearchChar(c) => {
                app.update_search(c);
            }

            // handle the backspace event:
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

            // handle the copy password event:
            AppEvent::CopyPassword => {
                app.copy_password();
            }

            // handle the edit entry event:
            AppEvent::EditEntry => {
                if !app.multi_selected.is_empty() {
                    app.notification = Some(Notification {
                        header: "Error".into(),
                        message: "Please select one entry only".into(),
                        color: Color::Red,
                        created: Instant::now(),
                    });
                } else if let Some(entry) = app.selected_password() {
                    app.open_modal(Modal::new_input(
                        InputType::Edit,
                        " Edit Entry ".into(),
                        Some(entry.clone()),
                    ));
                }
            }

            // handle the delete entry event:
            AppEvent::DeleteEntry | AppEvent::BulkDelete => {
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
                    let selected_entries: Vec<String> = app
                        .filtered_passwords
                        .iter()
                        .filter(|entry| app.multi_selected.contains(&entry.id))
                        .map(|entry| format!("{} | {}", entry.name, entry.id))
                        .collect();
                    let entries_list = selected_entries.join("\n- ");
                    let message = format!(
                        "Are you sure you want to delete these entries?\n\n- {}",
                        entries_list
                    );
                    app.open_modal(Modal::new_confirmation(
                        ConfirmationType::BulkDelete,
                        " Confirm Bulk Delete ".into(),
                        message,
                        None,
                    ));
                }
            }

            // handle the create entry event:
            AppEvent::CreateEntry => {
                app.open_modal(Modal::new_input(
                    InputType::Create,
                    " Create Entry ".into(),
                    None,
                ));
            }

            // handle the multi select event:
            AppEvent::MultiSelect => {
                app.toggle_multi_select();
            }

            // handle the close modal event:
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

// default implementation for the event handler:
impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
