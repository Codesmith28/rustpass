use crate::tui::app::App;
use crate::tui::keybindings::{KeyAction, KeyBindings};
use crossterm::event::{self, KeyEvent};
use std::time::Duration;

use super::app::fuzzy_match;

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
                // Re-filter using case insensitive fuzzy matching:
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
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
