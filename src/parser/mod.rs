use crate::lexer::Lexer;

pub mod parse_tree;
mod error;

#[macro_export]
macro_rules! unexpected_token {
    ($token:expr) => {
        panic!("Unexpected token: {:#?}", $token)
    };
}

#[derive(Debug)]
pub struct Parser {
    tree: parse_tree::ParseTree,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        Self {
            tree: parse_tree::ParseTree::parse(lexer),
        }
    }

    pub fn tree(&self) -> &parse_tree::ParseTree {
        &self.tree
    }
}
