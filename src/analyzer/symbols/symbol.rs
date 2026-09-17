use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone)]
pub enum CallableKind {
    Function,
    Procedure,
}

#[derive(Debug, Clone)]
pub struct CallableObj {
    pub signature_span: Option<Span>,
    pub definition_span: Option<Span>,
    pub kind: Option<CallableKind>,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Value,
    Type,
    Callable(CallableObj),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
}

impl CallableObj {
    pub const SYNTHETIC_FUNCTION: CallableObj = CallableObj {
        signature_span: Some(Span::SYNTHETIC),
        definition_span: Some(Span::SYNTHETIC),
        kind: Some(CallableKind::Function),
    };

    pub const SYNTHETIC_PROCEDURE: CallableObj = CallableObj {
        signature_span: Some(Span::SYNTHETIC),
        definition_span: Some(Span::SYNTHETIC),
        kind: Some(CallableKind::Procedure),
    };

    #[cfg(debug_assertions)]
    pub fn check_invariants(&self) {
        debug_assert_eq!(
            self.signature_span.is_some(),
            self.kind.is_some(),
        );
    }
}