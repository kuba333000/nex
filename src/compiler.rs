use crate::ansi_codes::*;

use crate::diagnostic::Diagnostic;

use crate::lexer;
use crate::parser;

use lexer::tokens::Token;
use lexer::lexer::Lexer;

use parser::nodes::Decl;
use parser::parser::Parser;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub id: usize,
    pub path: String,
    pub content: String,
    pub line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(id: usize, path: String, content: String) -> Self { Self { id, path, content, line_starts: Vec::new() } }

    pub fn fill_line_starts(&mut self) {
        self.line_starts = vec![0];

        self.line_starts.extend(
            self.content.bytes()
                .enumerate()
                .filter_map(|(i, b)| (b == b'\n').then_some(i + 1)),
        );
    }

    pub fn get_lines(&mut self, start_line_num: usize, end_line_num: usize) -> Vec<&str> {
        self.fill_line_starts();

        let mut lines = Vec::new();

        for line_num in start_line_num..=end_line_num {
            if line_num >= self.line_starts.len() { break; };

            let start = self.line_starts[line_num];
            let end = if line_num + 1 < self.line_starts.len() {
                self.line_starts[line_num + 1]
            } else {
                self.content.len()
            };

            lines.push(self.content[start..end].trim_end_matches('\n'));
        }

        lines
    }

    pub fn pos_from_char(&mut self, offset: usize) -> (usize, usize) {
        self.fill_line_starts();

        let line = match self.line_starts.binary_search(&offset) {
            Ok(line) => line,
            Err(line) => line - 1,
        };

        let column = offset - self.line_starts[line];

        (line, column)
    }
}

#[derive(Debug, Clone)]
pub struct Compiler {
    source: SourceFile,
    filename: String,
}

impl Compiler {
    pub fn new(source: SourceFile) -> Self { Self { source, filename: String::new() } }

    pub fn compile(&mut self) {
        self.filename = self.source.path.clone().split("/").last().unwrap().to_string();

        let tokens = self.lex(&self.source.content);

        let declarations = self.parse(tokens);

        println!("{:#?}", declarations);
    }
    
    pub fn print_diagnostics(&mut self, diagnostics: Vec<Diagnostic>) {
        for diagnostic in diagnostics.iter() {
            let (start_line, start_column) = self.source.pos_from_char(diagnostic.span.start);
            let (end_line, end_column) = self.source.pos_from_char(diagnostic.span.end);

            let lines = self.source.get_lines(start_line, end_line);

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
                .map(|(i, line)| format!("{BOLD_CYAN}{} |{RESET} {}", start_line + i + 1, line))
                .collect::<Vec<_>>()
                .join("\n");
            
            if start_line == end_line {
                error_source_code.push_str(&format!(
                    "\n   {BOLD_CYAN}|{RESET} {}{BOLD_RED}{}{RESET}",
                    " ".repeat(start_column - indent),
                    "^".repeat(end_column - start_column)
                ));
            }

            eprintln!("{}{BOLD}: {}{RESET} {BOLD_CYAN}->{RESET} {}:{}:{}\n   {BOLD_CYAN}|{RESET}\n{}\n",
            diagnostic.severity,
            diagnostic.message,
            self.filename,
            start_line + 1,
            start_column + 1,
            error_source_code
            );
        }

        eprintln!("{BOLD_RED}error{RESET}{BOLD}: aborting due to {} previous error(s){RESET}\n", diagnostics.len());
        
        std::process::exit(1);
    }

    pub fn lex(&self, input: &str) -> Vec<Token> { Lexer::new(input).get_tokens() }

    pub fn parse(&mut self, token_vec: Vec<Token>) -> Vec<Decl> {
        let mut parser = Parser::new(token_vec);
        let declarations = parser.parse();

        if !parser.diagnostics.is_empty() { self.print_diagnostics(parser.diagnostics); };

        declarations
    }
}