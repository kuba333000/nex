use std::fmt;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Func,
    Proc,
    Def,
    Return,
    Leave,
    If,
    When,
    Else,

    Identifier,

    String,
    Number,

    Assign,
    Walrus,

    Equal,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    Dot,

    Concat,

    Add,
    Sub,
    Mul,
    Div,

    Semicolon,

    Colon,
    Arrow,
    Comma,

    LeftParen,
    RightParen,

    LeftBracket,
    RightBracket,

    LeftBrace,
    RightBrace,

    EndOfFile,
    Invalid,

    Missing(Box<TokenKind>),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<String>, span: Span) -> Self { Self { kind, lexeme, span } }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.lexeme {
            Some(lexeme) => write!(f, "{:?}(\"{}\")", self.kind, lexeme.escape_debug()),
            None => write!(f, "{:?}", self.kind),
        }
    }
}