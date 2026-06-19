use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Equal,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    Concat,

    Plus,
    Minus,
    Mult,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Integer(i64),
    Float(i64),

    String(String),

    Identifier(String),

    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}