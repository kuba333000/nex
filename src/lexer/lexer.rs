use crate::lexer::tokens::Token;

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

    fn lookup_token_table(&mut self) -> Option<Token> {
        match self.token_buffer.as_str() {
            "->" => Some(Token::Arrow),
            "-" => Some(Token::Minus),
            _ => None,
        }
    }

    fn longest_match(&mut self, first: char) -> Token { // TODO: fix this function
        self.token_buffer = String::from(first);
        
        loop {
            let Some(ch) = self.chars.peek() else {
                break self.lookup_token_table().unwrap();
            };

            self.token_buffer.push(*ch);
            let Some(_contender) = self.lookup_token_table() else {
                self.token_buffer.pop();
                break self.lookup_token_table().unwrap();
            };
        }        
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(ch) = self.chars.next() else {
            return Token::EndOfFile;
        };

        match ch {
            c if c.is_ascii_digit() || c == '.' => self.read_number(c),
            c if c.is_alphabetic() || c == '_' => self.read_word(c),

            c if matches!(c, '-') => self.longest_match(c),

            '=' => Token::Assign,

            '+' => Token::Plus,

            ';' => Token::Semicolon,
            ':' => Token::Colon,

            '(' => Token::LeftParen,
            ')' => Token::RightParen,

            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,

            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,

            _ => Token::Invalid(ch.to_string()),
        }
    }
}