use crate::tui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn render_ui(f: &mut Frame, app: &App) {
    // Add minimal padding to the entire UI
    let padded_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area())[0];

    // Split the screen horizontally: left pane for search & passwords, right pane for preview
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // left pane (search box + passwords list)
            Constraint::Percentage(50), // preview pane (full height)
        ])
        .split(padded_area);

    // In the left pane, split vertically for search box and passwords list
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // search box at the top
            Constraint::Min(0),    // remaining area for passwords
        ])
        .split(main_chunks[0]);

    // Render left pane widgets
    render_search_box(f, app, left_chunks[0]);
    render_password_list(f, app, left_chunks[1]);

    // Render preview pane using the entire height of the right pane
    render_preview(f, app, main_chunks[1]);

    // Overlay help panel if enabled (centers itself over the whole area)
    if app.show_help {
        let help_area = centered_rect(60, 60, f.area());
        f.render_widget(Clear, help_area); // Clear the background
        render_help_panel(f, help_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_search_box(f: &mut Frame, app: &App, area: Rect) {
    let search = Paragraph::new(app.search_input.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Search ")
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(search, area);
}

fn render_password_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = if app.filtered_passwords.is_empty() {
        vec![ListItem::new("No passwords loaded")]
    } else {
        app.filtered_passwords
            .iter()
            .map(|entry| ListItem::new(format!("{} | {}", entry.name, entry.id)))
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

fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    let details = if let Some(selected) = app.selected_password() {
        format!(
            "Name: {}\nID: {}\nPassword: ****\nURL: {}\nNotes: {}",
            selected.name,
            selected.id,
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

fn render_help_panel(f: &mut Frame, area: Rect) {
    let help_text = "\
        Keybindings:
        ────────────
        ↑/↓        Navigate list
        Ctrl+h     Toggle help
        q/Esc      Quit
    ";

    let help = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Help ")
            .border_style(Style::default().fg(Color::Yellow)),
    );

    f.render_widget(help, area);
}
