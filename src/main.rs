use std::fs;

mod lexer;

fn main() {
    let path = "./data/lexer_test.txt".to_string();
    let source = fs::read_to_string(&path).expect("Failed to read file");

    let lexer = lexer::Lexer::new(path, &source);

    for token in lexer {
        println!("{}", token);
    }
}
