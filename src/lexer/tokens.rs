use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Keyword,
    Type,
    Identifier,

    String,
    Number,

    Defined,
    Equal,
    NotEq,

    Greater,
    GreaterEq,

    Less,
    LessEq,

    Dot,

    Concat,

    Plus,
    Minus,
    Mult,
    Div,

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
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_kind: TokenKind,
    pub lexeme: Option<String>,
    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn new(token_kind: TokenKind, lexeme: Option<String>, start: usize, length: usize) -> Self { Self { token_kind, lexeme, start, length } }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.lexeme {
            Some(lexeme) => write!(f, "{:?}(\"{}\")", self.token_kind, lexeme),
            None => write!(f, "{:?}", self.token_kind),
        }
    }
}