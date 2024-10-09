use super::op::generate_op;
use super::term::generate_term;
use crate::parser::parse_tree::expr::{PTNExpr, PTNExprType};

pub fn generate_expr(expr: &PTNExpr) -> String {
    let mut output = String::new();

    match expr.expr_type() {
        PTNExprType::Term {
            term,
        } => {
            output.push_str(&generate_term(term));
        }
        PTNExprType::BinOp {
            left,
            right,
            op,
        } => {
            output.push_str(&generate_term(left));
            output.push_str(" ");
            output.push_str(generate_op(op));
            output.push_str(" ");
            output.push_str(&generate_expr(right));
        }
    }

    output
}
