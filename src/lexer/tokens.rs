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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Let => write!(f, "let"),
            TokenKind::Func => write!(f, "func"),
            TokenKind::Proc => write!(f, "proc"),
            TokenKind::Def => write!(f, "def"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Leave => write!(f, "leave"),
            TokenKind::If => write!(f, "if"),
            TokenKind::When => write!(f, "when"),
            TokenKind::Else => write!(f, "else"),

            TokenKind::Identifier => write!(f, "identifier"),

            TokenKind::String => write!(f, "string"),
            TokenKind::Number => write!(f, "number"),

            TokenKind::Assign => write!(f, "="),

            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEq => write!(f, ">="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEq => write!(f, "<="),

            TokenKind::Dot => write!(f, "."),

            TokenKind::Concat => write!(f, "&"),

            TokenKind::Add => write!(f, "+"),
            TokenKind::Sub => write!(f, "-"),
            TokenKind::Mul => write!(f, "*"),
            TokenKind::Div => write!(f, "/"),

            TokenKind::Semicolon => write!(f, ";"),

            TokenKind::Colon => write!(f, ":"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Comma => write!(f, ","),

            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),

            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),

            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),

            TokenKind::EndOfFile => write!(f, "EOF"),
            TokenKind::Invalid => write!(f, "invalid"),

            TokenKind::Missing(token) => write!(f, "missing token ({})", token),
        }
    }
}