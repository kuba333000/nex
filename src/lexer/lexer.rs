use crate::lexer::tokens::OPERATORS;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::MatchKind;

use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    token_buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            token_buffer: String::from(""),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self, first: char) -> Token {
        let mut identifier = String::from(first);

        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                identifier.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        if matches!(identifier.as_str(), "if" | "else" | "func" | "def") {
            Token::Keyword(identifier)
        } else {
            Token::Identifier(identifier)
        }
    }

    fn consume_digits(&mut self, buf: &mut String) -> bool {
        let mut has_digit = false;

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                has_digit = true;

                buf.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        has_digit
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::from(first);

        let mut has_start_digits = self.consume_digits(&mut number);
        let mut has_end_digits = false;

        if first.is_ascii_digit() {
            has_start_digits = true;
        }
        
        if self.chars.peek() == Some(&'.') {
            number.push('.');
            self.chars.next();

            has_end_digits = self.consume_digits(&mut number);
        }

        if has_start_digits || has_end_digits {
            return Token::Number(number);
        } else {
            return Token::Dot;
        }
    }

    fn lookup_state(&self) -> MatchKind {
        let s = self.token_buffer.as_str();

        let token = OPERATORS
            .iter()
            .find(|(op, _)| *op == s)
            .map(|(_, tok)| tok.clone());

        let is_prefix = OPERATORS
            .iter()
            .any(|(op, _)| op.starts_with(s) && op.len() > s.len());

        match (token, is_prefix) {
            (Some(tok), true) => MatchKind::TokenAndPrefix(tok),
            (Some(tok), false) => MatchKind::Token(tok),
            (None, true) => MatchKind::Prefix,
            (None, false) => MatchKind::None,
        }
    }

    fn longest_match(&mut self, first: char) -> Token {
        self.token_buffer.clear();
        self.token_buffer.push(first);

        let mut last_match = None;

        loop {
            match self.lookup_state() {
                MatchKind::None => break,
                MatchKind::Prefix => {}
                MatchKind::Token(tok) => {
                    last_match = Some(tok);
                    break;
                }
                MatchKind::TokenAndPrefix(tok) => {
                    last_match = Some(tok);
                }
            }

            let Some(ch) = self.chars.peek() else {
                break;
            };

            self.token_buffer.push(*ch);
            self.chars.next();
        }

        last_match.unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(ch) = self.chars.next() else {
            return Token::EndOfFile;
        };

        match ch {
            c if c.is_ascii_digit() || c == '.' => self.read_number(c),
            c if c.is_alphabetic() || c == '_' => self.read_word(c),

            c if matches!(c, '-' | '>' | '<' | ':') => self.longest_match(c),

            '=' => Token::Equal,

            '>' => Token::Greater,
            '<' => Token::Less,

            '+' => Token::Plus,
            '*' => Token::Mult,
            '/' => Token::Div,

            ';' => Token::Semicolon,

            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,

            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            _ => Token::Invalid(ch.to_string()),
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            match token {
                Token::EndOfFile => {
                    tokens.push(token);
                    break;
                }
                Token::Invalid(c) => {
                    panic!("unexpected character: {}", c);
                }
                _ => {}
            }

            tokens.push(token);
        }

        tokens
    }
}