use crate::lexer::{Lexer, TokenType};

pub fn expected_custom(lexer: &mut Lexer, expected: String) {
    let mut prev_token = lexer.prev_token().unwrap().clone();
    let mut location = prev_token.location_mut().clone();
    location.advance_by(&prev_token.lexeme());
    // eprint!("Expected semicolon: {}", location);
    panic!("Expected {}: {}", expected, location);
}

pub fn expected_token(lexer: &mut Lexer, expected: TokenType) {
    expected_custom(lexer, expected.to_string());
}
