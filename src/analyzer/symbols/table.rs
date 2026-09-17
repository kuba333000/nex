use std::collections::HashMap;

use crate::analyzer::symbols::{Symbol, SymbolId, SymbolKind, Scope, ScopeId};
use crate::span::Span;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,

    pub global_types: HashMap<String, SymbolId>,
    pub global_callables: HashMap<String, SymbolId>,

    pub scopes: Vec<Scope>,
    pub current_scope_id: ScopeId,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),

            global_types: HashMap::new(),
            global_callables: HashMap::new(),

            scopes: vec![Scope::new(None)],
            current_scope_id: ScopeId(0),
        }
    }

    fn new_symbol_id(&self) -> SymbolId {
        SymbolId(self.symbols.len())
    }

    fn new_scope_id(&self) -> ScopeId {
        ScopeId(self.scopes.len())
    }

    pub fn get_current_scope(&self) -> &Scope {
        self.scopes.get(self.current_scope_id.0).unwrap()
    }

    pub fn get_current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.get_mut(self.current_scope_id.0).unwrap()
    }

    pub fn enter_scope(&mut self) {
        let id = self.new_scope_id();
        self.scopes.push(Scope::new(Some(self.current_scope_id)));
        self.current_scope_id = id;
    }

    pub fn exit_scope(&mut self) {
        if self.get_current_scope().parent.is_none() {
            panic!("Global scope cannot be closed!")
        }

        self.current_scope_id = self.get_current_scope().parent.unwrap();
    }

    pub fn insert(&mut self, name: String, kind: SymbolKind, span: Span) -> SymbolId {
        let id = self.new_symbol_id();
        self.symbols.push(Symbol { name, kind, span: span });
        id
    }

    pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut Symbol> {
        self.symbols.get_mut(id.0)
    }
}