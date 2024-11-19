#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Plus,
    PlusPlus,
    PlusEquals,
    Minus,
    MinusMinus,
    MinusEquals,
    Star,
    StarEquals,
    StarStar,
    StarStarEquals,
    Slash,
    SlashEquals,
    Percent,
    PercentEquals,
    Caret,
    CaretEquals,
    Ampersand,
    AmpersandEquals,
    AmpersandAmpersand,
    AmpersandAmpersandEquals,
    Pipe,
    PipeEquals,
    PipePipe,
    PipePipeEquals,
    ExclamationMark,
    ExclamationMarkEquals,
    Comma,
    Dot,
    LeftAngle,
    LeftAngleEquals,
    LeftAngleLeftAngle,
    LeftAngleLeftAngleEquals,
    RightAngle,
    RightAngleEquals,
    RightAngleRightAngle,
    RightAngleRightAngleEquals,
    Equals,
    EqualsEquals,

    Number(f64),
    Identifier(&'a str),
    Keyword(Keyword),
    String(&'a str),

    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    EndOfInput,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Nil,
    True,
    False,
    If,
    Else,
    While,
}

pub static KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf::phf_map! {
    "nil" => Keyword::Nil,
    "true" => Keyword::True,
    "false" => Keyword::False,
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "while" => Keyword::While,
};