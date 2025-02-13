use rustpass::tui;
use rustpass::utils;

use log::error;

use tui::app::App;
use tui::events::EventHandler;

use tui::data::load_passwords;
use tui::layout::setup_terminal;
use tui::widgets::ui::render_ui;

use utils::logger::init_logger;

fn main() -> std::io::Result<()> {
    init_logger();

    let passwords = match load_passwords("./passwords.json") {
        Ok(passwords) => {
            // debug!("Successfully loaded password file");
            // debug!("Number of passwords loaded: {}", passwords.len());
            passwords
        }
        Err(e) => {
            error!("Failed to load passwords: {}", e);
            Vec::new() // Return empty vec on error
        }
    };

    // Setup TUI
    let mut terminal = setup_terminal()?;

    let mut app = App::new(passwords);
    let mut events = EventHandler::new();

    while app.running {
        terminal.draw(|f| render_ui(f, &app))?;

        if let Some(_key) = events.next_event(&mut app) {}
    }

    Ok(())
}
