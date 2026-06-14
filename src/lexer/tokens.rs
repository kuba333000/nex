#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Dot,
    Plus,
    Minus,
    Assign,
    Semicolon,
    EndOfFile,
    Invalid(String)
}