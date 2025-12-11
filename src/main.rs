mod lexer;

use lexer::Lexer;
use std::fs;

fn main() {
    let source = fs::read_to_string("analyzer.l").expect("Unable to read file");
    let lexer = Lexer::new(&source);
    for token in lexer {
        println!("{:?}", token.unwrap());
    }
}
