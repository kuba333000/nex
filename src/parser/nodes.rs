use crate::span::Span;

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expr {
    StrType,
    IntType,

    String(String),

    Integer(i64),
    Real(f64),

    Identifier(String),

    Unary {
        op: UnaryOp,
        expr: Box<AstNode>,
    },

    Binary {
        op: BinaryOp,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

#[derive(Debug)]
pub enum AstNodeKind {
    Expr { kind: Expr },
}

#[derive(Debug)]
pub struct AstNode {
    pub category: AstNodeKind,
    pub span: Span,
}