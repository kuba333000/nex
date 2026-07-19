use std::fmt;

use crate::ansi_codes::*;
use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "{BOLD_RED}error{RESET}"),
            Severity::Warning => write!(f, "{BOLD_YELLOW}warning{RESET}"),
            Severity::Note => write!(f, "{BOLD_CYAN}note{RESET}"),
            Severity::Help => write!(f, "{BOLD_GREEN}help{RESET}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub span: Span,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.severity, self.message, self.span)
    }
}