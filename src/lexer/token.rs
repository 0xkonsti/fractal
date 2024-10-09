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
    Equal,    // =

    Dollar, // $

    Colon,     // :
    SemiColon, // ;

    // Multi-character tokens

    // Keywords
    Fn,
    Return,
    Let,

    // Types (more Keywords)
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    CharType,
    StrType,

    // Literals
    Integer,
    Float,
    String,
    Character,

    // Identifiers
    Identifier,
}

impl TokenType {
    pub fn is_single_char_token(c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LParen),
            ')' => Some(TokenType::RParen),
            '{' => Some(TokenType::LBrace),
            '}' => Some(TokenType::RBrace),

            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Asterisk),
            '/' => Some(TokenType::Slash),
            '=' => Some(TokenType::Equal),

            '$' => Some(TokenType::Dollar),

            ':' => Some(TokenType::Colon),
            ';' => Some(TokenType::SemiColon),
            _ => None,
        }
    }

    pub fn is_keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "fn" => Some(TokenType::Fn),
            "return" => Some(TokenType::Return),
            "let" => Some(TokenType::Let),

            // Types
            "i8" => Some(TokenType::I8),
            "i16" => Some(TokenType::I16),
            "i32" => Some(TokenType::I32),
            "i64" => Some(TokenType::I64),
            "u8" => Some(TokenType::U8),
            "u16" => Some(TokenType::U16),
            "u32" => Some(TokenType::U32),
            "u64" => Some(TokenType::U64),
            "f32" => Some(TokenType::F32),
            "f64" => Some(TokenType::F64),
            "bool" => Some(TokenType::Bool),
            "char" => Some(TokenType::CharType),
            "str" => Some(TokenType::StrType),

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
        format!("{:?}", self).to_uppercase()
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
