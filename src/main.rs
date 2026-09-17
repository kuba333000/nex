mod span;
mod ansi_codes;

mod diagnostics;

mod source_file;
mod lexer;
mod parser;
mod analyzer;
mod compiler;

use source_file::SourceFile;

use compiler::Compiler;

fn main() {
    let example_path = String::from("./examples/factorial.nex");

    let source_code = std::fs::read_to_string(example_path.clone()).unwrap();
    let source = SourceFile::new(0, example_path, source_code);

    let mut compiler = Compiler::new(&source);

    compiler.compile();
}