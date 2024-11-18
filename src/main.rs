mod text_box;
mod eval;
mod commands;
mod runtime;
mod prompt;

use crate::eval::lex::TokenIterator;
use crate::eval::tokens::Token;
use crate::prompt::print_prompt;
use crate::text_box::TextBox;
use crossterm::cursor::{position, MoveTo, MoveToColumn};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::style::{Print, Stylize};
use crossterm::terminal::enable_raw_mode;
use crossterm::{event, execute, queue};
use std::io::stdout;

fn main() -> ! {
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut history = Vec::<TextBox>::new();
    let mut text_box = TextBox::new();
    
    let syntax_highlighting = false;
    
    loop {
        queue!(stdout(), MoveToColumn(0));
    
        print_prompt();

        let (min_cursor_position, y) = position()
            .expect("Failed to get cursor position");

        text_box.clear();

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
                        KeyCode::Enter => {
                            println!();
                            break
                        },
                        KeyCode::Left => {
                            text_box.move_cursor_n_chars_left(1);
                        }
                        KeyCode::Right => {
                            text_box.move_cursor_n_chars_right(1);
                        }
                        KeyCode::Up => {}
                        KeyCode::Down => {}
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
            
            let mut stdout = stdout();
            
            queue!(
                stdout,
                MoveTo(min_cursor_position, y)
            ).expect("Failed to queue cursor movement");
            
            let partition = text_box.parts();
            
            if syntax_highlighting {
                let mut token_iterator = TokenIterator::new(partition);
                
                loop {
                    let token = match token_iterator.next() {
                        Ok(token) => token,
                        Err(_) => break,
                    };
                    
                    if let Token::EndOfInput = token.value {
                        break;
                    }
                    
                    let (a, b) = text_box.range(token.range);
                    
                    for part in [a, b] {
                        match &token.value {
                            Token::Number(_) => {
                                queue!(stdout, Print(part.cyan()))
                            }
                            Token::Identifier(_) => {
                                queue!(stdout, Print(part.yellow()))
                            }
                            _ => {
                                queue!(stdout, Print(part))
                            }
                        }.expect("Failed to print token");
                    }
                }
            } else {
                queue!(stdout, Print(partition.0), Print(partition.1))
                    .expect("Failed to queue partition");
            }

            execute!(
                stdout,
                Print(" "),
                MoveTo(text_box.chars_left_from_cursor() as u16 + min_cursor_position, y)
            ).expect("Failed to print input")
        }

        /*
        let (left, right) = text_box.parts();
        let input = format!("{}{}", left, right);

        let trimmed_input = input.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        let mut args = trimmed_input.trim().split_whitespace().into_iter();

        disable_raw_mode().expect("Failed to disable raw mode");

        if let Some((_, command)) = BUILT_IN_COMMANDS.get_entry(args.next().unwrap()) {
            command(args);
        } else {
            let mut command = std::process::Command::new(trimmed_input);
            command.args(args);
            if command.status().is_err() {
                execute!(stdout(), Print("Could not find program :(\n".red()))
                    .expect("Failed to print error message");
            }
        }

        enable_raw_mode().expect("Failed to enable raw mode");
         */
    }
}