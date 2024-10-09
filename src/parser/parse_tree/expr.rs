use super::term::PTNTerm;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::{Lexer, TokenType};

#[derive(Debug, Clone)]
pub enum PTNExprType {
    Term {
        term: PTNTerm,
    },
    BinOp {
        left: PTNTerm,
        right: Box<PTNExpr>,
        op: TokenType,
    },
}

#[derive(Debug, Clone)]
pub struct PTNExpr {
    expr_type: PTNExprType,
}

impl PTNExpr {
    pub fn expr_type(&self) -> &PTNExprType {
        &self.expr_type
    }
}

impl PTNode for PTNExpr {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let left = PTNTerm::parse(lexer);

        if let Some(token) = lexer.peek_token().cloned() {
            match token.token_type() {
                TokenType::Plus | TokenType::Minus => {
                    lexer.next_token();
                    let right = PTNExpr::parse(lexer);
                    return Box::new(Self {
                        expr_type: PTNExprType::BinOp {
                            left: downcast_node!(left, PTNTerm),
                            right: Box::new(downcast_node!(right, PTNExpr)),
                            op: token.token_type(),
                        },
                    });
                }
                _ => {}
            }
        }

        Box::new(Self {
            expr_type: PTNExprType::Term {
                term: downcast_node!(left, PTNTerm),
            },
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Expr
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

pub fn get_expr(lexer: &mut Lexer) -> PTNExpr {
    let expr = PTNExpr::parse(lexer);
    if let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        if token.token_type() != TokenType::Semicolon {
            panic!("Expected semicolon");
        }
    }
    downcast_node!(expr, PTNExpr)
}
