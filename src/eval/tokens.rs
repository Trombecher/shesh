pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Identifier(String),
    LeftAngle,
    RightAngle,
    EndOfInput,
}

pub enum Keyword {
    Nil,
    True,
    False,
    If,
    Else,
    While,
}

static KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf::phf_map! {
    "nil" => Keyword::Nil,
    "true" => Keyword::True,
    "false" => Keyword::False,
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "while" => Keyword::While,
};