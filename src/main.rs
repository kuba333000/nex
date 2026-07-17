mod span;
mod diagnostic;

mod lexer;
mod parser;
mod compiler;

use compiler::{SourceFile, Compiler};

fn main() {
    let example_path = String::from("./examples/fibonacci.nex");

    let source_code = std::fs::read_to_string(example_path.clone()).unwrap();
    let source = SourceFile {
        id: 0,
        path: example_path,
        content: source_code
    };

    let compiler = Compiler::new(source);

    compiler.compile();
}