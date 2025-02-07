use crate::tui::app::App;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next_event(&mut self, app: &mut App) -> Option<KeyEvent> {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Esc => app.quit(),
                    KeyCode::Up => app.move_selection_up(),
                    KeyCode::Down => app.move_selection_down(),
                    KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.toggle_help();
                    }
                    KeyCode::Char(c) => app.update_search(c),
                    KeyCode::Backspace => {
                        app.search_input.pop(); // Remove last character
                        app.update_search('\0'); // Re-filter list
                    }
                    _ => {}
                }
                return Some(key);
            }
        }
        None
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
