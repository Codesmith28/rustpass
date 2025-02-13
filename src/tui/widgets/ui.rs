use super::{
    help::render_help_panel, layout::centered_rect, list::render_password_list,
    modal::render_modal, notification::render_notification, preview::render_preview,
    search::render_search_box,
};
use crate::tui::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Clear,
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
