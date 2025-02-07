use super::layout::restore_terminal;
use crate::models::data::PasswordEntry;

pub fn fuzzy_match(query: &str, target: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let mut query_chars = query.chars();
    let mut current = query_chars.next().unwrap();
    for c in target.chars() {
        if c == current {
            if let Some(next) = query_chars.next() {
                current = next;
            } else {
                return true;
            }
        }
    }
    false
}

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

    pub fn update_search(&mut self, c: char) {
        self.search_input.push(c);
        self.filter_passwords();
    }

    pub fn filter_passwords(&mut self) {
        let search = self.search_input.to_lowercase();
        self.filtered_passwords = self
            .all_passwords
            .iter()
            .filter(|p| {
                let name = p.name.to_lowercase();
                let id = p.id.to_lowercase();
                fuzzy_match(&search, &name) || fuzzy_match(&search, &id)
            })
            .cloned()
            .collect();
        self.selected_index = 0;
    }

    pub fn move_selection_up(&mut self) {
        let len = self.filtered_passwords.len();
        if len > 0 {
            self.selected_index = if self.selected_index == 0 {
                len - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn move_selection_down(&mut self) {
        let len = self.filtered_passwords.len();
        if len > 0 {
            self.selected_index = if self.selected_index >= len - 1 {
                0
            } else {
                self.selected_index + 1
            };
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
