use super::layout::restore_terminal;
use crate::models::data::PasswordEntry;

pub struct App {
    pub running: bool,
    pub search_input: String,
    pub all_passwords: Vec<PasswordEntry>,
    pub filtered_passwords: Vec<PasswordEntry>,
    pub selected_index: usize,
    pub show_help: bool,
}

impl App {
    pub fn new(passwords: Vec<PasswordEntry>) -> Self {
        Self {
            running: true,
            search_input: String::new(),
            all_passwords: passwords.clone(),
            filtered_passwords: passwords,
            selected_index: 0,
            show_help: false,
        }
    }

    pub fn update_search(&mut self, input: char) {
        self.search_input.push(input);
        self.filtered_passwords = self
            .all_passwords
            .iter()
            .filter(|p| p.name.contains(&self.search_input) || p.id.contains(&self.search_input))
            .cloned()
            .collect();
        self.selected_index = 0; // Reset selection on search update
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.filtered_passwords.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn selected_password(&self) -> Option<&PasswordEntry> {
        if self.filtered_passwords.is_empty() {
            None
        } else {
            Some(&self.filtered_passwords[self.selected_index])
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
        let _ = restore_terminal();
    }
}
