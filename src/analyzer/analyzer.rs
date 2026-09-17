use crate::parser::ast::Module;
use crate::diagnostics::DiagnosticSink;

use crate::analyzer::symbols::SymbolTable;

use crate::analyzer::name_resolver::NameResolver;

pub struct SemanticAnalyzer<'a> {
    module: &'a Module,
    diagnostic_sink: &'a mut DiagnosticSink,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(module: &'a Module, diagnostic_sink: &'a mut DiagnosticSink) -> Self {
        Self {
            module,
            diagnostic_sink,
        }
    }

    pub fn analyze(self) {
        let mut symbol_table = SymbolTable::new();

        let name_resolver = NameResolver::new(self.module, &mut symbol_table, self.diagnostic_sink);
        name_resolver.resolve();

        std::fs::write("dump/analyzer.symbols.dump", format!("{:#?}", symbol_table.symbols)).unwrap();
        std::fs::write("dump/analyzer.scopes.dump", format!("{:#?}", symbol_table.scopes)).unwrap();
    }
}