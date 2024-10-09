mod codegen;
mod lexer;
mod parser;

use std::fs;

fn main() {
    // let path = "./data/general/lexer_test.txt".to_string();
    let path = "./data/code/varible.frac".to_string();
    let source = fs::read_to_string(&path).expect("Failed to read file");

    let mut lexer = lexer::Lexer::new(path, &source);
    let mut parser = parser::Parser::new(&mut lexer);
    let mut codegen = codegen::Codegen::new();

    for token in lexer {
        println!("{}", token);
    }

    println!("{:#?}", parser);

    codegen.generate(&mut parser);
    codegen.save("./out/c/main.c");
}
