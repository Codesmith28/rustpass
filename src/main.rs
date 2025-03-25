use std::io;

use rustpass::{
    cli::handler::{handle_command, parse_args, Command},
    tui::{self, run_tui},
    utils::logger::init_logger,
};

fn main() -> io::Result<()> {
    init_logger();

    // Set up panic hook to restore terminal on panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = tui::layout::restore_terminal();
        default_panic(panic_info);
    }));

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let command = parse_args(args);

    // Handle TUI command separately
    if let Command::Tui = command {
        return run_tui();
    }

    // Handle all other commands
    handle_command(command)
}
