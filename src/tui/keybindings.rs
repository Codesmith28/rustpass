use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// enum for the events that can be triggered by the user:
#[derive(Debug)]
pub enum AppEvent {
    Quit,             // Esc: quit the app
    MoveUp,           // Up: move up
    MoveDown,         // Down: move down
    ToggleHelp,       // Alt+h
    SearchChar(char), // character input
    Backspace,        // Backspace: always handled
    CopyPassword,     // Alt+c
    EditEntry,        // Alt+e
    DeleteEntry,      // Alt+d
    BulkDelete,       // Alt+b
    CreateEntry,      // Alt+n
    MultiSelect,      // Tab: mark current & move to next
    CloseModal,       // Esc: close modal
}

// default implementation for the key bindings:
impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}

// struct for the key bindings:
pub struct KeyBindings {
    pub quit: KeyEvent,
    pub move_up: KeyEvent,
    pub move_down: KeyEvent,
    pub toggle_help: KeyEvent,
}

// implementation for the key bindings:
impl KeyBindings {
    pub fn new() -> Self {
        Self {
            quit: KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            move_up: KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            move_down: KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            toggle_help: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::ALT),
        }
    }

    // match the action for the key bindings:   
    pub fn match_action(&self, key: KeyEvent) -> Option<AppEvent> {
        // Handle Escape key specially
        if key.code == KeyCode::Esc {
            return Some(AppEvent::CloseModal);
        }

        // Backspace is always handled.
        if key.code == KeyCode::Backspace {
            return Some(AppEvent::Backspace);
        }

        // Handle Tab for multi-selection only when no modal is active
        if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::SHIFT) {
            return Some(AppEvent::MultiSelect);
        }

        // Handle Alt shortcuts:
        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Char('c') => return Some(AppEvent::CopyPassword),
                KeyCode::Char('e') => return Some(AppEvent::EditEntry),
                KeyCode::Char('d') => return Some(AppEvent::DeleteEntry),
                KeyCode::Char('n') => return Some(AppEvent::CreateEntry),
                _ => {}
            }
        }

        // handle the quit event:
        if key == self.quit {
            return Some(AppEvent::Quit);
        }

        // handle the move up event:
        if key == self.move_up {
            return Some(AppEvent::MoveUp);
        }

        // handle the move down event:
        if key == self.move_down {
            return Some(AppEvent::MoveDown);
        }

        // handle the toggle help event:
        if key == self.toggle_help {
            return Some(AppEvent::ToggleHelp);
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
                Some(AppEvent::SearchChar(c))
            }
        } else {
            None
        }
    }
}
