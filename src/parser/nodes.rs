use std::fmt;

use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub default: Option<Box<Expr>>,
}

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
pub enum ExprKind {
    String(String),

    Integer(i64),
    Real(f64),

    Identifier(String),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Block(Block),
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    LocalVariable {
        name: String,
        value: Box<Expr>,
    },

    If {
        cond: Box<Expr>,
        if_body: Block,
        else_body: Option<Box<Stmt>>,
    },

    Block(Block),
}

#[derive(Debug, Clone)]
pub enum DeclKind {
    GlobalVariable {
        name: String,
        value: Box<Expr>,
    },

    FunctionSignature {
        name: String,
        domain: Box<Type>,
        codomain: Box<Type>,
    },

    ProcedureSignature {
        name: String,
        domain: Box<Type>,
    },

    Callable {
        name: String,
        paramaters: Vec<Parameter>,
        body: Block,
    },
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Named(String),
    Tuple(Vec<Type>),
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub kind: DeclKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.statements
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        
        write!(f, "{}", s)
    }
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
        match &self.kind {
            ExprKind::String(s) => write!(f, "\"{s}\""),
            ExprKind::Integer(i) => write!(f, "{i}"),
            ExprKind::Real(r) => write!(f, "{r}"),

            ExprKind::Identifier(name) => write!(f, "{name}"),

            ExprKind::Unary { op, expr } => {
                write!(f, "({}{})", op, expr)
            },

            ExprKind::Binary { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            },

            ExprKind::Block(block) => {
                write!(f, "{}", block)
            }
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            StmtKind::LocalVariable { name, value } => {
                write!(f, "LocalVariable({}, {})", name, value)
            },

            StmtKind::If { cond, if_body, else_body } => {
                match else_body {
                    Some(body) => write!(f, "If({}, {}, {})", cond, if_body, *body),
                    None => write!(f, "If({}, {})", cond, if_body),
                }
            },

            StmtKind::Block(block) => {
                write!(f, "{}", block)
            }
        }
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DeclKind::GlobalVariable { name, value } => {
                write!(f, "GlobalVariable({}, {})", name, value)
            },

            DeclKind::FunctionSignature { name, domain, codomain } => {
                write!(f, "FunctionSignature({}, {}, {})", name, domain, codomain)
            },

            DeclKind::ProcedureSignature { name, domain } => {
                write!(f, "FunctionSignature({}, {})", name, domain)
            },

            DeclKind::Callable { name, paramaters, body } => {
                write!(f, "CallableDefinition({}, {:?}, {})", name, paramaters, body)
            },
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TypeKind::Named(name) => {
                write!(f, "Type({name})", )
            },

            TypeKind::Tuple(names) => {
                write!(f, "TypeTuple({:?})", names)
            },
        }
    }
}