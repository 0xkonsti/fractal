use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::unexpected_token;

#[derive(Debug, Clone)]
pub struct PTNParamList {
    // params: Vec<PTNParam>,
}

impl PTNode for PTNParamList {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::LParen {
                unexpected_token!(token);
            }
        }

        // TODO: Parse params

        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::RParen {
                unexpected_token!(token);
            }
        }

        Box::new(Self {
            // params: Vec::new(),
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::ParamList
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
