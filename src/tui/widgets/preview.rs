use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    let details = if let Some(selected) = app.selected_password() {
        format!(
            "Name: {}\nID: {}\nPassword: {}\nURL: {}\nNotes: {}",
            selected.name,
            selected.id,
            selected.password,
            selected.metadata.url.as_deref().unwrap_or("N/A"),
            selected.metadata.notes.as_deref().unwrap_or("None")
        )
    } else {
        "No password selected".to_string()
    };

    let preview = Paragraph::new(details).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Preview "),
    );

    f.render_widget(preview, area);
}
