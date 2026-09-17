use crate::span::Span;

use crate::analyzer::symbols::SymbolId;

#[derive(Debug, Clone)]
pub struct Hir {
    decls: Vec<HirDecl>
}

#[derive(Debug, Clone)]
pub struct HirDecl {
    pub symbol: SymbolId,
    pub kind: HirDeclKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum HirDeclKind {
    BindValue(HirExprId),
    BindType(SymbolId),

    BindFunction {
        domain: Vec<SymbolId>,
        codomain: SymbolId,
        effects: SymbolId,
        signature_span: Option<Span>,
        definition_span: Option<Span>,
        body: HirBlockId,
    },

    BindProcedure {
        domain: SymbolId,
        effects: SymbolId,
        signature_span: Option<Span>,
        definition_span: Option<Span>,
        body: HirBlockId,
    },
}