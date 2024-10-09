use super::number::PTNNumber;
use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_tree::expr::PTNExpr;
use crate::{downcast_node, unexpected_token};

#[derive(Debug, Clone)]
pub enum PTNFactorType {
    Ident { ident: String },
    Number { number: PTNNumber },
    Expr { expr: Box<PTNExpr> },
}

#[derive(Debug, Clone)]
pub struct PTNFactor {
    factor_type: PTNFactorType,
}

impl PTNFactor {
    pub fn factor_type(&self) -> &PTNFactorType {
        &self.factor_type
    }
}

impl PTNode for PTNFactor {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.peek_token().cloned() {
            match token.token_type() {
                TokenType::Identifier => {
                    lexer.next_token();
                    return Box::new(Self {
                        factor_type: PTNFactorType::Ident {
                            ident: token.lexeme().to_string(),
                        },
                    });
                }
                TokenType::LParen => {
                    lexer.next_token();
                    let expr = PTNExpr::parse(lexer);
                    if let Some(token) = lexer.next_token() {
                        if token.token_type() != TokenType::RParen {
                            unexpected_token!(token);
                        }
                    }

                    return Box::new(Self {
                        factor_type: PTNFactorType::Expr {
                            expr: Box::new(downcast_node!(expr, PTNExpr)),
                        },
                    });
                }
                _type if _type.is_number() => {
                    let number = PTNNumber::parse(lexer);
                    return Box::new(Self {
                        factor_type: PTNFactorType::Number {
                            number: downcast_node!(number, PTNNumber),
                        },
                    });
                }
                _ => unexpected_token!(token),
            }
        }

        panic!("Unexpected EOF");
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Factor
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
