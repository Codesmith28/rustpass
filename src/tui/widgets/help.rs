use ratatui::{
    layout::Rect,
    style::{ Color, Style },
    widgets::{ Block, BorderType, Borders, Paragraph, Clear },
    Frame,
};

pub fn render_help_panel(f: &mut Frame, area: Rect) {
    // Clear the area before rendering the help panel
    f.render_widget(Clear, area);

    let help_text =
        "\
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
            .border_style(Style::default().fg(Color::Yellow))
    );

    f.render_widget(help, area);
}
