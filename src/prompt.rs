use std::io::{stdout, Write};

pub fn print_prompt() {
    let cwd = std::env::current_dir();

    if let Ok(cwd) = cwd {
        print!("{}> ", cwd.display());
    } else {
        print!("?> ");
    }

    stdout().flush()
        .expect("Failed to flush stdout");
}