mod lexer;

use lexer::lexer::Lexer;
use lexer::tokens::Token;

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

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();

        match token {
            Token::EndOfFile => {
                tokens.push(token);
                break;
            }
            Token::Invalid(c) => {
                panic!("unexpected character: {}", c);
            }
            _ => {}
        }

        tokens.push(token);
    }

    println!("{:?}", tokens);
}