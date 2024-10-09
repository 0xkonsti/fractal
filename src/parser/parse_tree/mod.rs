pub mod block;
pub mod c_block;
pub mod c_stmt;
pub mod expr;
pub mod factor;
pub mod func_decl;
pub mod number;
pub mod param_list;
pub mod program;
pub mod stmt;
pub mod term;
pub mod var_decl;

use crate::lexer::Lexer;

#[macro_export]
macro_rules! downcast_node {
    ($node:expr, $node_type:ident) => {
        $node.as_any().downcast_ref::<$node_type>().unwrap().clone()
    };
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum PTNodeType {
    Program,

    FuncDecl,

    ParamList,

    Block,
    CBlock,
    Stmt,
    CStmt,
    VarDecl,
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
        format!("{:?}", self).to_uppercase()
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
