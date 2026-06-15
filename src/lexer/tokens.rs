#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Identifier(String),

    Number(String),
    
    Assign,

    Dot,

    Plus,
    Minus,

    Semicolon,

    Colon,
    Arrow,

    LeftParen,
    RightParen,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    EndOfFile,
    Invalid(String)
}