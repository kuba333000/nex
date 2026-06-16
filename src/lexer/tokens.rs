pub const OPERATORS: &[(&str, Token)] = &[
    ("->", Token::Arrow),
    ("-", Token::Minus),
    ("!=", Token::NotEq),
    (">=", Token::GreaterEq),
    (">", Token::Greater),
    ("<=", Token::LessEq),
    ("<", Token::Less),
    (":=", Token::Defined),
    (":", Token::Colon),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Identifier(String),

    Number(String),

    Defined,
    Equal,
    NotEq,

    Greater,
    GreaterEq,

    Less,
    LessEq,

    Dot,

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
    Invalid(String)
}

pub enum MatchKind {
    None,
    Prefix,
    Token(Token),
    TokenAndPrefix(Token),
}