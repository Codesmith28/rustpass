use crossterm::event::KeyCode;

pub struct KeyBindings {
    pub quit: KeyCode,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            quit: KeyCode::Char('q'),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}
