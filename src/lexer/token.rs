use super::location::Location;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Special tokens
    Eof,
    Invalid,

    // Single-character tokens
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }

    // Multi-character tokens

    // Keywords
    Fn_,

    // Types

    // Literals
    Integer,
    Float,

    // Identifiers
    Identifier,
}

impl TokenType {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_uppercase().replace("_", "")
    }
}

#[derive(Debug)]
pub struct Token {
    tt: TokenType,
    lexeme: String,
    location: Location,
}

impl Token {
    pub fn new(tt: TokenType, lexeme: String, location: Location) -> Self {
        Self {
            tt,
            lexeme,
            location,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:<15}] ~ {:<60} {}",
            self.tt.to_string(),
            self.lexeme,
            self.location
        )
    }
}

pub struct SingleCharToken {
    tt: TokenType,
    c: char,
}

impl SingleCharToken {
    pub fn new(tt: TokenType, c: char) -> Self {
        Self {
            tt,
            c,
        }
    }

    pub fn get_tt(&self) -> TokenType {
        self.tt
    }

    pub fn get_c(&self) -> char {
        self.c
    }
}
