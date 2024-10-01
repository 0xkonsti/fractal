mod block;
mod expr;
mod factor;
mod func_decl;
mod number;
mod param_list;
mod program;
mod stmt;
mod term;

use crate::lexer::Lexer;

#[macro_export]
macro_rules! downcast_node {
    ($node:expr, $node_type:ident) => {
        $node.as_any().downcast_ref::<$node_type>().unwrap().clone()
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PTNodeType {
    Program,

    FuncDecl,

    ParamList,

    Block,
    Stmt,
    Expr,
    Term,
    Factor,
    Number,
}

pub trait PTNode: std::fmt::Debug {
    fn parse(lexer: &mut Lexer) -> Box<dyn PTNode>
    where
        Self: Sized;

    fn node_type(&self) -> PTNodeType;

    fn as_any(&self) -> Box<dyn std::any::Any>;
}

#[derive(Debug)]
pub struct ParseTree {
    root: Box<dyn PTNode>,
}

impl PTNodeType {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_uppercase().replace("_", "")
    }
}

impl ParseTree {
    pub fn parse(lexer: &mut Lexer) -> Self {
        let root = program::PTNProgram::parse(lexer);
        Self {
            root,
        }
    }

    pub fn root(&self) -> &Box<dyn PTNode> {
        &self.root
    }
}
