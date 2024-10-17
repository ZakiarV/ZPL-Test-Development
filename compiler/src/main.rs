mod lexer;

use lexer::tokenize;

fn main() {
    let input = "let x = 5 + 5;";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}
