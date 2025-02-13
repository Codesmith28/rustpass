use crate::tui::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
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
                // If multi-selected, prefix with "o " and use green
                if app.multi_selected.contains(&i) {
                    let content = format!("o {} | {}", entry.name, entry.id);
                    return ListItem::new(content).style(Style::default().fg(Color::Green));
                }
                // Otherwise, use "> " for current hover, "  " for normal.
                let prefix = if i == app.selected_index { "> " } else { "  " };
                let content = format!("{}{} | {}", prefix, entry.name, entry.id);
                let item = ListItem::new(content);
                if i == app.selected_index {
                    item.style(Style::default().fg(Color::Yellow))
                } else {
                    item
                }
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
