use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};

#[derive(Debug, Clone)]
pub struct PTNCBlock {
    raw: String,
}

impl PTNCBlock {
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

impl PTNode for PTNCBlock {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let mut raw = String::new();

        lexer.next_token(); // TODO: check if this is a '{'

        while let Some(token) = lexer.next_token() {
            println!("{}", token.lexeme());
            if token.token_type() == TokenType::RBrace {
                break;
            }

            raw.push_str(token.lexeme());
            raw.push(' ');
        }

        Box::new(Self {
            raw,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::CBlock
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
