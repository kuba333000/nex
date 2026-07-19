mod span;
mod diagnostic;

mod lexer;
mod parser;
mod compiler;
mod ansi_codes;

use compiler::{SourceFile, Compiler};

fn main() {
    let example_path = String::from("./examples/fibonacci.nex");

    let source_code = std::fs::read_to_string(example_path.clone()).unwrap();
    let source = SourceFile::new(0, example_path, source_code);

    let mut compiler = Compiler::new(source);

    compiler.compile();
}