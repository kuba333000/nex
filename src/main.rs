mod span;
mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

use crate::parser::parser::ParseError;

static TOKEN_PRINT_TYPE: &str = "simple";
static NODE_PRINT_TYPE: &str = "pretty";

fn main() {
    // example source code for a Fibonacci function
    let source_code = "let x = 10;

func factorial : Int -> Int;
def factorial(n) {
    return when n <= 1 { leave 1; } else { leave n * factorial(n-1); };
}

proc main : ();
def main {
    display(\"10! = \" & factorial(x));

    display(\"This
    is a
    multi-line
    string\");
}";

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
    if let Err(error) = parser_result {
        let error_message = match error {
            ParseError::UnexpectedToken { expected, found } => format!("Expected token kind \"{:?}\", found \"{:?}\" instead. At {:?}", expected, found.token_kind, found.span),
            ParseError::UnexpectedTokenOneOf { expected, found } => format!("Expected any token kind of \"{:?}\", found \"{:?}\" instead. At {:?}", expected, found.token_kind, found.span),
            
            _ => String::from("Parser error!"),
        };

        eprintln!("{error_message}");
        std::process::exit(1);
    }

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
    } else if NODE_PRINT_TYPE == "pretty" {
        println!("{:#?}", nodes);
    } else {
        assert_eq!(NODE_PRINT_TYPE, "none");
    }
}