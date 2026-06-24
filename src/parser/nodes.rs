use std::fmt;

use crate::span::Span;

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Equal,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    Concat,

    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum AstNodeKind {
    Expr { kind: Expr },
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub category: AstNodeKind,
    pub span: Span,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            BinaryOp::Equal => "==",
            BinaryOp::NotEq => "!=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEq => ">=",
            BinaryOp::Less => "<",
            BinaryOp::LessEq => "<=",

            BinaryOp::Concat => "..",

            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
        };

        write!(f, "{op}")
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::StrType => write!(f, "string"),
            Expr::IntType => write!(f, "int"),

            Expr::String(s) => write!(f, "\"{s}\""),
            Expr::Integer(i) => write!(f, "{i}"),
            Expr::Real(r) => write!(f, "{r}"),

            Expr::Identifier(name) => write!(f, "{name}"),

            Expr::Unary { op, expr } => {
                write!(f, "({}{})", op, expr)
            }

            Expr::Binary { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.category {
            AstNodeKind::Expr { kind } => write!(f, "{kind}"),
        }
    }
}