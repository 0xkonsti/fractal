use crate::lexer::TokenType;

pub fn generate_op(op: &TokenType) -> &str {
    match op {
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Asterisk => "*",
        TokenType::Slash => "/",
        _ => panic!("Invalid operator"),
    }
}
