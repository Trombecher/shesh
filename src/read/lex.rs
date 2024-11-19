use std::str::from_raw_parts;
use crate::read::bytes::{Cursor, Span};
use crate::read::tokens::Token;

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self { cursor }
    }

    pub fn next(&mut self) -> Result<Span<Token<'a>>, ()> {
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
            Some(b'+') => {
                self.cursor.advance();
                Ok(Token::Plus)
            },
            Some(b'-') => {
                self.cursor.advance();
                Ok(Token::Minus)
            },
            Some(b'*') => {
                self.cursor.advance();
                Ok(Token::Star)
            },
            Some(b'/') => {
                self.cursor.advance();
                Ok(Token::Slash)
            },
            Some(b'^') => {
                self.cursor.advance();
                Ok(Token::Caret)
            },
            Some(b'(') => {
                self.cursor.advance();
                Ok(Token::LeftParenthesis)
            },
            Some(b')') => {
                self.cursor.advance();
                Ok(Token::RightParenthesis)
            },
            Some(_) => {
                let current = self.cursor.pointer();
                
                while let Some(byte) = self.cursor.peek() {
                    if !byte.is_ascii_alphanumeric() {
                        break;
                    }
                    self.cursor.advance();
                }
                
                Ok(Token::Identifier(unsafe {
                    from_raw_parts(current, self.cursor.pointer() as usize - current as usize)
                }))
            },
            None => Ok(Token::EndOfInput),
        };

        token.map(|token| Span {
            value: token,
            range: start_index..self.cursor.index(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_lex() {
        let mut lexer = Lexer::new(Cursor::new(" + - * / ** % 789.012"));

        assert_eq!(lexer.next(), Ok(Span { value: Token::Plus, range: 0..1 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Minus, range: 2..3 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Star, range: 4..5 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Slash, range: 6..7 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Star, range: 8..9 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Star, range: 10..11 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::Number(789.012), range: 12..20 }));
        assert_eq!(lexer.next(), Ok(Span { value: Token::EndOfInput, range: 20..20 }));
    }
}