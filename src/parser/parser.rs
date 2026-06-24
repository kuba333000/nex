use crate::span::Span;

use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{AstNode, AstNodeKind, Expr, UnaryOp, BinaryOp};

type BpSize = u8;

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
    EOFError,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Self { Self { tokens: token_vec, pos: 0, } }

    fn peek(&self) -> Token { self.tokens.get(self.pos).cloned().unwrap() }

    // fn lookahead(&self) -> Option<Token> { self.tokens.get(self.pos + offset).copied() }

    fn next(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token, ParseError> {
        let consumed = self.peek();
        if consumed.token_kind == token_kind {
            self.pos += 1;
            Ok(consumed)
        } else {
            Err(ParseError::SyntaxError)
        }
    }
    
    fn get_prefix_op(&self, kind: TokenKind) -> Option<(UnaryOp, BpSize)> {
        match kind {
            TokenKind::Sub => Some((UnaryOp::Neg, 30)),
            _ => None,
        }
    }

    fn get_infix_op(&self, op: TokenKind) -> Option<(BinaryOp, BpSize, BpSize)> {
        match op {
            TokenKind::Equal => Some((BinaryOp::Equal, 9, 10)),
            TokenKind::NotEq => Some((BinaryOp::NotEq, 9, 10)),
            TokenKind::Add => Some((BinaryOp::Add, 19, 20)),
            TokenKind::Sub => Some((BinaryOp::Sub, 19, 20)),
            TokenKind::Mul => Some((BinaryOp::Mult, 21, 22)),
            TokenKind::Div => Some((BinaryOp::Div, 21, 22)),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AstNode>, ParseError> {
        self.pos = 0;

        let mut nodes = Vec::new();
        loop {
            match self.parse_statement() {
                Ok(node) => nodes.push(node),
                Err(ParseError::EOFError) => break Ok(nodes),
                Err(ParseError::SyntaxError) => break Err(ParseError::SyntaxError),
            }
        }
    }

    fn parse_statement(&mut self) -> Result<AstNode, ParseError> { // TODO
        if self.peek().token_kind == TokenKind::EndOfFile { return Err(ParseError::EOFError); };

        self.parse_expression(0)?.ok_or(ParseError::SyntaxError)
    }

    fn parse_expression(&mut self, min_bp: BpSize) -> Result<Option<AstNode>, ParseError> {
        let mut lhs = self.parse_prefix()?.ok_or(ParseError::SyntaxError)?;
        
        loop {
            let Some((op, lbp, rbp)) = self.get_infix_op(self.peek().token_kind) else {
                break;
            };

            if lbp < min_bp {
                break;
            }
            
            self.next();

            let rhs = self.parse_expression(rbp)?.ok_or(ParseError::SyntaxError)?;

            lhs = AstNode {
                category: AstNodeKind::Expr {
                    kind: Expr::Binary {
                        op,
                        left: Box::new(lhs.clone()),
                        right: Box::new(rhs.clone()),
                    }
                },
                span: Span::new_by_spans(lhs.span, rhs.span),
            };
        }

        Ok(Some(lhs))
    }

    fn parse_prefix(&mut self) -> Result<Option<AstNode>, ParseError> {
        let token = self.peek();
        let kind = token.token_kind;

        if let Some((op, bp)) = self.get_prefix_op(kind.clone()) {
            let op_span = token.span;
            self.consume(kind)?;

            let operand = self.parse_expression(bp)?.unwrap();

            let span = Span::new_by_spans(op_span, operand.span);
            return Ok(Some(AstNode {
                category: AstNodeKind::Expr {
                    kind: Expr::Unary {
                        op,
                        expr: Box::new(operand),
                    }
                },
                span,
            }));
        }

        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Result<Option<AstNode>, ParseError> {
        let token = self.peek();

        let literal_kind = match token.token_kind {
            TokenKind::StrType => { self.consume(TokenKind::StrType)?; Some(Expr::StrType) },
            TokenKind::IntType => { self.consume(TokenKind::IntType)?; Some(Expr::IntType) },

            TokenKind::Number => {
                self.consume(TokenKind::Number)?;

                let lexeme = token.lexeme.unwrap();
                
                if lexeme.contains('.')
                { Some(Expr::Real(lexeme.parse::<f64>().unwrap())) } else { Some(Expr::Integer(lexeme.parse::<i64>().unwrap())) }
            },
            
            _ => None,
        };

        if literal_kind.is_none() { Ok(None) } else {
            Ok(Some(AstNode {
                category: AstNodeKind::Expr { kind: literal_kind.unwrap() },
                span: token.span,
            }))
        }
    }
}