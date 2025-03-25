use std::io;
use std::fs;
use std::path::PathBuf;

use crate::cli::commands::{
    execute_add,
    execute_help,
    execute_list,
    execute_lock,
    execute_remove,
    execute_unlock,
};
use crate::daemon::{ service::DaemonService, client::DaemonClient };

pub enum Command {
    Add {
        name: String,
        password: String,
    },
    List,
    Remove {
        name: String,
    },
    Unlock {
        password: Option<String>,
    },
    Lock,
    Help,
    Tui,
    StartDaemon,
    StopDaemon,
    DaemonStatus,
    Invalid,
}

pub fn parse_args(args: Vec<String>) -> Command {
    if args.len() < 2 {
        return Command::Invalid;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 5 {
                println!("Not enough arguments for add command");
                println!("Usage: rsp add <name> <username> <password>");
                return Command::Invalid;
            }
            Command::Add {
                name: args[2].clone(),
                password: args[4].clone(),
            }
        }
        "list" => Command::List,
        "remove" => {
            if args.len() < 3 {
                println!("Not enough arguments for remove command");
                println!("Usage: rsp remove <name>");
                return Command::Invalid;
            }
            Command::Remove { name: args[2].clone() }
        }
        "unlock" => {
            let password = if args.len() > 2 { Some(args[2].clone()) } else { None };
            Command::Unlock { password }
        }
        "lock" => Command::Lock,
        "help" => Command::Help,
        "tui" => Command::Tui,
        "daemon" => {
            if args.len() < 3 {
                println!("Not enough arguments for daemon command");
                println!("Usage: rsp daemon [start|stop|status]");
                return Command::Invalid;
            }
            match args[2].as_str() {
                "start" => Command::StartDaemon,
                "stop" => Command::StopDaemon,
                "status" => Command::DaemonStatus,
                _ => {
                    println!("Unknown daemon command: {}", args[2]);
                    println!("Usage: rsp daemon [start|stop|status]");
                    Command::Invalid
                }
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Run 'rsp help' for usage information");
            Command::Invalid
        }
    }
}

pub fn handle_command(command: Command) -> io::Result<()> {
    match command {
        Command::Add { name, password } => execute_add(name, password),
        Command::List => execute_list(),
        Command::Remove { name } => execute_remove(name),
        Command::Unlock { password } => execute_unlock(password),
        Command::Lock => execute_lock(),
        Command::Help => execute_help(),
        Command::Tui => Ok(()), // This will be handled in main.rs
        Command::StartDaemon => {
            if DaemonClient::is_running() {
                println!("Daemon is already running");
                Ok(())
            } else {
                println!("Starting daemon...");

                // Use our new start_daemon function
                match DaemonService::start_daemon() {
                    Ok(_) => {
                        println!("Daemon started successfully in background");

                        // Give it a moment to initialize
                        std::thread::sleep(std::time::Duration::from_millis(500));

                        if DaemonClient::is_running() {
                            println!("Daemon is now accepting connections");
                        } else {
                            println!(
                                "Warning: Daemon started but is not yet accepting connections"
                            );
                            println!("Check daemon logs for details");
                        }

                        Ok(())
                    }
                    Err(e) => {
                        println!("Failed to start daemon: {}", e);
                        Err(
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Failed to start daemon: {}", e)
                            )
                        )
                    }
                }
            }
        }
        Command::StopDaemon => {
            if DaemonClient::is_running() {
                let mut connection = interprocess::local_socket::LocalSocketStream::connect(
                    crate::daemon::ipc::get_socket_path()
                )?;
                serde_json
                    ::to_writer(&mut connection, &crate::daemon::ipc::DaemonCommand::Exit)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                println!("Daemon stopped");
                Ok(())
            } else {
                println!("Daemon is not running");
                Ok(())
            }
        }
        Command::DaemonStatus => {
            if DaemonClient::is_running() {
                match DaemonClient::get_state() {
                    Ok(state) => {
                        println!("Daemon is running");
                        println!("State: {}", if state.unlocked { "Unlocked" } else { "Locked" });

                        // Get log file contents
                        let log_path = dirs
                            ::data_dir()
                            .unwrap_or_else(|| PathBuf::from("/tmp"))
                            .join("rustpass/daemon.log");

                        if let Ok(log_content) = fs::read_to_string(log_path) {
                            let recent_logs: Vec<_> = log_content.lines().rev().take(10).collect();

                            println!("\nRecent daemon activity:");
                            for line in recent_logs.iter().rev() {
                                println!("  {}", line);
                            }
                        } else {
                            println!("\nCould not read daemon logs");
                        }

                        Ok(())
                    }
                    Err(e) => {
                        println!("Daemon is running but failed to get state: {}", e);
                        Ok(())
                    }
                }
            } else {
                println!("Daemon is not running");

                // Check if daemon crashed
                let pid_path = dirs
                    ::data_dir()
                    .unwrap_or_else(|| PathBuf::from("/tmp"))
                    .join("rustpass/daemon.pid");

                if pid_path.exists() {
                    println!("Warning: Found daemon PID file but daemon is not responding.");
                    println!("The daemon may have crashed. Check logs for details.");
                }

                Ok(())
            }
        }
        Command::Invalid => {
            execute_help()?;
            Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
        }
    }
}
