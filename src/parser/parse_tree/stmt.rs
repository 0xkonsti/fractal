use super::block::PTNBlock;
use super::c_block::PTNCBlock;
use super::expr::PTNExpr;
use super::var_decl::PTNVarDecl;
use super::{PTNode, PTNodeType};
use crate::downcast_node;
use crate::lexer::{Lexer, TokenType};
use crate::parser::error;
use crate::parser::parse_tree::expr::get_expr;

#[derive(Debug, Clone)]
pub enum StmtType {
    Expr { expr: PTNExpr },
    VarDecl { var_decl: PTNVarDecl },
    Block { block: PTNBlock },
    CBlock { c_block: PTNCBlock },
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
                TokenType::Dollar => {
                    lexer.next_token();
                    if let Some(token) = lexer.peek_token() {
                        match token.token_type() {
                            TokenType::LBrace => {
                                lexer.next_token();
                                return Box::new(Self {
                                    stmt_type: StmtType::CBlock {
                                        c_block: downcast_node!(PTNCBlock::parse(lexer), PTNCBlock),
                                    },
                                });
                            }
                            _ => {
                                error::expected_token(lexer, TokenType::LBrace);
                            }
                        }
                    }
                }
                TokenType::LBrace => {
                    return Box::new(Self {
                        stmt_type: StmtType::Block {
                            block: downcast_node!(PTNBlock::parse(lexer), PTNBlock),
                        },
                    });
                }
                TokenType::Return => {
                    lexer.next_token();
                    return Box::new(Self {
                        stmt_type: StmtType::Return {
                            expr: get_expr(lexer),
                        },
                    });
                }
                TokenType::Let => {
                    lexer.next_token();
                    return Box::new(Self {
                        stmt_type: StmtType::VarDecl {
                            var_decl: downcast_node!(PTNVarDecl::parse(lexer), PTNVarDecl),
                        },
                    });
                }
                _ => {
                    return Box::new(Self {
                        stmt_type: StmtType::Expr {
                            expr: get_expr(lexer),
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
