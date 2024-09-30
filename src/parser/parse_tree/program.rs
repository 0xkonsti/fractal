use super::func_decl::PTNFuncDecl;
use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::{downcast_node, unexpected_token};

#[derive(Debug, Clone)]
pub struct PTNProgram {
    func_decls: Vec<PTNFuncDecl>,
}

impl PTNode for PTNProgram {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        let mut func_decls = Vec::new();
        while let Some(token) = lexer.peek_token() {
            match token.token_type() {
                TokenType::Fn_ => {
                    let func_decl = PTNFuncDecl::parse(lexer);
                    func_decls.push(downcast_node!(func_decl, PTNFuncDecl));
                }
                TokenType::Eof => {
                    break;
                }
                _ => {
                    unexpected_token!(token);
                }
            }
        }
        Box::new(Self {
            func_decls,
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::Program
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
