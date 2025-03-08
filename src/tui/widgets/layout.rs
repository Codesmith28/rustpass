    use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn bottom_right_rect(percent_width: u16, percent_height: u16, r: Rect) -> Rect {
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

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
