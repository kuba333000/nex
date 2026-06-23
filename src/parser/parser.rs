use crate::span::Span;

use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{AstNode, AstNodeKind, Expr, UnaryOp};

type Bp = u8;

#[derive(Debug)]
pub struct SyntaxError;

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

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token, SyntaxError> {
        let consumed = self.peek();
        if consumed.token_kind == token_kind {
            self.pos += 1;
            Ok(consumed)
        } else {
            Err(SyntaxError)
        }
    }
    
    fn get_unary_info(&self, kind: TokenKind) -> Option<(UnaryOp, Bp)> {
        match kind {
            TokenKind::Sub => Some((UnaryOp::Neg, 30)),
            _ => None,
        }
    }

    fn get_infix_bp(&self, op: TokenKind) -> Option<(Bp, Bp)> {
        match op {
            TokenKind::Assign => Some((10, 9)),
            TokenKind::Equal
            | TokenKind::NotEq   => Some((11, 12)),
            TokenKind::Add
            | TokenKind::Sub => Some((19, 20)),
            TokenKind::Mul
            | TokenKind::Div => Some((21, 22)),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.pos = 0;

        let mut nodes = Vec::new();
        while let Some(node) = self.parse_statement() {
            nodes.push(node);
        }

        nodes
    }

    fn parse_statement(&mut self) -> Result<Option<AstNode>, SyntaxError> {
        let node = self.parse_expression()?;
        // TODO
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Option<AstNode>, SyntaxError> {
        let node = self.parse_prefix()?;
        // TODO
    }

    fn parse_prefix(&mut self) -> Result<Option<AstNode>, SyntaxError> {
        let token = self.peek();
        let kind = token.token_kind;

        if let Some((op, bp)) = self.get_unary_info(kind.clone()) {
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

    fn parse_literal(&mut self) -> Result<Option<AstNode>, SyntaxError> {
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