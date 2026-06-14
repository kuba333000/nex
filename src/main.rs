mod lexer;

use lexer::lexer::Lexer;
use lexer::tokens::Token;

fn main() {
    let source_code = "a_ 6. 21. 738. 913. 3233.21939 512. . 3.6 . . ."; // example source code
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