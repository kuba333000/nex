use crate::source_file::SourceFile;
use crate::diagnostics::{DiagnosticSink, DiagnosticRenderer};

use crate::lexer;
use crate::parser;
use crate::analyzer;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use analyzer::analyzer::SemanticAnalyzer;

pub struct Compiler<'a> {
    source: &'a SourceFile,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a SourceFile) -> Self {
        Self { source }
    }

    pub fn compile(&mut self) {
        let mut diagnostic_sink = DiagnosticSink::new();

        let lexer = Lexer::new(&self.source.content, &mut diagnostic_sink);
        let tokens = lexer.get_tokens();

        std::fs::write("dump/lexer.tokens.dump", format!("{:#?}", tokens)).unwrap();

        let parser = Parser::new(tokens.clone(), &mut diagnostic_sink);
        let ast = parser.parse_module();

        std::fs::write("dump/parser.ast.dump", format!("{:#?}", ast)).unwrap();

        if !diagnostic_sink.is_empty() {
            DiagnosticRenderer::render(&diagnostic_sink, self.source.clone());
            std::process::exit(1);
        }

        let analyzer = SemanticAnalyzer::new(&ast, &mut diagnostic_sink);
        let annotated_ast = analyzer.analyze();
    }
}