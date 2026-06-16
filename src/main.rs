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

display factorial(x);";

    println!("{}", source_code);

    let mut lexer = Lexer::new(source_code);

    println!("{:?}", lexer.get_tokens());
}