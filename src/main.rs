mod lexer;

use lexer::lexer::Lexer;

fn main() {
    // example source code for a Fibonacci function
    let source_code = "let x = 10;

func factorial : int -> int;

def factorial(n) := {
    if n <= 1: return 1;
    return n * factorial(n-1);
}

display \"10! = \" & factorial(x);

display \"This
is a
multi-line
string\"";

    // let source_code = "->";

    let mut lexer = Lexer::new(source_code);

    let s = lexer
        .get_tokens()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    println!("{s}");
}