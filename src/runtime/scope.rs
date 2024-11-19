use crate::runtime::{Value, Variable};
use crossterm::cursor::MoveTo;
use crossterm::{execute, queue};
use crossterm::terminal::{Clear, ClearType};
use std::collections::HashMap;
use std::io::{stdout, Write};
use crossterm::style::Print;

pub type Scope = HashMap<String, Variable>;

pub fn new_root_scope() -> Scope {
    let mut scope = Scope::new();

    for (name, value) in std::env::vars() {
        scope.insert(name, Variable {
            mutable: false,
            value: Value::String(value)
        });
    }

    scope.insert("clear".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            execute!(stdout(), Clear(ClearType::All), Clear(ClearType::Purge), MoveTo(0, 0))
                .unwrap();

            Ok(Value::Nil)
        })
    });

    scope.insert("exit".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            std::process::exit(0)
        })
    });

    scope.insert("pwd".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            if let Ok(cwd) = std::env::current_dir() {
                println!("{}", cwd.display());
            } else {
                eprintln!("CWD does not exist");
            }

            Ok(Value::Nil)
        })
    });

    scope.insert("debug_print_scope".to_string(), Variable {
        mutable: false,
        value: Value::Function(|scope| {
            println!("{:#?}", scope);
            Ok(Value::Nil)
        })
    });

    scope.insert("ls".to_string(), Variable {
        mutable: false,
        value: Value::Function(|_| {
            let mut stdout = stdout();

            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        queue!(stdout, Print(entry.file_name().to_string_lossy()), Print("\n"))
                            .unwrap();
                    }
                }

                stdout.flush().unwrap();
            } else {
                eprintln!("Failed to read directory");
            }

            Ok(Value::Nil)
        })
    });

    scope
}