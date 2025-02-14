use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    MoveUp,
    MoveDown,
    ToggleHelp,
    SearchChar(char),
    Backspace,
    CopyPassword, // Alt+c
    EditEntry,    // Alt+e
    DeleteEntry,  // Alt+d
    BulkDelete,
    CreateEntry, // Alt+n
    MultiSelect, // Tab: mark current & move to next
    CloseModal,  // Esc: close modal
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
            toggle_help: KeyEvent::new(KeyCode::Char('h'), KeyModifiers::ALT),
        }
    }

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

        if key == self.quit {
            return Some(AppEvent::Quit);
        }
        if key == self.move_up {
            return Some(AppEvent::MoveUp);
        }
        if key == self.move_down {
            return Some(AppEvent::MoveDown);
        }
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
