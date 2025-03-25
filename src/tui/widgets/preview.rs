use crate::tui::app::App;
use ratatui::{ layout::Rect, widgets::{ Block, BorderType, Borders, Paragraph, Clear }, Frame };

pub fn render_preview(f: &mut Frame, app: &App, area: Rect) {
    // Clear the area before rendering the preview
    f.render_widget(Clear, area);

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
        Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Preview ")
    );

    // Create a smaller area for the hint text below the preview
    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height - 3,
        width: area.width,
        height: 3,
    };

    // Adjust preview area to make room for hint
    let preview_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height - 3,
    };

    // Render both widgets
    f.render_widget(preview, preview_area);
    f.render_widget(
        Paragraph::new("Press Alt + H for help").block(
            Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" Help ")
        ),
        hint_area
    );
}
