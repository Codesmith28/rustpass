use crate::models::data::PasswordEntry;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use super::layout::centered_rect;

pub enum ModalType {
    Edit,
    Create,
    Delete,
}

pub struct Modal {
    pub typ: ModalType,
    pub title: String,
    pub content: String,
    pub entry: Option<PasswordEntry>,
}

impl Modal {
    pub fn new(
        typ: ModalType,
        title: String,
        content: String,
        entry: Option<PasswordEntry>,
    ) -> Self {
        Self {
            typ,
            title,
            content,
            entry,
        }
    }
}

pub fn render_modal(f: &mut Frame, modal: &Modal, area: Rect) {
    let modal_area = centered_rect(60, 40, area);
    f.render_widget(Clear, modal_area);

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
                .border_style(Style::default().fg(match modal.typ {
                    ModalType::Edit => Color::Yellow,
                    ModalType::Create => Color::Green,
                    ModalType::Delete => Color::Red,
                })),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, modal_area);
}
