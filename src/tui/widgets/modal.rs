use crate::models::data::{Metadata, PasswordEntry};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::layout::centered_rect;

#[derive(PartialEq)]
pub enum ModalType {
    Confirm(ConfirmationType),
    Input(InputType),
}

#[derive(PartialEq)]
pub enum ConfirmationType {
    Delete,
    BulkDelete,
}

#[derive(PartialEq)]
pub enum InputType {
    Edit,
    Create,
}

pub struct Modal {
    pub typ: ModalType,
    pub title: String,
    pub content: String,
    pub entry: Option<PasswordEntry>,
    // New fields for input handling
    pub input_fields: Vec<InputField>,
    pub active_field: usize,
}

pub struct InputField {
    pub label: String,
    pub value: String,
    pub is_password: bool,
}

impl Modal {
    pub fn new_confirmation(
        typ: ConfirmationType,
        title: String,
        content: String,
        entry: Option<PasswordEntry>,
    ) -> Self {
        Self {
            typ: ModalType::Confirm(typ),
            title,
            content,
            entry,
            input_fields: Vec::new(),
            active_field: 0,
        }
    }

    pub fn new_input(typ: InputType, title: String, entry: Option<PasswordEntry>) -> Self {
        let input_fields = match &entry {
            Some(e) => vec![
                InputField {
                    label: "Name".into(),
                    value: e.name.clone(),
                    is_password: false,
                },
                InputField {
                    label: "ID".into(),
                    value: e.id.clone(),
                    is_password: false,
                },
                InputField {
                    label: "Password".into(),
                    value: e.password.clone(),
                    is_password: true,
                },
                InputField {
                    label: "URL".into(),
                    value: e.metadata.url.clone().unwrap_or_default(),
                    is_password: false,
                },
                InputField {
                    label: "Notes".into(),
                    value: e.metadata.notes.clone().unwrap_or_default(),
                    is_password: false,
                },
            ],
            None => vec![
                InputField {
                    label: "Name".into(),
                    value: String::new(),
                    is_password: false,
                },
                InputField {
                    label: "ID".into(),
                    value: String::new(),
                    is_password: false,
                },
                InputField {
                    label: "Password".into(),
                    value: String::new(),
                    is_password: true,
                },
                InputField {
                    label: "URL".into(),
                    value: String::new(),
                    is_password: false,
                },
                InputField {
                    label: "Notes".into(),
                    value: String::new(),
                    is_password: false,
                },
            ],
        };

        Self {
            typ: ModalType::Input(typ),
            title,
            content: String::new(),
            entry,
            input_fields,
            active_field: 0,
        }
    }

    pub fn next_field(&mut self) {
        self.active_field = (self.active_field + 1) % self.input_fields.len();
    }

    pub fn prev_field(&mut self) {
        self.active_field = if self.active_field == 0 {
            self.input_fields.len() - 1
        } else {
            self.active_field - 1
        };
    }

    pub fn handle_input(&mut self, c: char) {
        if let Some(field) = self.input_fields.get_mut(self.active_field) {
            field.value.push(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        if let Some(field) = self.input_fields.get_mut(self.active_field) {
            field.value.pop();
        }
    }

    pub fn to_password_entry(&self) -> Option<PasswordEntry> {
        if self.input_fields.len() < 5 {
            return None;
        }

        Some(PasswordEntry {
            name: self.input_fields[0].value.clone(),
            id: self.input_fields[1].value.clone(),
            password: self.input_fields[2].value.clone(),
            metadata: Metadata {
                url: Some(self.input_fields[3].value.clone()),
                notes: Some(self.input_fields[4].value.clone()),
            },
        })
    }
}

pub fn render_modal(f: &mut Frame, modal: &Modal, area: Rect) {
    let modal_area = centered_rect(60, 40, area);
    f.render_widget(Clear, modal_area);

    match &modal.typ {
        ModalType::Confirm(_) => render_confirmation_modal(f, modal, modal_area),
        ModalType::Input(_) => render_input_modal(f, modal, modal_area),
    }
}

fn render_confirmation_modal(f: &mut Frame, modal: &Modal, area: Rect) {
    let content = format!(
        "{}\n\nPress Enter to confirm or Esc to cancel",
        modal.content
    );
    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(modal.title.clone())
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_input_modal(f: &mut Frame, modal: &Modal, area: Rect) {
    let items: Vec<ListItem> = modal
        .input_fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let is_active = i == modal.active_field;
            let value = if field.is_password && !is_active && !field.value.is_empty() {
                "•".repeat(field.value.len())
            } else {
                field.value.clone()
            };

            let line = Line::from(vec![
                Span::raw(format!("{}: ", field.label)),
                Span::styled(
                    value.clone(),
                    Style::default().fg(if is_active {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                ),
                Span::styled(
                    if is_active { "_" } else { "" },
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let help_text = "\nTab: Next field | Enter: Confirm | Esc: Cancel";
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(modal.title.clone())
            .border_style(Style::default().fg(match modal.typ {
                ModalType::Input(InputType::Edit) => Color::Yellow,
                ModalType::Input(InputType::Create) => Color::Green,
                _ => Color::White,
            }))
            .title(format!("{}{}", modal.title, help_text)),
    );

    f.render_widget(list, area);
}
