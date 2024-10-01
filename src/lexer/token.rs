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

    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /

    Dollar, // $

    SemiColon, // ;

    // Multi-character tokens

    // Keywords
    Fn_,
    Return,

    // Types

    // Literals
    Integer,
    Float,

    // Identifiers
    Identifier,
}

impl TokenType {
    pub fn is_keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "fn" => Some(TokenType::Fn_),
            "return" => Some(TokenType::Return),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            TokenType::Integer | TokenType::Float => true,
            _ => false,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self).to_uppercase().replace("_", "")
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn token_type(&self) -> TokenType {
        self.tt
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
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
