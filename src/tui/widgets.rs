use crate::tui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_ui(f: &mut Frame, _app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let block = Block::default().title("Rust TUI").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
}
