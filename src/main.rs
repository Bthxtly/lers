mod lexer;
mod parser;

use parser::Parser;
use std::fs;

fn main() {
    read_and_parse_and_write();
}

fn read_and_parse_and_write() {
    let source = fs::read_to_string("analyzer.l").expect("Unable to read file");
    let mut parser = Parser::new(&source);
    let target_code = parser.gen_code();
    fs::write("lers.yy.c", target_code).expect("Unable to write file");
}
