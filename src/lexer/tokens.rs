use std::fmt;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Display,
    Let,
    Func,
    Def,
    Return,
    If,
    Else,

    IntType,
    StrType,

    String,
    Number,

    Identifier,

    Assign,
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
    pub span: Span,
}

impl Token {
    pub fn new(token_kind: TokenKind, lexeme: Option<String>, span: Span) -> Self { Self { token_kind, lexeme, span } }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.lexeme {
            Some(lexeme) => write!(f, "{:?}(\"{}\")", self.token_kind, lexeme.escape_debug()),
            None => write!(f, "{:?}", self.token_kind),
        }
    }
}