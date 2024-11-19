use crate::read::bytes::Span;
use crate::read::lex::Lexer;
use crate::read::tokens::Token;

pub struct Buffered<'a> {
    lexer: Lexer<'a>,
    force_take: Option<Span<Token<'a>>>,
}

impl<'a> Buffered<'a> {
    #[inline]
    pub const fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            force_take: None,
        }
    }

    #[inline]
    pub fn peek(&mut self) -> Result<&Span<Token<'a>>, ()> {
        Ok(self.force_take.get_or_insert(self.lexer.next()?))
    }

    #[inline]
    pub fn next(&mut self) -> Result<Span<Token<'a>>, ()> {
        match self.force_take.take() {
            Some(token) => Ok(token),
            None => self.lexer.next(),
        }
    }
}