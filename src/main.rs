mod span;
mod lexer;
mod parser;

use lexer::lexer::Lexer;
use parser::parser::Parser;

static TOKEN_PRINT_TYPE: &str = "none";

fn main() {
    // example source code for a Fibonacci function
    let source_code = "let x = 10;

func factorial : int -> int;

def factorial(n) := {
    return 1 if n <= 1 else n * factorial(n-1);
}

display \"10! = \" & factorial(x);

display \"This
is a
multi-line
string\"";

    // let source_code = "\"test\"";

    let mut lexer = Lexer::new(source_code);

    if TOKEN_PRINT_TYPE == "simple" {
        let s = lexer
            .get_tokens()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        
        println!("{s}");
    } else if TOKEN_PRINT_TYPE == "full" {
        println!("{:?}", lexer.get_tokens());
    } else {
        assert_eq!(TOKEN_PRINT_TYPE, "none");
    }

    let mut parser = Parser::new(lexer.get_tokens());
}