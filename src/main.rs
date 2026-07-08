mod span;
mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

static TOKEN_PRINT_TYPE: &str = "simple";
static NODE_PRINT_TYPE: &str = "simple";

fn main() {
    // example source code for a Fibonacci function
    let source_code = "let x = 10;

func factorial : Int -> Int;
def factorial(n) := {
    return 1 if n <= 1 else n * factorial(n-1);
}

proc main : ();
def main {
    display \"10! = \" & factorial(x);

    display \"This
    is a
    multi-line
    string\"
}";

    // let source_code = "1 + 2 * 3"; // expression based source code

    let mut lexer = Lexer::new(source_code);

    let tokens = lexer.get_tokens();
    if TOKEN_PRINT_TYPE == "simple" {
        let s = tokens
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        
        println!("{s}");
    } else if TOKEN_PRINT_TYPE == "full" {
        println!("{:?}", tokens);
    } else if TOKEN_PRINT_TYPE == "pretty" {
        println!("{:#?}", tokens);
    } else {
        assert_eq!(TOKEN_PRINT_TYPE, "none");
    }

    let mut parser = Parser::new(tokens);

    let parser_result = parser.parse();
    let nodes = parser_result.unwrap();

    if NODE_PRINT_TYPE == "simple" {
        let s = nodes
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        
        println!("{s}");
    } else if NODE_PRINT_TYPE == "full" {
        println!("{:?}", nodes);
    } else if TOKEN_PRINT_TYPE == "pretty" {
        println!("{:#?}", nodes);
    } else {
        assert_eq!(NODE_PRINT_TYPE, "none");
    }
}