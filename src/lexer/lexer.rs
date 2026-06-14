use crate::lexer::tokens::Token;

use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(ch) = self.chars.next() else {
            return Token::EndOfFile;
        };

        match ch {
            c if c.is_ascii_digit() || c == '.' => self.read_number(c),
            c if c.is_alphabetic() || c == '_' => self.read_identifier(c),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            _ => Token::Invalid(ch.to_string()),
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

    fn read_identifier(&mut self, first: char) -> Token {
        let mut identifier = String::from(first);

        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                identifier.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        Token::Identifier(identifier)
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
}