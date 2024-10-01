use super::expr::PTNExpr;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::{Lexer, TokenType};

#[derive(Debug, Clone)]
pub enum StmtType {
    Expr { expr: PTNExpr },
    Return { expr: PTNExpr },
}

#[derive(Debug, Clone)]
pub struct PTNStmt {
    stmt_type: StmtType,
}

impl PTNStmt {
    pub fn stmt_type(&self) -> &StmtType {
        &self.stmt_type
    }
}

impl PTNode for PTNStmt {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.peek_token() {
            match token.token_type() {
                TokenType::Return => {
                    lexer.next_token();
                    return Box::new(Self {
                        stmt_type: StmtType::Return {
                            expr: get_expr(lexer)
                        },
                    });
                }
                _ => {
                    return Box::new(Self {
                        stmt_type: StmtType::Expr {
                            expr: get_expr(lexer)
                        },
                    });
                }
            }
        }

        panic!("Unexpected EOF");
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Stmt
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

fn get_expr(lexer: &mut Lexer) -> PTNExpr {
    let expr = PTNExpr::parse(lexer);
    if let Some(token) = lexer.next_token() {
        if token.token_type() != TokenType::SemiColon {
            panic!("Expected semicolon");
        }
    }
    downcast_node!(expr, PTNExpr)
}
