use crate::analyzer::symbols::SymbolId;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    pub values: HashMap<String, SymbolId>,
    pub parent: Option<ScopeId>,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>) -> Self { Self { values: HashMap::new(), parent } }

    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn insert(&mut self, name: String, id: SymbolId) {
        self.values.insert(name, id);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub usize);