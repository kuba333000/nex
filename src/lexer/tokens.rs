#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(String),
    Plus,
    Minus,
    Assign,
    Semicolon,
    EndOfFile,
    Invalid
}