use crossterm::event::{self, KeyCode, KeyEvent};
use std::time::Duration;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next_event(&mut self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                return Some(code);
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
