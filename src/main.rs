mod ast;
mod codegen;
mod lexer;
mod parser;

use codegen::CodeGen;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1) {
        read_and_parse_and_write(&filename);
    } else {
        read_and_parse_and_write("analyzer.l");
    }
}

fn read_and_parse_and_write(filename: &str) {
    let source = fs::read_to_string(filename).expect("Unable to read file");
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let mut codegen = CodeGen::new(ast);
    let target_code = codegen.generate();
    fs::write("lers.yy.c", target_code).expect("Unable to write file");
}
