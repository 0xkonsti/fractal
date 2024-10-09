use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct PTNCStmt {
    stmt: String, // For now we just paste the C code as is
}

impl PTNCStmt {
    pub fn stmt(&self) -> &str {
        &self.stmt
    }
}

impl PTNode for PTNCStmt {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let mut stmt = String::new();
        while lexer.peek_token().is_some() {
            let token = lexer.next_token().unwrap();
            stmt.push_str(token.lexeme());
            if token.token_type() == TokenType::Semicolon {
                break;
            }
            if let Some(peek_token) = lexer.peek_token() {
                if peek_token.token_type() != TokenType::Semicolon {
                    {
                        stmt.push_str(" ");
                    }
                }
            }
        }
        Box::new(Self {
            stmt,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::CStmt
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
}
