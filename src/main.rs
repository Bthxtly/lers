mod lexer;
mod parser;

use parser::Parser;
use std::fs;

fn main() {
    read_and_parse_and_write();
}

fn read_and_parse_and_write() {
    let source = fs::read_to_string("analyzer.l").expect("Unable to read file");
    let parser = Parser::new(&source);
    let mut target_code = String::new();
    for code in parser {
        target_code.push_str(&code);
        target_code.push('\n');
    }
    fs::write("lers.yy.c", target_code).expect("Unable to write file");
}
