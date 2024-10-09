use super::function::generate_function_decl;
use crate::parser::parse_tree::program::PTNProgram;

pub fn generate_program(program: &PTNProgram, indent_depth: usize) -> String {
    let mut output = String::new();

    for func_decl in program.func_decls() {
        output.push_str(&generate_function_decl(func_decl, indent_depth));
    }

    output
}
