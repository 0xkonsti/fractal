use super::push_indented;
use super::stmt::generate_stmt;
use crate::parser::parse_tree::block::PTNBlock;
use std::collections::HashSet;

pub fn generate_block(
    block: &PTNBlock,
    includes: &mut HashSet<String>,
    pre: Option<&str>,
    post: Option<&str>,
) -> String {
    let mut output = String::new();

    push_indented(&mut output, 0, "{\n");

    if let Some(pre) = pre {
        push_indented(&mut output, 1, pre);
    }

    for stmt in block.stmts() {
        let (stmt, multi_line) = generate_stmt(stmt, includes);
        if multi_line {
            for line in stmt.lines() {
                push_indented(&mut output, 1, format!("{}\n", line).as_str());
            }
        } else {
            push_indented(&mut output, 1, &stmt);
        }
    }

    if let Some(post) = post {
        push_indented(&mut output, 1, post);
    }

    push_indented(&mut output, 0, "}\n");

    output
}
