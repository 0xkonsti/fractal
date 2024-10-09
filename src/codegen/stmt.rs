use std::collections::HashSet;

use super::{block::generate_block, expr::generate_expr};
use super::var_decl::generate_var_decl;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};

pub fn generate_stmt(stmt: &PTNStmt, includes: &mut HashSet<String>) -> (String, bool) {
    let mut output = String::new();
    let mut multi_line = false;
    match stmt.stmt_type() {
        StmtType::Block {
            block,
        } => {
            output.push_str(&generate_block(block, includes, None, None));
            multi_line = true;
        }
        StmtType::Return {
            expr,
        } => {
            output.push_str("return ");
            output.push_str(&generate_expr(expr));
            output.push_str(";\n");
        }
        StmtType::VarDecl {
            var_decl,
        } => {
            output.push_str(&generate_var_decl(&var_decl, includes));
        }
        StmtType::Expr {
            expr,
        } => {
            output.push_str(&generate_expr(expr));
            output.push_str(";\n");
        }
    }

    (output, multi_line)
}
