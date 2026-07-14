use crate::span::Span;

use crate::lexer::tokens::{TokenKind, Token};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self { Self { chars: input.chars().collect(), pos: 0, } }

    fn peek(&self) -> Option<char> { self.chars.get(self.pos).copied() }

    fn peek_slice(&self, length: usize) -> Option<&[char]> { self.chars.get(self.pos..self.pos + length) }

    fn peek_eq(&self, s: &str) -> bool {
        self.peek_slice(s.chars().count())
            .is_some_and(|c| c.iter().copied().eq(s.chars()))
    }

    // fn lookahead(&mut self, offset: usize) -> Option<char> { self.chars.get(self.pos + offset).copied() }

    fn next(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn advance_n(&mut self, length: usize) -> Option<&[char]> {
        let slice = self.chars.get(self.pos..self.pos + length);
        self.pos += length;
        slice
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> Token {
        let start_pos = self.pos;
        let first = self.next().unwrap();
        let mut identifier = String::from(first);

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.next();
            } else {
                break;
            }
        }

        let kind = match identifier.as_str() {
            "let" => TokenKind::Let,
            "func" => TokenKind::Func,
            "proc" => TokenKind::Proc,
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "leave" => TokenKind::Leave,
            "if" => TokenKind::If,
            "when" => TokenKind::When,
            "else" => TokenKind::Else,

            _ => TokenKind::Identifier,
        };

        Token {
            token_kind: kind.clone(),
            lexeme: if kind == TokenKind::Identifier { Some(identifier.clone()) } else { None },
            span: Span { start: start_pos, end: self.pos },
        }
    }

    fn consume_digits(&mut self, buf: &mut String) -> bool {
        let mut has_digit = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                has_digit = true;

                buf.push(c);
                self.next();
            } else {
                break;
            }
        }

        has_digit
    }

    fn read_number(&mut self) -> Token {
        let start_pos = self.pos;
        let first = self.next().unwrap();
        let mut number = String::from(first);

        let mut has_start_digits = self.consume_digits(&mut number);
        let mut has_end_digits = false;

        if first.is_ascii_digit() {
            has_start_digits = true;
        }
        
        if self.peek() == Some('.') {
            number.push('.');
            self.next();

            has_end_digits = self.consume_digits(&mut number);
        }

        Token {
            token_kind: if has_start_digits || has_end_digits { TokenKind::Number }
            else { TokenKind::Dot },

            lexeme: Some(number),
            span: Span { start: start_pos, end: self.pos },
        }
    }

    fn read_string(&mut self) -> Token {
        let start_pos = self.pos;
        let mut string = String::from(self.next().unwrap());

        while let Some(c) = self.peek() {
            if c == '"' {
                self.next();
                break;
            }

            string.push(c);
            self.next();
        }

        Token {
            token_kind: TokenKind::String,
            lexeme: Some(string),
            span: Span { start: start_pos, end: self.pos },
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start_pos = self.pos;

        let Some(ch) = self.peek() else {
            return Token {
                token_kind: TokenKind::EndOfFile,
                lexeme: None,
                span: Span { start: start_pos, end: self.pos },
            };
        };

        // lexeme mapping
        let token = match ch {
            c if c.is_alphabetic() || c == '_' => Some(self.read_word()),
            c if c.is_ascii_digit() || c == '.' => Some(self.read_number()),
            c if c == '"' => Some(self.read_string()),

            _ => None
        };

        if token.is_some() {
            return token.unwrap();
        }

        // multi-character mapping
        let token = match () {
            _ if self.peek_eq(":=") => Some(Token::new(TokenKind::Defined, None, Span { start: start_pos, end: start_pos + 2 })),

            _ if self.peek_eq("->") => Some(Token::new(TokenKind::Arrow, None, Span { start: start_pos, end: start_pos + 2 })),
            
            _ if self.peek_eq("==") => Some(Token::new(TokenKind::Equal, None, Span { start: start_pos, end: start_pos + 2 })),
            _ if self.peek_eq("!=") => Some(Token::new(TokenKind::NotEq, None, Span { start: start_pos, end: start_pos + 2 })),
            _ if self.peek_eq(">=") => Some(Token::new(TokenKind::GreaterEq, None, Span { start: start_pos, end: start_pos + 2 })),
            _ if self.peek_eq("<=") => Some(Token::new(TokenKind::LessEq, None, Span { start: start_pos, end: start_pos + 2 })),

            _ => None
        };

        if token.is_some() {
            let token_unwrapped = token.unwrap();
            self.advance_n(token_unwrapped.span.get_length());
            return token_unwrapped;
        }

        // single character mapping
        let kind = match ch {
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,

            ',' => TokenKind::Comma,

            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,

            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,

            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,

            '=' => TokenKind::Assign,

            '>' => TokenKind::Greater,
            '<' => TokenKind::Less,

            '&' => TokenKind::Concat,

            '+' => TokenKind::Add,
            '-' => TokenKind::Sub,
            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,

            _ => TokenKind::Invalid,
        };

        self.next();

        Token {
            token_kind: kind.clone(),
            lexeme: if kind == TokenKind::Invalid { Some(ch.to_string()) } else { None },
            span: Span { start: start_pos, end: self.pos },
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        self.pos = 0;
        
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            match token {
                Token { token_kind: TokenKind::EndOfFile, .. } => {
                    tokens.push(token);
                    break;
                }
                Token { token_kind: TokenKind::Invalid, lexeme: Some(c), .. } => {
                    panic!("unexpected character: {}", c);
                }
                _ => {}
            }

            tokens.push(token);
        }

        tokens
    }
}