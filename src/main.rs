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
    // to log the events for debugging:
    init_logger();

    // load the list of passwords from the file:
    let passwords = match load_passwords("./passwords.json") {
        Ok(passwords) => {
            //debug!("Successfully loaded password file! \n Number of passwords loaded: {}", passwords.len());
            passwords
        }
        Err(e) => {
            error!("Failed to load passwords: {}", e);
            // Return empty vec on error
            Vec::new()
        }
    };

    // Setup TUI
    let mut terminal = setup_terminal()?;

    // create the app:      
    let mut app = App::new(passwords);

    // create the event handler:
    let mut events = EventHandler::new();

    // run the app:
    while app.running {
        terminal.draw(|f| render_ui(f, &app))?;
        if let Some(_key) = events.next_event(&mut app) {}
    }

    // exit the app:
    Ok(())
}
