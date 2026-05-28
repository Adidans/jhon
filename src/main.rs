use jhon::Token;
use logos::Logos;

fn main() {
    let mut lexer = Token::lexer("{}}");
    for token in lexer {
        match token {
            Ok(t) => println!("{t:?}"),
            Err(e) => eprintln!("Error lexing"),
        }
    }
}
