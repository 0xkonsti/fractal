use crate::parser::parse_tree::c_stmt::PTNCStmt;

pub fn generate_c_stmt(stmt: &PTNCStmt) -> &str {
    return stmt.stmt();
}
