use std::fmt;

use crate::ansi_codes::*;
use crate::span::Span;

use crate::source_file::SourceFile;

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

#[derive(Debug, Clone)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
    span: Span,
}

impl Diagnostic {
    pub fn new(severity: Severity, message: String, span: Span) -> Self { Self { severity, message, span } }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.severity, self.message, self.span)
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticSink {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSink {
    pub fn new() -> Self { Self { diagnostics: Vec::new() } }

    pub fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn error(&mut self, span: Span, message: String) {
        self.diagnostics.push(Diagnostic::new(Severity::Error, message, span));
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn size(&self) -> usize {
        self.diagnostics.len()
    }
}

pub struct DiagnosticRenderer;

impl DiagnosticRenderer {
    pub fn render(diagnostic_sink: &DiagnosticSink, mut source: SourceFile) {
        for diagnostic in diagnostic_sink.diagnostics.iter() {
            let (start_line, start_column) = source.pos_from_char(diagnostic.span.start);
            let (end_line, end_column) = source.pos_from_char(diagnostic.span.end);

            let lines = source.get_lines(start_line, end_line);

            let max = *vec![start_line + 1, end_line + 1].iter().max().unwrap();
            let padding = max.ilog10() as usize + 1;

            let indent = lines
                .iter()
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.chars().take_while(|c| *c == ' ').count())
                .min()
                .unwrap_or(0);
            
            let lines_dedented: Vec<&str> = lines
                .into_iter()
                .map(|s| &s[indent.min(s.len())..])
                .collect();

            let mut error_source_code = lines_dedented
                .iter()
                .enumerate()
                .map(|(i, line)| format!("{BOLD_CYAN}{:<padding$} |{RESET} {}", start_line + i + 1, line))
                .collect::<Vec<_>>()
                .join("\n");
            
            if start_line == end_line {
                error_source_code.push_str(&format!(
                    "\n{} {BOLD_CYAN}|{RESET} {}{BOLD_RED}{}{RESET}",
                    " ".repeat(padding),
                    " ".repeat(start_column - indent),
                    "^".repeat(end_column - start_column)
                ));
            }

            eprintln!("{}{BOLD}: {}{RESET} {BOLD_CYAN}->{RESET} {}:{}:{}\n{} {BOLD_CYAN}|{RESET}\n{}\n",
                diagnostic.severity,
                diagnostic.message,
                
                source.path,
                start_line + 1,
                start_column + 1,

                " ".repeat(padding),
                error_source_code
            );
        }

        eprintln!("{BOLD_RED}Nex compiler error{RESET}{BOLD}: aborting due to {} previous error(s){RESET}\n", diagnostic_sink.size());
    }
}