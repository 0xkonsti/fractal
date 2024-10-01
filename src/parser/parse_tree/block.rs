use super::stmt::PTNStmt;
use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::{downcast_node, unexpected_token};

#[derive(Debug, Clone)]
pub struct PTNBlock {
    stmts: Vec<PTNStmt>,
}

impl PTNode for PTNBlock {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::LBrace {
                unexpected_token!(token);
            }
        }

        let mut stmts = Vec::new();
        while let Some(token) = lexer.peek_token() {
            match token.token_type() {
                TokenType::RBrace => {
                    break;
                }
                _ => {
                    let stmt = PTNStmt::parse(lexer);
                    stmts.push(downcast_node!(stmt, PTNStmt));
                }
            }
        }

        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::RBrace {
                unexpected_token!(token);
            }
        }

        Box::new(Self {
            stmts,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Block
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
