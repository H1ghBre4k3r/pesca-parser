mod id;
mod num;

pub use self::id::*;
pub use self::num::*;

use crate::lexer::Tokens;
use crate::parser::combinators::Comb;
use crate::{
    lexer::Token,
    parser::{FromTokens, ParseError},
};

use super::AstNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Id(Id),
    Num(Num),
    Addition(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
}

impl FromTokens<Token> for Expression {
    fn parse(tokens: &mut Tokens<Token>) -> Result<AstNode, ParseError> {
        let matcher = Comb::NUM | Comb::ID;

        let result = matcher.parse(tokens)?;
        let expr = match result.get(0) {
            Some(AstNode::Id(id)) => Expression::Id(id.clone()),
            Some(AstNode::Num(num)) => Expression::Num(num.clone()),
            None | Some(_) => unreachable!(),
        };

        let Some(next) = tokens.peek() else {
            return Ok(expr.into());
        };

        let tuple = match next {
            Token::Semicolon { .. } => return Ok(expr.into()),
            Token::Times { .. } => {
                tokens.next();
                Expression::Multiplication
            }
            Token::Plus { .. } => {
                tokens.next();
                Expression::Addition
            }
            t => todo!("{t:?}"),
        };

        let matcher = Comb::EXPR;
        let result = matcher.parse(tokens)?;
        let rhs = match result.get(0) {
            Some(AstNode::Expression(rhs)) => rhs.clone(),
            None | Some(_) => unreachable!(),
        };

        Ok(tuple(Box::new(expr), Box::new(rhs)).into())
    }
}

impl From<Expression> for AstNode {
    fn from(value: Expression) -> Self {
        AstNode::Expression(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        let tokens = vec![Token::Id {
            value: "some_id".into(),
            position: (0, 0),
        }];
        let tokens = tokens;

        assert_eq!(
            Expression::parse(&mut tokens.into()),
            Ok(AstNode::Expression(Expression::Id(Id("some_id".into()))))
        )
    }

    #[test]
    fn test_parse_num() {
        let tokens = vec![Token::Num {
            value: 42,
            position: (0, 0),
        }];
        let tokens = tokens;

        assert_eq!(
            Expression::parse(&mut tokens.into()),
            Ok(AstNode::Expression(Expression::Num(Num(42))))
        )
    }
}
