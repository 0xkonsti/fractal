use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::unexpected_token;

#[derive(Debug, Clone)]
pub enum NumberType {
    Integer,
    Float,
}

#[derive(Debug, Clone)]
pub struct PTNNumber {
    number_type: NumberType,
    value: String,
}

impl PTNNumber {
    pub fn number_type(&self) -> &NumberType {
        &self.number_type
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PTNode for PTNNumber {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let (number_type, value) = if let Some(token) = lexer.next_token() {
            (
                match token.token_type() {
                    TokenType::Integer => NumberType::Integer,
                    TokenType::Float => NumberType::Float,
                    _ => unexpected_token!(token),
                },
                token.lexeme().to_string(),
            )
        } else {
            panic!("Unexpected EOF");
        };

        Box::new(Self {
            number_type,
            value,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Number
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
