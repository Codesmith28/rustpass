use crate::{models::data::PasswordEntry, tui::app::App};
use ::std::time::{Duration, Instant};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render a notification popup in the bottom-right that lasts 5 seconds.
/// After 4 seconds, the popup starts sliding to the right.
pub fn render_notification(f: &mut Frame, notif: &Notification) {
    let now = Instant::now();
    let elapsed = now.duration_since(notif.created);

    // Only render while within 5 seconds.
    if elapsed > Duration::from_secs(5) {
        return;
    }

    // Base area for notification (e.g. 30% wide, 15% high)
    let base_area = bottom_right_rect(30, 15, f.area());

    // Compute slide offset: after 4 seconds, slide out over the final second.
    let slide_offset = if elapsed > Duration::from_secs(4) {
        let extra = elapsed - Duration::from_secs(4);
        // Calculate offset proportionally to the extra time.
        ((base_area.width as f32) * (extra.as_secs_f32() / 1.0)).min(base_area.width as f32) as u16
    } else {
        0
    };

    // Adjust the notification area to slide horizontally.
    let notif_area = Rect {
        x: base_area.x + slide_offset,
        y: base_area.y,
        width: base_area.width.saturating_sub(slide_offset),
        height: base_area.height,
    };

    let text = format!("{}\n\n{}", notif.header, notif.message);
    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Notification")
            .border_style(Style::default().fg(notif.color)),
    );

    f.render_widget(widget, notif_area);
}

// A simple Notification struct.
pub struct Notification {
    pub header: String,
    pub message: String,
    pub color: Color,
    pub created: Instant,
}

fn bottom_right_rect(percent_width: u16, percent_height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_width / 100;
    let popup_height = r.height * percent_height / 100;
    let x = r.x + r.width - popup_width;
    let y = r.y + r.height - popup_height;
    Rect {
        x,
        y,
        width: popup_width,
        height: popup_height,
    }
}

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
    render_preview(f, app, main_chunks[1]);

    // Overlay help panel if enabled (centers itself over the whole area)
    if app.show_help {
        let help_area = centered_rect(60, 60, f.area());
        f.render_widget(Clear, help_area); // Clear the background
        render_help_panel(f, help_area);
    }

    if let Some(modal) = &app.modal {
        render_modal(f, modal, f.area());
    }

    if let Some(notif) = &app.notification {
        render_notification(f, notif);
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

fn render_preview(f: &mut Frame, app: &App, area: Rect) {
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

fn render_help_panel(f: &mut Frame, area: Rect) {
    let help_text = "\
Keybindings:
────────────────
↑/↓         Navigate list
Ctrl+h      Toggle help
q/Esc       Quit
Alt+c       Copy password
Alt+e       Edit entry
Alt+d       Delete entry (or delete multi-selected)
Alt+n       Create new entry
Tab         Multi-select current & move to next
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
