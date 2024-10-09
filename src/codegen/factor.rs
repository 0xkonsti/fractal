use super::expr::generate_expr;
use crate::parser::parse_tree::factor::{PTNFactor, PTNFactorType};

pub fn generate_factor(factor: &PTNFactor) -> String {
    let mut output = String::new();

    match factor.factor_type() {
        PTNFactorType::Expr {
            expr,
        } => {
            output.push_str("(");
            output.push_str(&generate_expr(expr));
            output.push_str(")");
        }
        PTNFactorType::Number {
            number,
        } => {
            output.push_str(number.value());
        }
        PTNFactorType::Ident {
            ident,
        } => {
            output.push_str(&ident.to_string());
        }
    }

    output
}
