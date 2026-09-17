use crate::span::Span;

use crate::diagnostics::*;
use crate::lexer::tokens::*;
use crate::parser::ast::*;

type BpSize = u8;

macro_rules! matches_token_kind {
    ($expr:expr, $($kind:ident)|+ $(,)?) => {
        matches!($expr, $(TokenKind::$kind)|+)
    };
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    diagnostic_sink: &'a mut DiagnosticSink,
}

impl<'a> Parser<'a> {
    pub fn new(token_vec: Vec<Token>, diagnostic_sink: &'a mut DiagnosticSink) -> Self {
        Self { tokens: token_vec, pos: 0, diagnostic_sink }
    }

    fn peek(&self) -> Token { self.tokens.get(self.pos).cloned().unwrap() }

    fn is_peek(&self, token_kind: TokenKind) -> bool {
        self.peek().kind == token_kind
    }

    fn look(&self, offset: isize) -> Token {
        let index = self
            .pos
            .checked_add_signed(offset)
            .unwrap_or(0);

        let index = index.min(self.tokens.len() - 1);

        self.tokens.get(index).cloned().unwrap()
    }

    fn next(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
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
            self.diagnostic_sink.error(token.span, format!("expected '{}', found '{}'", token_kind, token.kind));
            self.pos += 1;

            if token_kind == TokenKind::Semicolon {
                self.synchronize();
            };

            Token { kind: TokenKind::Missing(Box::new(token_kind)), lexeme: token.lexeme, span: token.span }
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

    pub fn parse_module(mut self) -> Module {
        let mut declarations = Vec::new();
        while self.peek().kind != TokenKind::EndOfFile {
            declarations.push(self.declaration());
        }

        Module(declarations)
    }

    fn identifier(&mut self) -> Ident {
        let token = self.consume(TokenKind::Identifier);

        let name = token.lexeme_or("<missing>");
        let span = token.span;
        
        Ident { name, span }
    }

    fn declaration(&mut self) -> Decl {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::Let => self.global_variable_decl(),
            TokenKind::Type => self.type_alias_decl(),
            TokenKind::Func => self.function_signature(),
            TokenKind::Proc => self.procedure_signature(),
            TokenKind::Def => self.callable_definition(),
            
            _ => {
                self.diagnostic_sink.error(token.span, format!(
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
        DeclKind::GlobalVariable(self.variable_decl())
    }

    fn type_alias_decl(&mut self) -> DeclKind {
        self.consume(TokenKind::Type);
        let ident = self.identifier();
        self.consume(TokenKind::Assign);
        let value = self.type_();
        self.consume(TokenKind::Semicolon);

        DeclKind::TypeAlias { ident, value: Box::new(value) }
    }

    fn function_signature(&mut self) -> DeclKind {
        self.consume(TokenKind::Func);
        let ident = self.identifier();

        let domain = if self.try_consume(TokenKind::Colon).is_some() { self.type_() } else { // implicit unit type
            let start = self.peek().span.start;

            Type {
                kind: TypeKind::Tuple(Vec::new()),
                span: Span { start, end: start } // zero-width
            }
        };

        self.consume(TokenKind::Arrow);
        let codomain = self.type_();

        let effects = if self.try_consume(TokenKind::Effects).is_some() {
            let mut effect_list = EffectList(Vec::new());
            
            effect_list.0.push(Effect { ident: self.identifier() });
            self.consume(TokenKind::Comma);

            while self.is_peek(TokenKind::Identifier) {
                effect_list.0.push(Effect { ident: self.identifier() });
                self.consume(TokenKind::Comma);
            }

            effect_list
        } else {
            EffectList(Vec::new())
        };
        self.consume(TokenKind::Semicolon);

        DeclKind::FunctionSignature { ident, domain: Box::new(domain), codomain: Box::new(codomain), effects }
    }

    fn procedure_signature(&mut self) -> DeclKind {
        self.consume(TokenKind::Proc);
        let ident = self.identifier();

        let domain = if self.try_consume(TokenKind::Colon).is_some() { self.type_() } else { // implicit unit type
            let start = self.peek().span.start;

            Type {
                kind: TypeKind::Tuple(Vec::new()),
                span: Span { start, end: start } // zero-width
            }
        };

        let effects = if self.try_consume(TokenKind::Effects).is_some() {
            let mut effect_list = EffectList(Vec::new());
            effect_list.0.push(Effect { ident: self.identifier() });

            while self.is_peek(TokenKind::Comma) {
                self.consume(TokenKind::Comma);
                effect_list.0.push(Effect { ident: self.identifier() });
            }

            effect_list
        } else {
            EffectList(Vec::new())
        };

        self.consume(TokenKind::Semicolon);

        DeclKind::ProcedureSignature { ident, domain: Box::new(domain), effects }
    }

    fn callable_definition(&mut self) -> DeclKind {
        self.consume(TokenKind::Def);
        let ident = self.identifier();

        // parameters
        let mut parameters = ParameterList::new();
        if self.try_consume(TokenKind::LeftParen).is_some() {
            if self.peek().kind == TokenKind::Identifier {
                parameters.push(self.identifier());
                
                while self.try_consume(TokenKind::Comma).is_some() {
                    parameters.push(self.identifier());
                }
            }

            self.consume(TokenKind::RightParen);
        };

        let body = self.block();

        DeclKind::CallableDefinition { ident, parameters, body }
    }

    fn type_(&mut self) -> Type {
        let token = self.peek();
        let start = token.span.start;

        let kind = match token.kind {
            TokenKind::Identifier => self.named_type(),
            TokenKind::LeftParen => self.tuple_type(),

            _ => {
                self.diagnostic_sink.error(token.span, format!(
                    "expected type, found '{}'",
                    token.kind
                ));
                
                TypeKind::Error
            }
        };

        let end = self.look(-1).span.end;

        Type { kind, span: Span { start, end } }
    }

    fn named_type(&mut self) -> TypeKind {
        let name = self.consume(TokenKind::Identifier);

        TypeKind::Named(name.lexeme.unwrap())
    }

    fn tuple_type(&mut self) -> TypeKind {
        self.consume(TokenKind::LeftParen);

        if self.try_consume(TokenKind::RightParen).is_some() {
            return TypeKind::Tuple(Vec::new()); // unit type ()
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

        TypeKind::Tuple(types)
    }

    fn variable_decl(&mut self) -> VariableDecl {
        self.consume(TokenKind::Let);
        let ident = self.identifier();

        let ty = if self.try_consume(TokenKind::Colon).is_some() {
            Some(Box::new(self.type_()))
        } else {
            None
        };

        self.consume(TokenKind::Assign);
        let value = self.expression();
        self.consume(TokenKind::Semicolon);

        VariableDecl { ident, ty, value: Box::new(value) }
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
                self.diagnostic_sink.error(token.span, format!(
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
        StmtKind::LocalVariable(self.variable_decl())
    }

    fn if_(&mut self) -> StmtKind {
        StmtKind::If(self.conditional(TokenKind::If))
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
        let token = self.consume(TokenKind::Identifier);
        let identifier = ExprKind::Identifier(token.lexeme.unwrap());

        self.consume(TokenKind::LeftParen);
        let mut arguments = Vec::new();

        arguments.push(self.expression());
        while self.try_consume(TokenKind::Comma).is_some() {
            arguments.push(self.expression());
        }

        self.consume(TokenKind::RightParen);
        self.consume(TokenKind::Semicolon);

        StmtKind::Call(Call { callee: Box::new(Expr { kind: identifier, span: token.span }), arguments })
    }

    fn conditional(&mut self, cond_kind: TokenKind) -> Conditional {
        if !matches_token_kind!(cond_kind, If | When) {
            panic!("Nex Internal Compiler Error: Conditional expects if/when keyword, found {}", cond_kind)
        };

        self.consume(cond_kind.clone());

        let cond = Box::new(self.expression());
        let body = self.block();

        if let Some(else_tok) = self.try_consume(TokenKind::Else) {
            let token = self.peek();
            let else_body = match token.kind {
                k if k == cond_kind => ElseClause::Conditional(
                    Box::new(self.conditional(cond_kind))
                ),

                TokenKind::LeftBrace => ElseClause::Block(self.block()),

                _ => {
                    self.diagnostic_sink.error(Span { start: else_tok.span.start, end: token.span.end }, format!(
                        "expected 'else' to be followed by '{}' or '{{', found '{}'",
                        cond_kind,
                        token.kind
                    ));

                    ElseClause::Error
                }
            };

            return Conditional { cond, body, else_body: Some(else_body) };
        };

        Conditional { cond, body, else_body: None }
    }

    fn block(&mut self) -> Block {
        self.consume(TokenKind::LeftBrace);

        let mut nodes = Vec::new();
        while self.peek().kind != TokenKind::RightBrace {
            nodes.push(self.statement())
        };

        self.consume(TokenKind::RightBrace);

        Block(nodes)
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
        ExprKind::When(self.conditional(TokenKind::When))
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
                let identifier = ExprKind::Identifier(token.lexeme.unwrap());

                if self.try_consume(TokenKind::LeftParen).is_some() { // function call
                    let mut arguments = Vec::new();
                    
                    if self.peek().kind != TokenKind::RightParen {
                        arguments.push(self.expression());
                        while self.try_consume(TokenKind::Comma).is_some() {
                            arguments.push(self.expression());
                        }
                    }

                    self.consume(TokenKind::RightParen);

                    ExprKind::Call(Call { callee: Box::new(Expr { kind: identifier, span: token.span }), arguments })
                } else {
                    identifier
                }
            },
            
            _ => {
                self.diagnostic_sink.error(token.span, format!(
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