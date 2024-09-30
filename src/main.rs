mod lexer;
mod parser;

use std::fs;

fn main() {
    // let path = "./data/general/lexer_test.txt".to_string();
    let path = "./data/code/base.fcl".to_string();
    let source = fs::read_to_string(&path).expect("Failed to read file");

    let mut lexer = lexer::Lexer::new(path, &source);
    let mut parser = parser::Parser::new(&mut lexer);

    // for token in lexer {
    //     println!("{}", token);
    // }

    println!("{:#?}", parser);
}
