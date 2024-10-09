use std::collections::HashSet;

use super::push_indented;
use super::stmt::generate_stmt;
use crate::parser::parse_tree::block::PTNBlock;

pub fn generate_block(
    block: &PTNBlock,
    includes: &mut HashSet<String>,
    indent_depth: usize,
    pre: Option<&str>,
    post: Option<&str>,
) -> String {
    let mut output = String::new();

    push_indented(&mut output, indent_depth, "{\n");

    if let Some(pre) = pre {
        push_indented(&mut output, indent_depth + 1, pre);
    }

    for stmt in block.stmts() {
        push_indented(&mut output, indent_depth + 1, &generate_stmt(stmt, includes));
    }

    if let Some(post) = post {
        push_indented(&mut output, indent_depth + 1, post);
    }

    output.push_str("}\n");

    output
}
