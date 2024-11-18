mod functions;

use std::str::SplitWhitespace;
use std::collections::HashMap;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

pub struct Variable {
    mutable: bool,
    value: Value,
}

pub enum Value {
    Number(f64),
    Nil,
    Function(fn(args: SplitWhitespace)),
}

pub type Scope = HashMap<String, Variable>;

pub fn new_root_scope() -> Scope {
    let mut scope = Scope::new();

    scope.insert("clear".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            if execute!(stdout(), Clear(ClearType::All), Clear(ClearType::Purge), MoveTo(0, 0)).is_err() {
                eprintln!("Failed to clear the screen");
            }
        })
    });

    scope
}