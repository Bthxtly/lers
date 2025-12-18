mod ast;
mod codegen;
mod lexer;
mod parser;

use codegen::CodeGen;
use parser::Parser;
use std::fs;

fn main() {
    read_and_parse_and_write();
}

fn read_and_parse_and_write() {
    let source = fs::read_to_string("analyzer.l").expect("Unable to read file");
    let mut parser = Parser::new(&source);
    let ast = parser.parse();
    let mut codegen = CodeGen::new(ast);
    let target_code = codegen.generate();
    fs::write("lers.yy.c", target_code).expect("Unable to write file");
}
