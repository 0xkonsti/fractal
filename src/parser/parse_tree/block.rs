use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::unexpected_token;

#[derive(Debug, Clone)]
pub struct PTNBlock {
    // stmts: Vec<PTNStmt>,
}

impl PTNode for PTNBlock {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::LBrace {
                unexpected_token!(token);
            }
        }

        // TODO: Parse stmts

        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::RBrace {
                unexpected_token!(token);
            }
        }

        Box::new(Self {})
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Block
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
