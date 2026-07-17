use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
}