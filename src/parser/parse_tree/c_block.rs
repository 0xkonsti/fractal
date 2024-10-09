use super::c_stmt::PTNCStmt;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::{Lexer, TokenType};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct PTNCBlock {
    stmts: Vec<PTNCStmt>,
}

impl PTNCBlock {
    pub fn stmts(&self) -> &Vec<PTNCStmt> {
        &self.stmts
    }
}

impl PTNode for PTNCBlock {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let mut stmts = Vec::new();
        while let Some(token) = lexer.peek_token() {
            if token.token_type() == TokenType::RBrace {
                lexer.next_token();
                break;
            }
            stmts.push(downcast_node!(PTNCStmt::parse(lexer), PTNCStmt));
        }
        Box::new(Self {
            stmts,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::CBlock
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}
