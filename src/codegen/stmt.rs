use super::expr::generate_expr;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};

pub fn generate_stmt(stmt: &PTNStmt) -> String {
    let mut output = String::new();
    match stmt.stmt_type() {
        StmtType::Return {
            expr,
        } => {
            output.push_str("return ");
            output.push_str(&generate_expr(expr));
            output.push_str(";\n");
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
