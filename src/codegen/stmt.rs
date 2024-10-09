use super::block::generate_block;
use super::c_stmt::generate_c_stmt;
use super::expr::generate_expr;
use super::push_indented;
use super::var_decl::generate_var_decl;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};
use std::collections::HashSet;

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
        StmtType::CBlock {
            c_block,
        } => {
            push_indented(&mut output, 0, "{\n");
            for stmt in c_block.stmts() {
                let stmt_str = generate_c_stmt(stmt);
                push_indented(&mut output, 2, format!("{}\n", stmt_str).as_str());
            }
            push_indented(&mut output, 1, "}\n");
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
