use std::collections::HashSet;

use super::expr::generate_expr;
use super::var_decl::generate_var_decl;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};

pub fn generate_stmt(stmt: &PTNStmt, includes: &mut HashSet<String>,) -> String {
    let mut output = String::new();
    match stmt.stmt_type() {
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

    output
}
