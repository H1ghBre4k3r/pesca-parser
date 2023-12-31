mod lexmap;
mod token;
mod tokens;

pub use lexmap::*;
pub use token::*;
pub use tokens::*;

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError(String);

pub type LexResult<T> = Result<T, LexError>;

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Error for LexError {}

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    lexikon: Lexikon,
    position: usize,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: vec![],
            lexikon: Lexikon::new(),
            position: 0,
            input,
        }
    }

    fn eat_whitespace(&mut self) {
        while self
            .input
            .as_bytes()
            .get(self.position)
            .map(|c| c.is_ascii_whitespace())
            .unwrap_or(false)
        {
            self.position += 1;
        }
    }

    pub fn lex(mut self) -> LexResult<Vec<Token>> {
        while self.position != self.input.len() {
            self.eat_whitespace();
            let (len, res) = self
                .lexikon
                .find_longest_match(&self.input[self.position..], self.position)
                .clone();

            match res {
                Some(t) => self.tokens.push(t),
                None => {
                    if self.position == self.input.len() {
                        return Ok(self.tokens);
                    } else {
                        panic!(
                            "Failed to lex '{}' at position {}; remaining '{}'",
                            self.input,
                            self.position,
                            &self.input[self.position..]
                        );
                    }
                }
            };
            self.position += len;
        }

        Ok(self.tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_alphabetic_id() {
        let lexer = Lexer::new("letter");

        assert_eq!(
            Ok(vec![Token::Id {
                value: "letter".into(),
                position: 0
            }]),
            lexer.lex()
        )
    }

    #[test]
    fn test_lex_numeric() {
        let lexer = Lexer::new("1337");

        assert_eq!(
            Ok(vec![Token::Integer {
                value: 1337,
                position: 0
            }]),
            lexer.lex()
        )
    }

    #[test]
    fn test_lex_function() {
        let lexer = Lexer::new("fn () {}");

        assert_eq!(
            Ok(vec![
                Token::FnKeyword { position: 0 },
                Token::LParen { position: 0 },
                Token::RParen { position: 0 },
                Token::LBrace { position: 0 },
                Token::RBrace { position: 0 }
            ]),
            lexer.lex()
        );
    }

    #[test]
    fn test_lex_let() {
        let lexer = Lexer::new("let foo = 42;");

        assert_eq!(
            Ok(vec![
                Token::Let { position: 0 },
                Token::Id {
                    value: "foo".into(),
                    position: 0
                },
                Token::Assign { position: 0 },
                Token::Integer {
                    value: 42,
                    position: 0
                },
                Token::Semicolon { position: 0 }
            ]),
            lexer.lex()
        );
    }
}
