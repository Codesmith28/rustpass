use crate::{
    models::data::PasswordEntry,
    tui::{app::App, widgets::notification::Notification},
};
use ratatui::style::Color;
use std::time::Instant;

pub fn verify_password(entry: &PasswordEntry, app: &mut App) -> bool {
    if entry.password.len() > 21 || entry.password.len() < 8 {
        app.notification = Some(Notification {
            header: "Error".into(),
            message: "Password must be between 8 and 21 characters long".to_string(),
            color: Color::Red,
            created: Instant::now(),
        });
        return false;
    }
    if entry.name.is_empty() || entry.id.is_empty() {
        app.notification = Some(Notification {
            header: "Error".into(),
            message: "Name and ID cannot be empty".into(),
            color: Color::Red,
            created: Instant::now(),
        });
        return false;
    }
    true
}
