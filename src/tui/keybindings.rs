use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum KeyAction {
    Quit,
    MoveUp,
    MoveDown,
    ToggleHelp,
    SearchChar(char),
    Backspace,
    CopyPassword, // Alt+c
    EditEntry,    // Alt+e
    DeleteEntry,  // Alt+d
    CreateEntry,  // Alt+n
    MultiSelect,  // Tab: mark current & move to next
    CloseModal,   // Esc: close modal
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
        // Handle Escape key specially
        if key.code == KeyCode::Esc {
            return Some(KeyAction::CloseModal);
        }

        // Backspace is always handled.
        if key.code == KeyCode::Backspace {
            return Some(KeyAction::Backspace);
        }

        // Handle Alt shortcuts:
        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('c') => return Some(KeyAction::CopyPassword),
                KeyCode::Char('e') => return Some(KeyAction::EditEntry),
                KeyCode::Char('d') => return Some(KeyAction::DeleteEntry),
                KeyCode::Char('n') => return Some(KeyAction::CreateEntry),
                _ => {}
            }
        }

        // Handle Tab for multi-selection.
        if key.code == KeyCode::Tab {
            return Some(KeyAction::MultiSelect);
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
        if key == self.toggle_help {
            return Some(KeyAction::ToggleHelp);
        }

        // For character input, ignore CTRL combinations (except ALT already handled).
        if let KeyEvent {
            code: KeyCode::Char(c),
            modifiers,
            ..
        } = key
        {
            if modifiers.contains(KeyModifiers::CONTROL) {
                None
            } else {
                Some(KeyAction::SearchChar(c))
            }
        } else {
            None
        }
    }
}
