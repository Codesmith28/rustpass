use std::io;

use rustpass::{
    cli::handler::{ handle_command, parse_args, Command },
    daemon::service::DaemonService,
    daemon::ipc,
    tui::{ self, run_tui },
    utils::logger::init_logger,
};

fn main() -> io::Result<()> {
    // Initialize logging
    init_logger();

    // Clean up any stale sockets
    let _ = ipc::cleanup_stale_socket();

    // Set up panic hook to restore terminal on panic
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(
        Box::new(move |panic_info| {
            let _ = tui::layout::restore_terminal();
            default_panic(panic_info);
        })
    );

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Special case for daemon mode (started by daemonize)
    if args.len() >= 2 && args[1] == "--daemon-direct-start" {
        // Initialize logging for daemon mode
        env_logger::Builder
            ::from_env(env_logger::Env::default())
            .filter_level(log::LevelFilter::Info)
            .format_timestamp_secs()
            .init();

        log::info!("Starting daemon in direct mode");
        let service = DaemonService::new();
        return service.start();
    }

    // Normal CLI operation
    let command = parse_args(args);

    // Handle TUI command separately
    if let Command::Tui = command {
        return run_tui();
    }

    // Handle all other commands
    handle_command(command)
}
