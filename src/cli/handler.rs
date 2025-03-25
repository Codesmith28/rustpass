use std::io;

use crate::cli::commands::{
    execute_add, execute_help, execute_list, execute_lock, execute_remove, execute_unlock,
};

pub enum Command {
    Add { name: String, password: String },
    List,
    Remove { name: String },
    Unlock { password: Option<String> },
    Lock,
    Help,
    Tui,
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
        Command::Invalid => {
            execute_help()?;
            Err(io::Error::new(io::ErrorKind::InvalidInput, ""))
        }
    }
} 