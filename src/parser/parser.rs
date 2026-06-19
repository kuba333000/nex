// use crate::lexer::tokens::TokenKind;
use crate::lexer::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Self { Self { tokens: token_vec, pos: 0, } }

    fn peek(&self) -> Option<Token> { self.tokens.get(self.pos).cloned() }

    // fn lookahead(&self) -> Option<Token> { self.tokens.get(self.pos + offset).copied() }

    fn next(&mut self) -> Option<Token> {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    // fn parse(&mut self) -> 
}