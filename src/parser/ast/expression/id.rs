use crate::{
    lexer::Token,
    parser::{FromTokens, ParseError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id(pub String);

impl FromTokens for Id {
    fn parse<I>(tokens: &mut std::iter::Peekable<I>) -> Result<Self, crate::parser::ParseError>
    where
        I: Iterator<Item = crate::lexer::Token>,
        Self: Sized,
    {
        let value = match tokens.next() {
            Some(Token::Id { value, .. }) => value,
            Some(token) => {
                return Err(ParseError {
                    message: "Tried to parse Id from non id token".into(),
                    position: Some(token.position()),
                })
            }
            None => return Err(ParseError::eof("Id")),
        };
        Ok(Id(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![Token::Id {
            value: "some_id".into(),
            position: (0, 0),
        }];
        let mut tokens = tokens.into_iter().peekable();
        assert_eq!(Id::parse(&mut tokens), Ok(Id("some_id".into())));
    }

    #[test]
    fn test_error_on_non_id() {
        let tokens = vec![Token::Num {
            value: 3,
            position: (0, 0),
        }];
        let mut tokens = tokens.into_iter().peekable();
        assert!(Id::parse(&mut tokens).is_err());
    }

    #[test]
    fn test_error_on_eof() {
        let tokens = vec![];
        let mut tokens = tokens.into_iter().peekable();
        assert!(Id::parse(&mut tokens).is_err());
    }
}
