use super::expr::generate_expr;
use crate::lexer::TokenType;
use crate::parser::parse_tree::var_decl::PTNVarDecl;
use std::collections::HashSet;

pub fn generate_var_decl(var_decl: &PTNVarDecl, includes: &mut HashSet<String>) -> String {
    let mut output = String::new();

    if var_decl.var_type().is_int_type() {
        includes.insert("<stdint.h>".to_string());
    }

    match var_decl.var_type() {
        TokenType::I8 => {
            output.push_str("int8_t ");
        }
        TokenType::I16 => {
            output.push_str("int16_t ");
        }
        TokenType::I32 => {
            output.push_str("int32_t ");
        }
        TokenType::I64 => {
            output.push_str("int64_t ");
        }
        TokenType::U8 => {
            output.push_str("uint8_t ");
        }
        TokenType::U16 => {
            output.push_str("uint16_t ");
        }
        TokenType::U32 => {
            output.push_str("uint32_t ");
        }
        TokenType::U64 => {
            output.push_str("uint64_t ");
        }
        TokenType::F32 => {
            output.push_str("float ");
        }
        TokenType::F64 => {
            output.push_str("double ");
        }
        TokenType::CharType => {
            output.push_str("char "); // TODO: not utf-8 safe!
        }
        TokenType::Bool => {
            output.push_str("int ");
        }
        TokenType::StrType => {
            output.push_str("char* "); // TODO: use own type maybe?
        }
        _ => {
            unreachable!()
        }
    }

    output.push_str(&var_decl.ident());
    output.push_str(" = ");
    output.push_str(&generate_expr(var_decl.expr()));
    output.push_str(";\n");

    output
}
