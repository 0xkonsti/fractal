use super::expr::PTNExpr;
use super::{PTNode, PTNodeType};
use crate::lexer::{self, TokenType};
use crate::parser::error;
use crate::parser::parse_tree::expr::{self, get_expr};
use crate::{downcast_node, unexpected_token};

#[derive(Debug, Clone)]
pub struct PTNVarDecl {
    ident: String,
    var_type: TokenType,
    expr: PTNExpr,
}

impl PTNVarDecl {
    pub fn ident(&self) -> &String {
        &self.ident
    }

    pub fn var_type(&self) -> &TokenType {
        &self.var_type
    }

    pub fn expr(&self) -> &PTNExpr {
        &self.expr
    }
}

impl PTNode for PTNVarDecl {
    fn parse(lexer: &mut lexer::Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::Identifier {
                unexpected_token!(token);
            }
            let ident = token.lexeme().to_string();

            let var_type = if let Some(token) = lexer.next_token() {
                if token.token_type() != TokenType::Colon {
                    error::expected_token(lexer, TokenType::Colon);
                }

                if let Some(token) = lexer.next_token() {
                    if token.token_type().is_type() {
                        token.token_type()
                    } else {
                        error::expected_custom(lexer, "type declaration".to_string());
                        panic!();
                    }
                } else {
                    panic!("Unexpected EOF");
                }
            } else {
                panic!("Unexpected EOF");
            };

            if let Some(token) = lexer.peek_token() {
                if token.token_type() == TokenType::Equal {
                    lexer.next_token();
                } else {
                    error::expected_token(lexer, TokenType::Equal);
                }
            }

            // TODO: Add support for type inference -> expr needs to have a type which gets calculated during parsing
            //      also needed for type checking (rightnow type is unchecked e.g i32 = "hello" is valid)
            let expr = get_expr(lexer);

            return Box::new(Self {
                ident,
                var_type,
                expr,
            });
        }

        panic!("Unexpected EOF");
    }

    fn node_type(&self) -> super::PTNodeType {
        PTNodeType::VarDecl
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
