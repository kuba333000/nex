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

        match self.chars.next() {
            Some(ch) => match ch {
                c if c.is_ascii_digit() => self.read_number(c),
                c if c.is_alphabetic() || c == '_' => self.read_identifier(c),
                '+' => Token::Plus,
                '-' => Token::Minus,
                '=' => Token::Assign,
                ';' => Token::Semicolon,
                _ => panic!("unexpected character"),
            }
            None => Token::EndOfFile,
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

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::from(first);

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                number.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        
        if self.chars.peek() == Some(&'.') {
            number.push('.');
            self.chars.next();

            while let Some(c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    number.push(*c);
                    self.chars.next();
                } else {
                    break;
                }
            }
        }

        Token::Number(number)
    }
}