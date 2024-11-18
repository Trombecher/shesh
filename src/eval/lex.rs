use crate::eval::bytes::{Cursor, Span};
use crate::eval::tokens::Token;

pub struct TokenIterator<'a> {
    cursor: Cursor<'a>,
}

impl<'a> TokenIterator<'a> {
    #[inline]
    pub fn new(partition: (&'a str, &'a str)) -> Self {
        TokenIterator {
            cursor: Cursor::new(partition),
        }
    }

    pub fn next(&mut self) -> Result<Span<Token>, ()> {
        self.cursor.skip_whitespace();

        let start_index = self.cursor.index();
        
        let token = match self.cursor.peek() {
            Some(first_digit) if matches!(first_digit, b'0'..=b'9') => {
                let mut number = (first_digit - b'0') as f64;

                self.cursor.advance();

                loop {
                    match self.cursor.peek() {
                        Some(x) if matches!(x, b'0'..=b'9') => {
                            number = number * 10.0 + (x - b'0') as f64;
                            self.cursor.advance();
                        },
                        _ => break,
                    }
                }

                if self.cursor.peek() == Some(b'.') {
                    self.cursor.advance();

                    let mut decimal = 0.1;

                    loop {
                        match self.cursor.peek() {
                            Some(x) if matches!(x, b'0'..=b'9') => {
                                number += (x - b'0') as f64 * decimal;
                                decimal *= 0.1;
                                self.cursor.advance();
                            },
                            _ => break,
                        }
                    }
                }

                Ok(Token::Number(number))
            }
            Some(_) => {
                let mut identifier = String::new();
                
                while let Some(byte) = self.cursor.peek() {
                    if byte.is_ascii_alphanumeric() {
                        identifier.push(byte as char);
                        self.cursor.advance();
                    } else {
                        break;
                    }
                }
                
                Ok(Token::Identifier(identifier))
            },
            None => Ok(Token::EndOfInput),
        };

        token.map(|token| Span {
            value: token,
            range: start_index..self.cursor.index(),
        })
    }
}