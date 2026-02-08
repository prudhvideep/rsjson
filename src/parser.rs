use crate::{JsonError, JsonValue, Token};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
}

impl IntoIterator for Parser {
    type Item = Token;
    type IntoIter = Peekable<std::vec::IntoIter<Token>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter().peekable()
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens }
    }

    pub fn parse(self) -> Result<JsonValue,JsonError> {
        let mut token_iter = self.into_iter();

        while let Some(token) = token_iter.peek() {
            println!("{:?}",token);
            token_iter.next();
        }

        Ok(JsonValue::Boolean)
    }
}