use std::fmt::DebugList;

use crate::span::Span;
use crate::diagnostic::{Severity, Diagnostic};

use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{Block, Parameter, UnaryOp, BinaryOp, ExprKind, StmtKind, DeclKind, TypeKind, Expr, Stmt, Decl, Type};

type BpSize = u8;

macro_rules! token_kinds {
    ($($token:ident)|*) => {
        vec![
            $(
                TokenKind::$token,
            )*
        ]
    };
}

pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    diagnostics: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Self { Self { tokens: token_vec, pos: 0, diagnostics: Vec::new() } }

    fn peek(&self) -> Token { self.tokens.get(self.pos).cloned().unwrap() }

    fn look(&self, offset: isize) -> Token {
        let index = self.pos.checked_add_signed(offset).unwrap();
        self.tokens.get(index).cloned().unwrap()
    }

    fn next(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn error(&mut self, span: Span, message: String) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message,
            span,
        });
    }

    fn consume(&mut self, token_kind: TokenKind) -> Token {
        let token = self.peek();
        if token.token_kind == token_kind {
            self.pos += 1;
            token
        } else {
            self.error(token.span, format!("Expected token kind \"{:?}\", found \"{:?}\" instead.", token_kind, token.token_kind));
            Token { token_kind: TokenKind::Missing(Box::new(token_kind)), lexeme: None, span: token.span }
        }
    }

    fn node_statement<F>(&mut self, f: F) -> Result<Stmt, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<StmtKind, ParseError>,
    {
        let start = self.peek().span.start;
        let kind = f(self)?;
        let end = self.look(-1).span.end;

        Ok(Stmt {
            kind,
            span: Span { start, end }
        })
    }

    fn node_expression<F>(&mut self, f: F) -> Result<Expr, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<ExprKind, ParseError>,
    {
        let start = self.peek().span.start;
        let kind = f(self)?;
        let end = self.look(-1).span.end;

        Ok(Expr {
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
            TokenKind::Assign => Some((BinaryOp::Assign, 1, 2)),
            
            TokenKind::Greater => Some((BinaryOp::Greater, 3, 4)),
            TokenKind::Less => Some((BinaryOp::Less, 3, 4)),
            TokenKind::GreaterEq => Some((BinaryOp::GreaterEq, 3, 4)),
            TokenKind::LessEq => Some((BinaryOp::LessEq, 3, 4)),

            TokenKind::Equal => Some((BinaryOp::Equal, 4, 5)),
            TokenKind::NotEq => Some((BinaryOp::NotEq, 4, 5)),

            TokenKind::Concat => Some((BinaryOp::Concat, 6, 7)),

            TokenKind::Add => Some((BinaryOp::Add, 8, 9)),
            TokenKind::Sub => Some((BinaryOp::Sub, 8, 9)),
            TokenKind::Mul => Some((BinaryOp::Mult, 10, 11)),
            TokenKind::Div => Some((BinaryOp::Div, 10, 11)),
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

    fn declaration(&mut self) -> Decl {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.token_kind {
            TokenKind::Let => self.global_variable_decl(),
            TokenKind::Func => self.function_signature(),
            TokenKind::Proc => self.procedure_signature(),
            TokenKind::Def => self.callable_definition(),
            
            _ => {
                self.error(Span { start, end: token.span.end }, format!("Expected declaration to start with 'let', 'func', 'proc', 'def'. Found {:?}", token.token_kind));
                DeclKind::Error
            },
        };

        let end = self.look(-1)?.span.end;

        Decl { kind, span: Span { start, end } }
    }

    fn global_variable_decl(&mut self) -> DeclKind {
        self.consume(TokenKind::Let);
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();
        self.consume(TokenKind::Assign);
        let value = self.expression();
        self.consume(TokenKind::Semicolon);

        DeclKind::GlobalVariable { name, value: Box::new(value) }
    }

    fn function_signature(&mut self) -> DeclKind {
        self.consume(TokenKind::Func);
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();
        self.consume(TokenKind::Colon);
        let domain = self.type_();
        self.consume(TokenKind::Arrow);
        let codomain = self.type_();
        self.consume(TokenKind::Semicolon);

        DeclKind::FunctionSignature { name, domain: Box::new(domain), codomain: Box::new(codomain) }
    }

    fn procedure_signature(&mut self) -> DeclKind {
        self.consume(TokenKind::Proc);
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();
        self.consume(TokenKind::Colon);
        let domain = self.type_();
        self.consume(TokenKind::Semicolon);

        DeclKind::ProcedureSignature { name, domain: Box::new(domain) }
    }

    fn callable_definition(&mut self) -> DeclKind {
        self.consume(TokenKind::Def)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();

        // parameters
        let mut parameters = Vec::new();
        if self.try_consume(TokenKind::LeftParen).is_some() {
            if self.peek().token_kind == TokenKind::Identifier {
                parameters.push(Parameter {
                    name: self.consume(TokenKind::Identifier)?.lexeme.unwrap(),
                    default: None // TODO: handle default function parameters in the future
                });
            }

            while self.try_consume(TokenKind::Comma).is_some() {
                parameters.push(Parameter {
                    name: self.consume(TokenKind::Identifier)?.lexeme.unwrap(),
                    default: None // TODO: handle default function parameters in the future
                });
            }

            self.consume(TokenKind::RightParen)?;
        };

        let body = self.block()?;

        Ok(DeclKind::Callable { name, parameters, body })
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
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.token_kind {
            TokenKind::Let => self.local_variable_decl(),
            TokenKind::If => self.if_(),
            TokenKind::Return => self.return_(),
            TokenKind::Leave => self.leave(),
            TokenKind::Identifier => self.procedure_call(),
            
            _ => return Err(ParseError::UnexpectedTokenOneOf {
                expected: token_kinds!(Let | If | Return | Leave),
                found: token
            }),
        }?;

        let end = self.look(-1).unwrap().span.end;

        Ok(Stmt { kind, span: Span { start, end } })
    }

    fn local_variable_decl(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::Let)?;
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();
        self.consume(TokenKind::Assign)?;
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon)?;

        Ok(StmtKind::LocalVariable { name, value: Box::new(value) })
    }

    fn if_(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::If)?;
        let cond = Box::new(self.expression()?);
        let if_body = self.block()?;

        if self.try_consume(TokenKind::Else).is_some() {
            let token = self.peek();
            let else_body = match token.token_kind {
                TokenKind::If => self.node_statement(Self::if_),
                TokenKind::LeftBrace => self.node_statement(|parser| Ok(StmtKind::Block(parser.block()?))),

                _ => Err(ParseError::UnexpectedTokenOneOf { expected: token_kinds!(If | LeftBrace), found: token }),
            }?;

            return Ok(StmtKind::If { cond, if_body, else_body: Some(Box::new(else_body)) });
        };

        Ok(StmtKind::If { cond, if_body, else_body: None })
    }

    fn return_(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::Return)?;
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon)?;

        Ok(StmtKind::Return { value: Box::new(value) })
    }

    fn leave(&mut self) -> Result<StmtKind, ParseError> {
        self.consume(TokenKind::Leave)?;
        let value = self.expression()?;
        self.consume(TokenKind::Semicolon)?;

        Ok(StmtKind::Leave { value: Box::new(value) })
    }

    fn procedure_call(&mut self) -> Result<StmtKind, ParseError> {
        let name = self.consume(TokenKind::Identifier)?.lexeme.unwrap();

        self.consume(TokenKind::LeftParen)?;
        let mut arguments = Vec::new();

        arguments.push(self.expression()?);
        while self.try_consume(TokenKind::Comma).is_some() {
            arguments.push(self.expression()?);
        }

        self.consume(TokenKind::RightParen)?;
        self.consume(TokenKind::Semicolon)?;

        Ok(StmtKind::ProcedureCall { name, arguments })
    }

    fn block(&mut self) -> Result<Block, ParseError> {
        self.consume(TokenKind::LeftBrace)?;

        let mut nodes = Vec::new();
        while self.peek().token_kind != TokenKind::RightBrace {
            nodes.push(self.statement()?)
        };

        self.consume(TokenKind::RightBrace)?;

        Ok(Block { statements: nodes })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.token_kind {
            TokenKind::When => self.when(),
            
            _ => return self.primary_expression(0),
        }?;

        let end = self.look(-1).unwrap().span.end;

        Ok(Expr { kind, span: Span { start, end } })
    }

    fn when(&mut self) -> Result<ExprKind, ParseError> {
        self.consume(TokenKind::When)?;
        let cond = Box::new(self.expression()?);
        let when_body = self.block()?;

        if self.try_consume(TokenKind::Else).is_some() {            
            let token = self.peek();
            let else_body = match token.token_kind {
                TokenKind::When => self.node_expression(Self::when),
                TokenKind::LeftBrace => self.node_expression(|parser| Ok(ExprKind::Block(parser.block()?))),

                _ => Err(ParseError::UnexpectedTokenOneOf { expected: token_kinds!(If | LeftBrace), found: token }),
            }?;

            return Ok(ExprKind::When { cond, when_body, else_body: Some(Box::new(else_body)) });
        };

        Ok(ExprKind::When { cond, when_body, else_body: None })
    }

    fn primary_expression(&mut self, min_bp: BpSize) -> Result<Expr, ParseError> {
        let mut lhs = self.prefix()?;
        
        loop {
            let Some((op, lbp, rbp)) = self.get_infix_op(self.peek().token_kind) else {
                break;
            };

            if lbp < min_bp {
                break;
            }
            
            self.next();

            let rhs = self.primary_expression(rbp)?;

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

            let operand = self.primary_expression(bp)?;

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

                Ok(ExprKind::Number(token.lexeme.unwrap()))
            },

            TokenKind::String => {
                self.consume(TokenKind::String)?;

                Ok(ExprKind::String(token.lexeme.unwrap()))
            }

            TokenKind::Identifier => {
                self.consume(TokenKind::Identifier)?;
                if self.try_consume(TokenKind::LeftParen).is_some() { // function call
                    let mut arguments = Vec::new();
                    if self.peek().token_kind != TokenKind::RightParen {
                        arguments.push(self.expression()?);
                        while self.try_consume(TokenKind::Comma).is_some() {
                            arguments.push(self.expression()?);
                        }
                    }

                    self.consume(TokenKind::RightParen)?;

                    Ok(ExprKind::FunctionCall { name: token.lexeme.unwrap(), arguments })
                } else {
                    Ok(ExprKind::Identifier(token.lexeme.unwrap()))
                }
            },
            
            _ => Err(ParseError::UnexpectedTokenOneOf { expected: token_kinds!(Number | String | Identifier), found: token }),
        }?;

        Ok(Expr {
            kind,
            span: Span { start, end: self.look(-1)?.span.end },
        })
    }
}