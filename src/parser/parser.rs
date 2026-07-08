use crate::span::Span;

use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{Block, Parameter, UnaryOp, BinaryOp, ExprKind, StmtKind, DeclKind, TypeKind, Expr, Stmt, Decl, Type};

type BpSize = u8;

#[derive(Debug)]
pub enum ParseError {
    TokenKindError,
    NegativeBoundaryError,
    ExceededBoundaryError,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Self { Self { tokens: token_vec, pos: 0, } }

    fn peek(&self) -> Token { self.tokens.get(self.pos).cloned().unwrap() }

    fn look(&self, offset: isize) -> Result<Token, ParseError> {
        let index = self.pos.checked_add_signed(offset).ok_or(ParseError::NegativeBoundaryError)?;
        self.tokens.get(index).cloned().ok_or(ParseError::ExceededBoundaryError)
    }

    fn next(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn consume(&mut self, token_kind: TokenKind) -> Result<Token, ParseError> {
        let token = self.peek();
        if token.token_kind == token_kind {
            self.pos += 1;
            Ok(token)
        } else { Err(ParseError::TokenKindError) }
    }

    fn node_statement<F>(&mut self, f: F) -> Result<Stmt, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<StmtKind, ParseError>,
    {
        let start = self.peek().span.start;
        let kind = f(self)?;
        let end = self.look(-1)?.span.end;

        Ok(Stmt {
            kind,
            span: Span { start, end }
        })
    }

    fn try_consume(&mut self, token_kind: TokenKind) -> Option<Token> {
        let token = self.peek();
        if token.token_kind == token_kind {
            self.pos += 1;
            Some(token)
        } else { None }
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

    pub fn parse(&mut self) -> Result<Vec<Decl>, ParseError> {
        self.pos = 0;

        let mut nodes = Vec::new();
        while self.peek().token_kind != TokenKind::EndOfFile {
            nodes.push(self.declaration()?);
        }

        Ok(nodes)
    }

    fn declaration(&mut self) -> Result<Decl, ParseError> {
        let start = self.peek().span.start;

        let kind = match self.peek().token_kind {
            TokenKind::Let => self.global_variable_decl(),
            TokenKind::Func => self.function_signature(),
            TokenKind::Proc => self.procedure_signature(),
            TokenKind::Def => self.callable_definition(),
            
            _ => return Err(ParseError::TokenKindError),
        }?;

        let end = self.look(-1)?.span.end;

        Ok(Decl { kind, span: Span { start, end } })
    }

    fn global_variable_decl(&mut self) -> Result<DeclKind, ParseError> {
        self.consume(TokenKind::Let)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::Assign)?;
        let value = self.expression(0)?;
        self.consume(TokenKind::Semicolon)?;

        Ok(DeclKind::GlobalVariable { name, value: Box::new(value) })
    }

    fn function_signature(&mut self) -> Result<DeclKind, ParseError> {
        self.consume(TokenKind::Func)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::Colon)?;
        let domain = self.type_()?;
        self.consume(TokenKind::Arrow)?;
        let codomain = self.type_()?;
        self.consume(TokenKind::Semicolon)?;

        Ok(DeclKind::FunctionSignature { name, domain: Box::new(domain), codomain: Box::new(codomain) })
    }

    fn procedure_signature(&mut self) -> Result<DeclKind, ParseError> {
        self.consume(TokenKind::Proc)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::Colon)?;
        let domain = self.type_()?;
        self.consume(TokenKind::Semicolon)?;

        Ok(DeclKind::ProcedureSignature { name, domain: Box::new(domain) })
    }

    fn callable_definition(&mut self) -> Result<DeclKind, ParseError> {
        self.consume(TokenKind::Def)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::LeftParen)?;

        let mut paramaters = Vec::new();
        if self.peek().token_kind == TokenKind::Identifier {
            paramaters.push(Parameter {
                name: self.consume(TokenKind::Identifier)?.lexeme.unwrap(),
                default: None // TODO: handle default function parameters in the future
            });

            while self.peek().token_kind == TokenKind::Comma {
                paramaters.push(Parameter {
                    name: self.consume(TokenKind::Identifier)?.lexeme.unwrap(),
                    default: None // TODO: handle default function parameters in the future
                });
            }
        }

        self.consume(TokenKind::RightParen)?;
        self.consume(TokenKind::Defined)?;

        let body = self.block()?;

        Ok(DeclKind::Callable { name, paramaters, body })
    }

    fn type_(&mut self) -> Result<Type, ParseError> {
        let start = self.peek().span.start;

        if self.peek().token_kind == TokenKind::Identifier {
            let name = self.consume(TokenKind::Identifier)?;

            Ok(Type {
                kind: TypeKind::Named(name.lexeme.unwrap()),
                span: Span { start, end: self.look(-1)?.span.end },
            })
        } else {
            self.consume(TokenKind::LeftParen)?;
            if self.peek().token_kind == TokenKind::RightParen { // unit type ()
                self.consume(TokenKind::RightParen)?;
                
                return Ok(Type {
                    kind: TypeKind::Tuple(Vec::new()),
                    span: Span { start, end: self.look(-1)?.span.end },
                });
            }
            
            let mut types = Vec::new();

            // at least two-element tuple
            types.push(self.type_()?);
            self.consume(TokenKind::Comma)?;
            types.push(self.type_()?);

            while self.peek().token_kind == TokenKind::Comma {
                self.consume(TokenKind::Comma)?;
                types.push(self.type_()?);
            }

            self.consume(TokenKind::RightParen)?;

            Ok(Type {
                kind: TypeKind::Tuple(types),
                span: Span { start, end: self.look(-1)?.span.end },
            })
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        let start = self.peek().span.start;

        let kind = match self.peek().token_kind {
            TokenKind::Let => self.local_variable_decl(),
            TokenKind::If => self.if_statement(),
            
            _ => return Err(ParseError::TokenKindError),
        }?;

        let end = self.look(-1).unwrap().span.end;

        Ok(Stmt { kind, span: Span { start, end } })
    }

    fn local_variable_decl(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::Let)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::Assign)?;
        let value = self.expression(0)?;
        self.consume(TokenKind::Semicolon)?;

        Ok(StmtKind::LocalVariable { name, value: Box::new(value) })
    }

    fn if_statement(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::If)?;
        let cond = Box::new(self.expression(0)?);
        let if_body = self.block()?;

        if self.try_consume(TokenKind::Else).is_some() {
            self.consume(TokenKind::Else)?;

            let else_body = match self.peek().token_kind {
                TokenKind::If => self.node_statement(Self::if_statement),
                TokenKind::LeftBrace => self.node_statement(|parser| Ok(StmtKind::Block(parser.block()?))),

                _ => Err(ParseError::TokenKindError),
            }?;

            return Ok(StmtKind::If { cond, if_body, else_body: Some(Box::new(else_body)) });
        };

        Ok(StmtKind::If { cond, if_body, else_body: None })
    }

    fn block(&mut self) -> Result<Block, ParseError> {
        let mut nodes = Vec::new();
        while self.peek().token_kind != TokenKind::RightBrace {
            nodes.push(self.statement()?)
        };

        Ok(Block { statements: nodes })
    }

    fn expression(&mut self, min_bp: BpSize) -> Result<Expr, ParseError> {
        let mut lhs = self.prefix()?;
        
        loop {
            let Some((op, lbp, rbp)) = self.get_infix_op(self.peek().token_kind) else {
                break;
            };

            if lbp < min_bp {
                break;
            }
            
            self.next();

            let rhs = self.expression(rbp)?;

            lhs = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(lhs.clone()),
                    right: Box::new(rhs.clone()),
                },
                span: Span::new_by_spans(lhs.span, rhs.span),
            };
        }

        Ok(lhs)
    }

    fn prefix(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();
        let kind = token.token_kind;

        if let Some((op, bp)) = self.get_prefix_op(kind.clone()) {
            let op_span = token.span;
            self.consume(kind)?;

            let operand = self.expression(bp)?;

            let span = Span::new_by_spans(op_span, operand.span);
            return Ok(Expr {
                kind: ExprKind::Unary {
                    op,
                    expr: Box::new(operand),
                },
                span,
            });
        }

        self.literal()
    }

    fn literal(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.token_kind {
            TokenKind::Number => {
                self.consume(TokenKind::Number)?;

                let lexeme = token.lexeme.unwrap();
                
                if lexeme.contains('.')
                { Ok(ExprKind::Real(lexeme.parse::<f64>().unwrap())) } else { Ok(ExprKind::Integer(lexeme.parse::<i64>().unwrap())) }
            },
            
            _ => Err(ParseError::TokenKindError),
        }?;

        Ok(Expr {
            kind,
            span: Span { start, end: self.look(-1)?.span.end },
        })
    }
}