#![feature(str_from_raw_parts)]

mod text_box;
mod read;
mod runtime;
mod prompt;

use crate::read::bytes::Cursor;
use crate::read::lex::Lexer;
use crate::prompt::print_prompt;
use crate::text_box::TextBox;
use crossterm::cursor::{position, MoveTo, MoveToColumn};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{event, execute, queue};
use std::io::stdout;
use crate::read::buffered::Buffered;
use crate::read::parse::parse;
use crate::runtime::{eval, new_root_scope, Value};

fn main() -> ! {
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut root_scope = new_root_scope();

    let mut history = Vec::<(String, usize)>::new();
    let mut history_entry_index: Option<usize>;
    let mut text_box = TextBox::new();

    let syntax_highlighting = false;
    
    loop {
        text_box.clear();

        let mut stdout = stdout();

        history_entry_index = None;

        // queue!(stdout, MoveToColumn(0)).expect("Failed to move cursor to column");

        print_prompt();

        let (min_cursor_position, y) = position()
            .expect("Failed to get cursor position");

        loop {
            match event::read().expect("Failed to read an event") {
                Event::FocusGained => {}
                Event::FocusLost => {}
                Event::Key(KeyEvent { code: key, kind, .. }) => {
                    if kind == KeyEventKind::Release || kind == KeyEventKind::Repeat {
                        continue;
                    }

                    match key {
                        KeyCode::Backspace => text_box.remove_char_left(),
                        KeyCode::Enter => break,
                        KeyCode::Left => {
                            text_box.move_cursor_n_chars_left(1);
                        }
                        KeyCode::Right => {
                            text_box.move_cursor_n_chars_right(1);
                        }
                        KeyCode::Up => {
                            let new_history_entry_index = history_entry_index
                                .unwrap_or_else(|| history.len())
                                .saturating_sub(1);

                            // Update text box with new history entry

                            text_box.clear();

                            let (str, char_count) = &history[new_history_entry_index];
                            unsafe { text_box.insert_str_with_cached_char_count(str, *char_count); }

                            history_entry_index = Some(new_history_entry_index);
                        }
                        KeyCode::Down => {
                            let max = history.len() - 1;

                            let new_history_entry_index = (history_entry_index
                                .unwrap_or_else(|| max) + 1).min(max);

                            // Update text box with new history entry

                            text_box.clear();

                            let (str, char_count) = &history[new_history_entry_index];
                            unsafe { text_box.insert_str_with_cached_char_count(str, *char_count); }

                            history_entry_index = Some(new_history_entry_index);
                        }
                        KeyCode::Char(c) => {
                            text_box.insert_char(c);
                        }
                        _ => {}
                    };
                }
                Event::Mouse(_) => {}
                Event::Paste(_) => {}
                Event::Resize(_, _) => {}
            }

            queue!(
                stdout,
                MoveTo(min_cursor_position, y)
            ).expect("Failed to queue cursor movement");

            let partition = text_box.parts();

            if syntax_highlighting {

            } else {
                queue!(stdout, Print(partition.0), Print(partition.1))
                    .expect("Failed to queue partition");
            }

            execute!(
                stdout,
                Print(" "),
                MoveTo(text_box.chars_left_from_cursor() as u16 + min_cursor_position, y)
            ).expect("Failed to print input");
        }

        execute!(stdout, Print("\n\r")).unwrap();

        text_box.move_cursor_to_end();

        let (input, _) = text_box.parts();
        history.push((input.to_string(), text_box.chars_left_from_cursor()));

        if input.is_empty() {
            continue;
        }

        let mut iter = Buffered::new(Lexer::new(Cursor::new(input)));
        let root_expression = if let Ok(expr) = parse(&mut iter, 0) {
            expr
        } else {
            execute!(stdout, Print("Error: Failed to parse input\n\r"));
            continue;
        };

        let _ = stdout;

        disable_raw_mode().expect("Failed to disable raw mode");
        let result = eval(&mut root_scope, &root_expression);

        match result {
            Ok(Value::Nil) => {}
            Ok(value) => {
                println!("= {}", value);
            }
            Err(runtime_error) => {
                println!("Error: {:?}", runtime_error);
            }
        }

        enable_raw_mode().expect("Failed to enable raw mode");
    }
}