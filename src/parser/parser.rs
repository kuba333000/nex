use crate::span::Span;
use crate::diagnostic::{Severity, Diagnostic};

use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::nodes::{Block, Parameter, UnaryOp, BinaryOp, ExprKind, StmtKind, DeclKind, TypeKind, Expr, Stmt, Decl, Type};

type BpSize = u8;

macro_rules! matches_token_kind {
    ($expr:expr, $($kind:ident)|+ $(,)?) => {
        matches!($expr, $(TokenKind::$kind)|+)
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    pub diagnostics: Vec<Diagnostic>,
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

    fn synchronize(&mut self) {
        self.next();

        while self.peek().kind != TokenKind::EndOfFile {
            if self.look(-1).kind == TokenKind::Semicolon { return; };

            if matches_token_kind!(self.peek().kind, Let | Func | Proc | Def | If | Return | Leave | When) { return; };

            self.next();
        }
    }
        
    fn consume(&mut self, token_kind: TokenKind) -> Token {
        let token = self.peek();
        if token.kind == token_kind {
            self.pos += 1;
            token
        } else {
            self.error(token.span, format!("expected '{}', found '{}'", token_kind, token.kind));
            Token { kind: TokenKind::Missing(Box::new(token_kind)), lexeme: None, span: token.span }
        }
    }

    fn node_statement<F>(&mut self, f: F) -> Stmt
    where
        F: FnOnce(&mut Self) -> StmtKind,
    {
        let start = self.peek().span.start;
        let kind = f(self);
        let end = self.look(-1).span.end;

        Stmt {
            kind,
            span: Span { start, end }
        }
    }

    fn node_expression<F>(&mut self, f: F) -> Expr
    where
        F: FnOnce(&mut Self) -> ExprKind,
    {
        let start = self.peek().span.start;
        let kind = f(self);
        let end = self.look(-1).span.end;

        Expr {
            kind,
            span: Span { start, end }
        }
    }

    fn try_consume(&mut self, token_kind: TokenKind) -> Option<Token> {
        let token = self.peek();
        if token.kind == token_kind {
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

    pub fn parse(&mut self) -> Vec<Decl> {
        self.pos = 0;

        let mut nodes = Vec::new();
        while self.peek().kind != TokenKind::EndOfFile {
            nodes.push(self.declaration() );
        }

        nodes
    }

    fn declaration(&mut self) -> Decl {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::Let => self.global_variable_decl(),
            TokenKind::Func => self.function_signature(),
            TokenKind::Proc => self.procedure_signature(),
            TokenKind::Def => self.callable_definition(),
            
            _ => {
                self.error(Span { start, end: token.span.end }, format!(
                    "expected declaration, found '{}'",
                    token.kind
                ));

                self.synchronize();
                DeclKind::Error
            },
        };

        let end = self.look(-1).span.end;

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
        self.consume(TokenKind::Def);
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();

        // parameters
        let mut parameters = Vec::new();
        if self.try_consume(TokenKind::LeftParen).is_some() {
            if self.peek().kind == TokenKind::Identifier {
                parameters.push(Parameter {
                    name: self.consume(TokenKind::Identifier).lexeme.unwrap(),
                    default: None // TODO: handle default function parameters in the future
                });
            }

            while self.try_consume(TokenKind::Comma).is_some() {
                parameters.push(Parameter {
                    name: self.consume(TokenKind::Identifier).lexeme.unwrap(),
                    default: None // TODO: handle default function parameters in the future
                });
            }

            self.consume(TokenKind::RightParen);
        };

        let body = self.block();

        DeclKind::Callable { name, parameters, body }
    }

    fn type_(&mut self) -> Type {
        let start = self.peek().span.start;

        if self.peek().kind == TokenKind::Identifier {
            let name = self.consume(TokenKind::Identifier);

            Type {
                kind: TypeKind::Named(name.lexeme.unwrap()),
                span: Span { start, end: self.look(-1).span.end },
            }
        } else {
            self.consume(TokenKind::LeftParen);
            if self.peek().kind == TokenKind::RightParen { // unit type ()
                self.consume(TokenKind::RightParen);
                
                return Type {
                    kind: TypeKind::Tuple(Vec::new()),
                    span: Span { start, end: self.look(-1).span.end },
                };
            }
            
            let mut types = Vec::new();

            // at least two-element tuple
            types.push(self.type_());
            self.consume(TokenKind::Comma);
            types.push(self.type_());

            while self.try_consume(TokenKind::Comma).is_some() {
                types.push(self.type_());
            }

            self.consume(TokenKind::RightParen);

            Type {
                kind: TypeKind::Tuple(types),
                span: Span { start, end: self.look(-1).span.end },
            }
        }
    }

    fn statement(&mut self) -> Stmt {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::Let => self.local_variable_decl(),
            TokenKind::If => self.if_(),
            TokenKind::Return => self.return_(),
            TokenKind::Leave => self.leave(),
            TokenKind::Identifier => self.procedure_call(),
            
            _ => {
                self.error(Span { start, end: token.span.end }, format!(
                    "expected statement, found '{}'",
                    token.kind
                ));

                self.synchronize();
                StmtKind::Error
            },
        };

        let end = self.look(-1).span.end;

        Stmt { kind, span: Span { start, end } }
    }

    fn local_variable_decl(&mut self) -> StmtKind {
        self.consume(TokenKind::Let);
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();
        self.consume(TokenKind::Assign);
        let value = self.expression();
        self.consume(TokenKind::Semicolon);

        StmtKind::LocalVariable { name, value: Box::new(value) }
    }

    fn if_(&mut self) -> StmtKind {
        self.consume(TokenKind::If);
        let cond = Box::new(self.expression());
        let if_body = self.block();

        if let Some(else_tok) = self.try_consume(TokenKind::Else) {
            let token = self.peek();
            let else_body = match token.kind {
                TokenKind::If => self.node_statement(Self::if_),
                TokenKind::LeftBrace => self.node_statement(|parser| StmtKind::Block(parser.block())),

                _ => {
                    self.error(Span { start: else_tok.span.start, end: token.span.end }, format!(
                        "expected 'else' to be followed by 'if' or '{{', found '{}'",
                        token.kind
                    ));

                    Stmt { kind: StmtKind::Error, span: token.span }
                }
            };

            return StmtKind::If { cond, if_body, else_body: Some(Box::new(else_body)) };
        };

        StmtKind::If { cond, if_body, else_body: None }
    }

    fn return_(&mut self) -> StmtKind {
        self.consume(TokenKind::Return);
        if self.try_consume(TokenKind::Semicolon).is_some() { return StmtKind::Return; }

        let value = self.expression();
        self.consume(TokenKind::Semicolon);

        StmtKind::ReturnValue { value: Box::new(value) }
    }

    fn leave(&mut self) -> StmtKind {
        self.consume(TokenKind::Leave);
        if self.try_consume(TokenKind::Semicolon).is_some() { return StmtKind::Leave; }

        let value = self.expression();
        self.consume(TokenKind::Semicolon);

        StmtKind::LeaveValue { value: Box::new(value) }
    }

    fn procedure_call(&mut self) -> StmtKind {
        let name = self.consume(TokenKind::Identifier).lexeme.unwrap();

        self.consume(TokenKind::LeftParen);
        let mut arguments = Vec::new();

        arguments.push(self.expression());
        while self.try_consume(TokenKind::Comma).is_some() {
            arguments.push(self.expression());
        }

        self.consume(TokenKind::RightParen);
        self.consume(TokenKind::Semicolon);

        StmtKind::ProcedureCall { name, arguments }
    }

    fn block(&mut self) -> Block {
        self.consume(TokenKind::LeftBrace);

        let mut nodes = Vec::new();
        while self.peek().kind != TokenKind::RightBrace {
            nodes.push(self.statement())
        };

        self.consume(TokenKind::RightBrace);

        Block { statements: nodes }
    }

    fn expression(&mut self) -> Expr {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::When => self.when(),
            
            _ => return self.primary_expression(0),
        };

        let end = self.look(-1).span.end;

        Expr { kind, span: Span { start, end } }
    }

    fn when(&mut self) -> ExprKind {
        self.consume(TokenKind::When);
        let cond = Box::new(self.expression());
        let when_body = self.block();

        if let Some(else_tok) = self.try_consume(TokenKind::Else) {            
            let token = self.peek();
            let else_body = match token.kind {
                TokenKind::When => self.node_expression(Self::when),
                TokenKind::LeftBrace => self.node_expression(|parser| ExprKind::Block(parser.block())),

                _ => {
                    self.error(Span { start: else_tok.span.start, end: token.span.end }, format!(
                        "expected 'else' to be followed by 'when' or '{{', found '{}'",
                        token.kind
                    ));

                    self.synchronize();
                    Expr { kind: ExprKind::Error, span: token.span }
                }
            };

            return ExprKind::When { cond, when_body, else_body: Some(Box::new(else_body)) };
        };

        ExprKind::When { cond, when_body, else_body: None }
    }

    fn primary_expression(&mut self, min_bp: BpSize) -> Expr {
        let mut lhs = self.prefix();
        
        loop {
            let Some((op, lbp, rbp)) = self.get_infix_op(self.peek().kind) else {
                break;
            };

            if lbp < min_bp {
                break;
            }
            
            self.next();

            let rhs = self.primary_expression(rbp);

            lhs = Expr {
                kind: ExprKind::Binary {
                    op,
                    left: Box::new(lhs.clone()),
                    right: Box::new(rhs.clone()),
                },
                span: Span::new_by_spans(lhs.span, rhs.span),
            };
        }

        lhs
    }

    fn prefix(&mut self) -> Expr {
        let token = self.peek();
        let kind = token.kind;

        if let Some((op, bp)) = self.get_prefix_op(kind.clone()) {
            let op_span = token.span;
            self.consume(kind);

            let operand = self.primary_expression(bp);

            let span = Span::new_by_spans(op_span, operand.span);
            return Expr {
                kind: ExprKind::Unary {
                    op,
                    expr: Box::new(operand),
                },
                span,
            };
        }

        self.literal()
    }

    fn literal(&mut self) -> Expr {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::Number => {
                self.consume(TokenKind::Number);

                ExprKind::Number(token.lexeme.unwrap())
            },

            TokenKind::String => {
                self.consume(TokenKind::String);

                ExprKind::String(token.lexeme.unwrap())
            }

            TokenKind::Identifier => {
                self.consume(TokenKind::Identifier);
                if self.try_consume(TokenKind::LeftParen).is_some() {
                    // function call
                    let mut arguments = Vec::new();
                    
                    if self.peek().kind != TokenKind::RightParen {
                        arguments.push(self.expression());
                        while self.try_consume(TokenKind::Comma).is_some() {
                            arguments.push(self.expression());
                        }
                    }

                    self.consume(TokenKind::RightParen);

                    ExprKind::FunctionCall { name: token.lexeme.unwrap(), arguments }
                } else {
                    ExprKind::Identifier(token.lexeme.unwrap())
                }
            },
            
            _ => {
                self.error(token.span, format!(
                    "expected literal, found '{}'",
                    token.kind
                ));

                ExprKind::Error
            }
        };

        Expr {
            kind,
            span: Span { start, end: self.look(-1).span.end },
        }
    }
}