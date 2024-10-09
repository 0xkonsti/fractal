use crate::downcast_node;
use crate::parser::parse_tree::program::PTNProgram;
use crate::parser::parse_tree::PTNodeType;
use crate::parser::Parser;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;

mod block;
mod expr;
mod factor;
mod function;
mod op;
mod program;
mod stmt;
mod term;
mod var_decl;

pub const INDENT: &str = "    ";

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

        // Add the necessary includes
        // TODO: check if these are actually necessary (e.g. might not use any int types)
        self.output.push_str("#include <stdint.h>\n\n");

        self.output
            .push_str(&program::generate_program(&program, 0));
    }

    pub fn save(&self, path: &str) {
        create_dir_all("./out/c").expect("Failed to create directory");
        let mut file = File::create(path).expect("Failed to create file");
        file.write_all(self.output.as_bytes())
            .expect("Failed to write to file");
    }
}

pub fn push_indented(output: &mut String, indent_depth: usize, line: &str) {
    output.push_str(&INDENT.repeat(indent_depth));
    output.push_str(line);
}
