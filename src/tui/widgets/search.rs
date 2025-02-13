use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_search_box(f: &mut Frame, app: &App, area: Rect) {
    let search = Paragraph::new(app.search_input.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Search ")
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(search, area);
}
