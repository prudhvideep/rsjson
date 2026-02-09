use crate::{JsonError, JsonValue, Token, parse};
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

    fn parse_object(token_iter : &mut Peekable<std::vec::IntoIter<Token>>) -> Result<JsonValue,JsonError> {
        todo!()
    }

    fn parse_array(token_iter : &mut Peekable<std::vec::IntoIter<Token>>) -> Result<JsonValue,JsonError> {
        todo!()
    }

    fn parse_string(token_iter : &mut Peekable<std::vec::IntoIter<Token>>) -> Result<JsonValue,JsonError> {
        todo!()
    }

    fn parse_number(token_iter : &mut Peekable<std::vec::IntoIter<Token>>) -> Result<JsonValue,JsonError> {
        todo!()
    }

    fn parse_boolean(token_iter : &mut Peekable<std::vec::IntoIter<Token>>) -> Result<JsonValue,JsonError> {
        todo!()
    }

    pub fn parse(self) -> Result<JsonValue, JsonError> {
        let mut token_iter: Peekable<std::vec::IntoIter<Token>> = self.into_iter();

        while let Some(token) = token_iter.peek() {
            match token {
                Token::LeftBrace => {
                    //consume the brace
                    token_iter.next();
                    
                    //check for the colon
                    let _res  = match token_iter.peek() {
                        Some(Token::Colon) => {
                            //consume the colon
                            token_iter.next();
                            Self::parse_object(&mut token_iter)
                        }
                        _ => {
                            Err(JsonError::UnexpectedToken)
                        }
                    };
                }
                Token::LeftBracket => {
                    //consume the bracket
                    token_iter.next();

                    let _res = match token_iter.peek() {
                        Some(_token) => {
                          Self::parse_array(&mut token_iter)
                        }
                        _ => {
                            Err(JsonError::UnexpectedToken)
                        }
                    };
                    println!("Parse json array")
                }
                Token::Number(_number) => {
                    let _res = Self::parse_number(&mut token_iter);
                }
                Token::Quote => {
                    let _res = Self::parse_string(&mut token_iter);
                }
                Token::False | Token::True => {
                    let _res = Self::parse_boolean(&mut token_iter);
                }
                _ => {
                    token_iter.next();
                }
            }
        }

        Ok(JsonValue::Boolean)
    }
}
