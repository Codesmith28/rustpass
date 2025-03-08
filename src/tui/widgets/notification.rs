use super::layout::bottom_right_rect;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};

pub struct Notification {
    pub header: String,
    pub message: String,
    pub color: Color,
    pub created: Instant,
}

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

    // Clear the area before rendering the notification
    f.render_widget(Clear, notif_area);

    let text = format!("{}\n\n{}", notif.header, notif.message);
    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Notification")
            .border_style(Style::default().fg(notif.color))
            .style(Style::default().bg(Color::Reset)),
    );

    f.render_widget(widget, notif_area);
}
