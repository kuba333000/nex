// use crate::span::Span;
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
}

#[derive(Debug, Clone)]
pub struct Compiler {
    source: SourceFile,
}

impl Compiler {
    pub fn new(source: SourceFile) -> Self { Self { source } }

    pub fn compile(&self) {
        let tokens = self.lex(&self.source.content);

        let declarations = self.parse(tokens);

        println!("{:#?}", declarations);
    }

    pub fn lex(&self, input: &str) -> Vec<Token> { Lexer::new(input).get_tokens() }

    pub fn parse(&self, token_vec: Vec<Token>) -> Vec<Decl> { Parser::new(token_vec).parse() }
}