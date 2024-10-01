use crate::downcast_node;
use crate::lexer::{Token, TokenType};
use crate::parser::parse_tree::expr::{PTNExpr, PTNExprType};
use crate::parser::parse_tree::factor::{PTNFactor, PTNFactorType};
use crate::parser::parse_tree::program::PTNProgram;
use crate::parser::parse_tree::stmt::{PTNStmt, StmtType};
use crate::parser::parse_tree::term::{PTNTerm, PTNTermType};
use crate::parser::parse_tree::PTNodeType;
use crate::parser::Parser;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;

const INDENT: &str = "    ";

pub struct Codegen {
    output: String,
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            output: String::new(),
        }
    }

    pub fn generate(&mut self, parser: &mut Parser) {
        let root = parser.tree().root();

        if root.node_type() != PTNodeType::Program {
            panic!("Root node is not a program");
        }

        let program = downcast_node!(root, PTNProgram);

        for func_decl in program.func_decls() {
            self.output.push_str("int ");
            self.output.push_str(if func_decl.is_main() {
                "main"
            } else {
                func_decl.name()
            });
            self.output.push_str("(");

            // TODO: Generate parameter list
            self.output.push_str("void");

            self.output.push_str(") {\n");

            let body = func_decl.body();
            for stmt in body.stmts() {
                self.output.push_str(INDENT);
                self.output.push_str(&generate_stmt(stmt));
            }

            if func_decl.is_main() {
                self.output.push_str(INDENT);
                self.output.push_str("return 0;\n");
            }

            self.output.push_str("}\n");
        }
    }

    pub fn save(&self, path: &str) {
        create_dir_all("./out/c").expect("Failed to create directory");
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(self.output.as_bytes())
            .expect("Failed to write to file");
    }
}

fn generate_stmt(stmt: &PTNStmt) -> String {
    let mut output = String::new();
    match stmt.stmt_type() {
        StmtType::Return {
            expr,
        } => {
            output.push_str("return ");
            output.push_str(&generate_expr(expr));
            output.push_str(";\n");
        }
        StmtType::Expr {
            expr,
        } => {
            output.push_str(&generate_expr(expr));
            output.push_str(";\n");
        }
        StmtType::CBlock {
            c_block,
        } => {
            output.push_str("{ ");
            output.push_str(&c_block.raw());
            output.push_str("}\n");
        }
    }

    output
}

fn generate_expr(expr: &PTNExpr) -> String {
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

fn generate_term(term: &PTNTerm) -> String {
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

fn generate_factor(factor: &PTNFactor) -> String {
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

fn generate_op(op: &TokenType) -> &str {
    match op {
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Asterisk => "*",
        TokenType::Slash => "/",
        _ => panic!("Invalid operator"),
    }
}
