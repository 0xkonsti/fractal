use super::block::PTNBlock;
use super::param_list::PTNParamList;
use super::{PTNode, PTNodeType};
use crate::lexer::{Lexer, TokenType};
use crate::{downcast_node, unexpected_token};

#[derive(Debug, Clone)]
pub struct PTNFuncDecl {
    name: String,
    is_main: bool,
    param_list: PTNParamList,
    body: PTNBlock,
}

impl PTNFuncDecl {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }

    pub fn param_list(&self) -> &PTNParamList {
        &self.param_list
    }

    pub fn body(&self) -> &PTNBlock {
        &self.body
    }
}

impl PTNode for PTNFuncDecl {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode> {
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::Fn_ {
                unexpected_token!(token);
            }
        }

        let mut name = String::new();
        if let Some(token) = lexer.next_token() {
            if token.token_type() != TokenType::Identifier {
                unexpected_token!(token);
            }

            name = token.lexeme().to_string();
        }
        let is_main = name == "main";

        let param_list = PTNParamList::parse(lexer);

        let block = PTNBlock::parse(lexer);

        Box::new(Self {
            name,
            is_main,
            param_list: downcast_node!(param_list, PTNParamList),
            body: downcast_node!(block, PTNBlock),
        })
    }

    fn node_type(&self) -> PTNodeType {
        PTNodeType::FuncDecl
    }

    fn as_any(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}
