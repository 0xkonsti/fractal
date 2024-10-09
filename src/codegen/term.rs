use super::factor::generate_factor;
use super::op::generate_op;
use crate::parser::parse_tree::term::{PTNTerm, PTNTermType};

pub fn generate_term(term: &PTNTerm) -> String {
    let mut output = String::new();

    match term.term_type() {
        PTNTermType::Factor {
            factor,
        } => {
            output.push_str(&generate_factor(factor));
        }
        PTNTermType::BinOp {
            left,
            right,
            op,
        } => {
            output.push_str(&generate_factor(left));
            output.push_str(" ");
            output.push_str(generate_op(op));
            output.push_str(" ");
            output.push_str(&generate_term(right));
        }
    }

    output
}
