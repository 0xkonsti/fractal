use super::factor::PTNFactor;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::{Lexer, TokenType};

#[derive(Debug, Clone)]
pub enum PTNTermType {
    Factor {
        factor: PTNFactor,
    },
    BinOp {
        left: PTNFactor,
        right: Box<PTNTerm>,
        op: TokenType,
    },
}

#[derive(Debug, Clone)]
pub struct PTNTerm {
    term_type: PTNTermType,
}

impl PTNTerm {
    pub fn term_type(&self) -> &PTNTermType {
        &self.term_type
    }
}

impl PTNode for PTNTerm {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let left = PTNFactor::parse(lexer);

        if let Some(token) = lexer.peek_token().cloned() {
            match token.token_type() {
                TokenType::Asterisk | TokenType::Slash => {
                    lexer.next_token();
                    let right = PTNTerm::parse(lexer);
                    return Box::new(Self {
                        term_type: PTNTermType::BinOp {
                            left: downcast_node!(left, PTNFactor),
                            right: Box::new(downcast_node!(right, PTNTerm)),
                            op: token.token_type(),
                        },
                    });
                }
                _ => {}
            }
        }

        Box::new(Self {
            term_type: PTNTermType::Factor {
                factor: downcast_node!(left, PTNFactor),
            },
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Term
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
