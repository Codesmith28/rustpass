use crate::tui::app::App;
use crate::tui::keybindings::{KeyAction, KeyBindings};
use crossterm::event::{self, KeyEvent};
use std::time::Duration;

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
                // Re-filter after deletion:
                app.filtered_passwords = app
                    .all_passwords
                    .iter()
                    .filter(|p| {
                        p.name.contains(&app.search_input) || p.id.contains(&app.search_input)
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
