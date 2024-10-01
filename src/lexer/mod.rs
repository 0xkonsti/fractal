pub mod location;
pub mod token;

use location::Location;
use std::iter::Peekable;
use std::str::Chars;
pub use token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    cursor: usize,
    location: Location,

    peeked: Option<Token>,

    at_eof: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(path: String, source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
            cursor: 0,
            location: Location::new(path, 1, 1),

            peeked: None,

            at_eof: false,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.__next()
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = self.next();
        }
        self.peeked.as_ref()
    }

    fn __next(&mut self) -> Option<Token> {
        if let Some(token) = self.peeked.take() {
            return Some(token);
        }

        self.next()
    }

    fn trim_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.next();
        if let Some(c) = c {
            self.cursor += 1;
            self.location.advance(&c);
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_eof {
            return None;
        }

        self.trim_whitespace();

        let location = self.location.clone();
        let mut is_float = false;
        let c = self.advance();

        if handle_comment(self) {
            return self.next();
        }

        let (tt, lexeme) = match c {
            Some(c) => match c {
                c if is_identifier(c, true) => {
                    let mut lexeme = c.to_string();
                    while let Some(&c) = self.peek() {
                        if is_identifier(c, false) {
                            lexeme.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }

                    if let Some(tt) = TokenType::is_keyword(&lexeme) {
                        (tt, lexeme)
                    } else {
                        (TokenType::Identifier, lexeme)
                    }
                }
                c if is_number(c, &mut is_float) => {
                    let mut lexeme = c.to_string();
                    while let Some(&c) = self.peek() {
                        if is_number(c, &mut is_float) {
                            lexeme.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }

                    (
                        if is_float {
                            TokenType::Float
                        } else {
                            TokenType::Integer
                        },
                        lexeme,
                    )
                }
                _ => {
                    // TODO: handle multi-char tokens before single-char tokens

                    if let Some(tt) = handle_single_char_token(c) {
                        (tt, c.to_string())
                    } else {
                        (TokenType::Invalid, c.to_string())
                    }
                }
            },
            None => {
                self.at_eof = true;
                (TokenType::Eof, String::new())
            }
        };

        let token = Some(Token::new(tt, lexeme, location));
        token
    }
}

// -------------< Helper Functions >------------- //

fn handle_comment(lexer: &mut Lexer) -> bool {
    let c = lexer.peek();
    if let Some(c) = c {
        match c {
            '/' => {
                while let Some(&c) = lexer.peek() {
                    if c == '\n' {
                        break;
                    }
                    lexer.advance();
                }

                return true;
            }
            '*' => {
                while let Some(c) = lexer.advance() {
                    if c == '*' {
                        if let Some(&next) = lexer.peek() {
                            if next == '/' {
                                lexer.advance();
                                break;
                            }
                        }
                    }
                }

                return true;
            }
            _ => {}
        }
    }

    false
}

fn is_identifier(c: char, is_first_char: bool) -> bool {
    c == '_' || c.is_alphabetic() || (!is_first_char && c.is_digit(10))
}

fn is_number(c: char, is_float: &mut bool) -> bool {
    if c == '.' {
        if *is_float {
            return false;
        }
        *is_float = true;
        return true;
    }

    c.is_digit(10)
}

fn handle_single_char_token(c: char) -> Option<TokenType> {
    match c {
        '(' => Some(TokenType::LParen),
        ')' => Some(TokenType::RParen),
        '{' => Some(TokenType::LBrace),
        '}' => Some(TokenType::RBrace),

        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '*' => Some(TokenType::Asterisk),
        '/' => Some(TokenType::Slash),

        '$' => Some(TokenType::Dollar),

        ';' => Some(TokenType::SemiColon),
        _ => None,
    }
}
