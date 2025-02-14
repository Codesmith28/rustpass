use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem},
    Frame,
};

pub fn render_password_list(f: &mut Frame, app: &App, area: Rect) {
    // Clear the area before re-rendering the list
    f.render_widget(Clear, area);

    let items: Vec<ListItem> = if app.filtered_passwords.is_empty() {
        vec![ListItem::new("  No passwords loaded")]
    } else {
        app.filtered_passwords
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let is_selected = i == app.selected_index;
                let is_multi_selected = app.multi_selected.contains(&i);

                let prefix = match (is_selected, is_multi_selected) {
                    (true, true) => "> o ",   // Both cursor and selected
                    (true, false) => ">   ",  // Just cursor
                    (false, true) => "  o ",  // Just selected
                    (false, false) => "    ", // Neither
                };

                let line = Line::from(vec![
                    Span::styled(
                        prefix,
                        Style::default().fg(if is_multi_selected {
                            Color::Yellow
                        } else if is_selected {
                            Color::Cyan // Changed to make ">" cyan when selected
                        } else {
                            Color::White
                        }),
                    ),
                    Span::styled(
                        format!("{} | {}", entry.name, entry.id),
                        Style::default().fg(if is_selected {
                            Color::Cyan
                        } else {
                            Color::White
                        }),
                    ),
                ]);

                ListItem::new(line)
            })
            .collect()
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Passwords "),
    );
    f.render_widget(list, area);
}
