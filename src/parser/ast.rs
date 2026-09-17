use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Module(pub Vec<Decl>);

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

#[derive(Debug, Clone)]
pub enum ExprKind {
    Number(String),
    String(String),
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

    When(Conditional),
    Call(Call),

    Error,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    LocalVariable(VariableDecl),

    Return,
    ReturnValue { value: Box<Expr>, },

    Leave,
    LeaveValue { value: Box<Expr>, },

    If(Conditional),
    Call(Call),

    Error,
}

#[derive(Debug, Clone)]
pub enum DeclKind {
    GlobalVariable(VariableDecl),

    TypeAlias {
        ident: Ident,
        value: Box<Type>,
    },

    FunctionSignature {
        ident: Ident,
        domain: Box<Type>,
        codomain: Box<Type>,
        effects: EffectList,
    },

    ProcedureSignature {
        ident: Ident,
        domain: Box<Type>,
        effects: EffectList,
    },

    CallableDefinition {
        ident: Ident,
        parameters: ParameterList,
        body: Block,
    },

    Error,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Named(String),
    Tuple(Vec<Type>),
    Error,
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
pub enum ElseClause {
    Conditional(Box<Conditional>),
    Block(Block),
    Error,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub cond: Box<Expr>,
    pub body: Block,
    pub else_body: Option<ElseClause>,
}

#[derive(Debug, Clone)]
pub struct Call  {
    pub callee: Box<Expr>,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub ident: Ident,
    pub ty: Option<Box<Type>>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub ident: Ident,
    pub default: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct ParameterList(pub Vec<Parameter>);

#[derive(Debug, Clone)]
pub struct Effect {
    pub ident: Ident,
}

#[derive(Debug, Clone)]
pub struct EffectList(pub Vec<Effect>);

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

impl ParameterList {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn push(&mut self, ident: Ident) {
        self.0.push(Parameter { ident, default: None });
    }

    pub fn push_default(&mut self, ident: Ident, default: Expr) {
        self.0.push(Parameter { ident, default: Some(Box::new(default)) });
    }
}

impl IntoIterator for ParameterList {
    type Item = Parameter;
    type IntoIter = std::vec::IntoIter<Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ParameterList {
    type Item = &'a Parameter;
    type IntoIter = std::slice::Iter<'a, Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut ParameterList {
    type Item = &'a mut Parameter;
    type IntoIter = std::slice::IterMut<'a, Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}