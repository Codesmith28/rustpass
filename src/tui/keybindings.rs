use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum KeyAction {
    Quit,
    MoveUp,
    MoveDown,
    ToggleHelp,
    SearchChar(char),
    Backspace,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KeyBindings {
    pub quit: KeyEvent,
    pub move_up: KeyEvent,
    pub move_down: KeyEvent,
    pub toggle_help: KeyEvent,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self {
            quit: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            move_up: KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            move_down: KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            toggle_help: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
        }
    }

    pub fn match_action(&self, key: KeyEvent) -> Option<KeyAction> {
        // Always return Backspace if the key's code is Backspace.
        if key.code == KeyCode::Backspace {
            return Some(KeyAction::Backspace);
        }
        if key == self.quit {
            return Some(KeyAction::Quit);
        }
        if key == self.move_up {
            return Some(KeyAction::MoveUp);
        }
        if key == self.move_down {
            return Some(KeyAction::MoveDown);
        }
        // Only handle CTRL+h for help.
        if key == self.toggle_help {
            return Some(KeyAction::ToggleHelp);
        }
        // For character input, ignore CTRL combinations except CTRL+h.
        if let KeyEvent {
            code: KeyCode::Char(c),
            modifiers,
            ..
        } = key
        {
            if modifiers.contains(KeyModifiers::CONTROL) {
                // Only CTRL+h is valid for toggle help (handled above).
                None
            } else {
                Some(KeyAction::SearchChar(c))
            }
        } else {
            None
        }
    }
}
