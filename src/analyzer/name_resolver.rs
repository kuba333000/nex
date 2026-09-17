use crate::diagnostics::*;
use crate::parser::ast::*;
use crate::analyzer::symbols::*;
use crate::span::Span;

pub struct NameResolver<'a> {
    module: &'a Module,
    symbol_table: &'a mut SymbolTable,
    diagnostic_sink: &'a mut DiagnosticSink,
}

impl<'a> NameResolver<'a> {
    pub fn new(module: &'a Module, symbol_table: &'a mut SymbolTable, diagnostic_sink: &'a mut DiagnosticSink) -> Self {
        Self {
            module,
            symbol_table,
            diagnostic_sink,
        }
    }

    fn get_symbol_mut(&mut self, id: SymbolId) -> Option<&mut Symbol> {
        self.symbol_table.get_mut(id)
    }

    fn declare(&mut self, ident: &Ident, kind: SymbolKind) -> SymbolId {
        let id = self.symbol_table.insert(ident.name.clone(), kind, ident.span);

        if self.symbol_table.get_current_scope().contains(&ident.name) {
            self.diagnostic_sink.error(ident.span, format!("duplicate declaration of '{}'", ident.name));
            return id;
        }
        
        self.symbol_table.get_current_scope_mut().insert(ident.name.clone(), id);

        id
    }

    fn declare_callable_signature(&mut self, ident: &Ident, kind: CallableKind, span: Span) -> SymbolId {
        let Some(id) = self.resolve_value(&ident.name) else {
            // new callable object
            
            let id = self.declare(ident, SymbolKind::Callable(CallableObj {
                signature_span: Some(span),
                definition_span: None,
                kind: Some(kind)
            }));

            return id;
        };

        let symbol = self.get_symbol_mut(id).unwrap();

        let SymbolKind::Callable(obj) = &mut symbol.kind else {
            // found non-callable with same name

            self.diagnostic_sink.error(
                ident.span,
                format!("duplicate declaration of '{}'", ident.name),
            );

            return id;
        };

        if obj.signature_span.is_some() {
            // duplicate signature

            self.diagnostic_sink.error(
                ident.span,
                format!("duplicate signature with name '{}'", ident.name),
            );

            return id;
        }

        obj.check_invariants();

        obj.signature_span = Some(span);
        obj.kind = Some(kind);

        id
    }

    fn resolve_value(&self, name: &str) -> Option<SymbolId> {
        self.symbol_table.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.values.get(name).copied())
    }

    fn declare_builtins(&mut self) {
        self.declare(
            &Ident { name: String::from("Int"), span: Span::SYNTHETIC },
            SymbolKind::Type
        );

        self.declare(
            &Ident { name: String::from("display"), span: Span::SYNTHETIC },
            SymbolKind::Callable(CallableObj::SYNTHETIC_FUNCTION)
        );
    }

    pub fn resolve(mut self) {
        self.collect_globals();

        self.build_hir();
    }

    fn collect_globals(&mut self) {
        self.declare_builtins(); // TODO: replace with a standard library prelude

        for decl in &self.module.0 {
            self.register(decl);
        }
    }

    fn build_hir(mut self) {
        for decl in &self.module.0 {
            self.visit_decl(decl);
        }
    }

    fn register(&mut self, decl: &Decl) {
        match &decl.kind {
            DeclKind::GlobalVariable(var_decl) => {
                self.declare(&var_decl.ident, SymbolKind::Value);
            },

            DeclKind::TypeAlias { ident, .. } => {
                self.declare(ident, SymbolKind::Type);
            },

            DeclKind::FunctionSignature { ident, .. } => {
                self.declare_callable_signature(ident, CallableKind::Function, decl.span);
            },

            DeclKind::ProcedureSignature { ident, .. } => {
                self.declare_callable_signature(ident, CallableKind::Procedure, decl.span);
            },

            DeclKind::CallableDefinition { ident, .. } => {
                let Some(id) = self.resolve_value(&ident.name) else {
                    // new callable object
                    
                    self.declare(ident, SymbolKind::Callable(CallableObj {
                        signature_span: None, 
                        definition_span: Some(decl.span), 
                        kind: None,
                    }));

                    return;
                };

                let symbol = self.get_symbol_mut(id).unwrap();

                let SymbolKind::Callable(obj) = &mut symbol.kind else {
                    // found non-callable with same name

                    self.diagnostic_sink.error(
                        ident.span,
                        format!("duplicate declaration of '{}'", ident.name),
                    );
                    
                    return;
                };

                if obj.definition_span.is_some() {
                    // duplicate definition

                    self.diagnostic_sink.error(
                        ident.span,
                        format!("duplicate definition with name '{}'", ident.name),
                    );

                    return;
                }

                obj.check_invariants();

                obj.definition_span = Some(decl.span);
            },

            DeclKind::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state ({})", decl.span),
        }
    }

    fn visit_decl(&mut self, decl: &Decl) {
        match &decl.kind {
            DeclKind::GlobalVariable(var_decl) => {
                self.visit_expr(&var_decl.value);
            }

            DeclKind::TypeAlias { value, .. } => {
                self.visit_type(value);
            }

            DeclKind::FunctionSignature { domain, codomain, .. } => {
                self.visit_type(domain);
                self.visit_type(codomain);
            }

            DeclKind::ProcedureSignature { domain, .. } => {
                self.visit_type(domain);
            }

            DeclKind::CallableDefinition { parameters, body, .. } => {
                self.visit_block_params(body, parameters);
            }

            DeclKind::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state ({})", decl.span),
        }
    }

    fn visit_conditional(&mut self, conditional: &Conditional) {
        self.visit_expr(&conditional.cond);
        self.visit_block(&conditional.body);

        if let Some(else_body) = &conditional.else_body {
            match else_body {
                ElseClause::Conditional(else_conditional)
                    => self.visit_conditional(else_conditional),
                
                ElseClause::Block(block)
                    => self.visit_block(block),
                
                ElseClause::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state"),
            }
        }
    }

    fn visit_block(&mut self, block: &Block) {
        self.symbol_table.enter_scope();

        for stmt in &block.0 {
            self.visit_stmt(stmt);
        }

        self.symbol_table.exit_scope();
    }

    fn visit_block_params(&mut self, block: &Block, parameters: &ParameterList) {
        self.symbol_table.enter_scope();

        for param in parameters {
            if param.default.is_some() { todo!("Implement default parameters!") }

            self.declare(&param.ident, SymbolKind::Value);
        }

        for stmt in &block.0 {
            self.visit_stmt(stmt);
        }

        self.symbol_table.exit_scope();
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::LocalVariable(var_decl) => {
                self.declare(&var_decl.ident, SymbolKind::Value);
                self.visit_expr(&var_decl.value);
            },

            StmtKind::ReturnValue { value } => {
                self.visit_expr(&value);
            },

            StmtKind::LeaveValue { value } => {
                self.visit_expr(&value);
            },

            StmtKind::If(conditional) => self.visit_conditional(conditional),

            StmtKind::Call(call) => {
                for arg in &call.arguments {
                    self.visit_expr(arg);
                }
            },

            StmtKind::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state ({})", stmt.span),

            _ => {},
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Identifier(identifier) => {
                if self.resolve_value(identifier).is_none() {
                    self.diagnostic_sink.error(expr.span, format!("Use of undefined variable '{}'", identifier));
                }
            },

            ExprKind::Unary { op, expr } => {
                self.visit_expr(expr);
            },

            ExprKind::Binary { op, left, right } => {
                self.visit_expr(left);
                self.visit_expr(right);
            },

            ExprKind::When(conditional) => self.visit_conditional(conditional),

            ExprKind::Call(call) => {
                for arg in &call.arguments {
                    self.visit_expr(arg);
                }
            },

            ExprKind::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state ({})", expr.span),

            _ => {}
        }
    }

    fn visit_type(&mut self, type_: &Type) {
        match &type_.kind {
            TypeKind::Named(name) => {
                if self.resolve_value(&name).is_none() {
                    self.diagnostic_sink.error(type_.span, format!("Use of undefined type '{}'", name));
                }
            },

            TypeKind::Tuple(types) => {
                for ty in types {
                    self.visit_type(ty);
                }
            },

            TypeKind::Error => panic!("Nex Internal Compiler Error: failed to terminate after error state ({})", type_.span),
        }
    }
}