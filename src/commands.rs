use std::io::stdout;
use std::str::SplitWhitespace;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

static BUILT_IN_COMMANDS: phf::Map<&str, fn(args: SplitWhitespace)> = phf::phf_map!(
    "exit" => |mut args| {
        std::process::exit(args.next()
            .and_then(|code: &str| code.parse().ok())
            .unwrap_or(0))
    },
    "cd" => |mut args| {
        let path = args.next()
            .unwrap_or(".");

        if let Err(_) = std::env::set_current_dir(path.to_string()) {
            eprintln!("Failed to change directory to {}", path);
        }
    },
    "clear" => |_| {
        if execute!(stdout(), Clear(ClearType::All), Clear(ClearType::Purge), MoveTo(0, 0)).is_err() {
            eprintln!("Failed to clear the screen");
        }
    },
    "pwd" => |_| {
        if let Ok(cwd) = std::env::current_dir() {
            println!("{}", cwd.display());
        } else {
            eprintln!("Failed to get current working directory");
        }
    }
    // "help" => "print help message"
);