use std::collections::HashSet;

use super::function::generate_function_decl;
use crate::parser::parse_tree::program::PTNProgram;

pub fn generate_program(program: &PTNProgram, includes: &mut HashSet<String>, indent_depth: usize) -> String {
    let mut output = String::new();

    for func_decl in program.func_decls() {
        output.push_str(&generate_function_decl(func_decl, includes, indent_depth));
    }

    output
}
