use std::collections::HashSet;

use super::block::generate_block;
use crate::parser::parse_tree::func_decl::PTNFuncDecl;

pub fn generate_function_decl(func_decl: &PTNFuncDecl, includes: &mut HashSet<String>) -> String {
    let mut output = String::new();

    // TODO: Generate return type
    output.push_str("int ");

    output.push_str(if func_decl.is_main() {
        "main"
    } else {
        func_decl.name()
    });

    output.push_str("(");

    // TODO: Generate parameter list
    output.push_str("void");

    output.push_str(") ");

    let body = func_decl.body();
    output.push_str(&generate_block(
        body,
        includes,
        None,
        if func_decl.is_main() {
            Some("return 0;\n")
        } else {
            None
        },
    ));

    return output;
}
