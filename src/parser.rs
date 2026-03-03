use crate::{JsonError, JsonValue, Token};
use std::{iter::Peekable};

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

    fn parse_object(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        todo!()
    }

    fn expect_comma(token_iter: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<(),JsonError> {
        if let Some(Token::Comma) = token_iter.peek() {
            token_iter.next();
            Ok(())
        } else {
            Err(JsonError::UnexpectedToken)
        }
    }

    fn parse_array(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let mut values: Vec<JsonValue> = Vec::new();

        while let Some(token) = token_iter.peek() {
            match token {
                Token::Null => {
                    values.push(JsonValue::Null);
                    token_iter.next();

                    Self::expect_comma(token_iter)?;
                },
                Token::True | Token::False => {
                    let value = matches!(token,Token::True);
                    values.push(JsonValue::Boolean(value));

                    token_iter.next();
                    Self::expect_comma(token_iter)?;
                },
                Token::Number(num_str) => {
                    values.push(JsonValue::Number(num_str.to_string()));

                    token_iter.next();
                    Self::expect_comma(token_iter)?;
                },
                Token::String(str) => {
                    values.push(JsonValue::String(str.to_string()));

                    token_iter.next();
                    Self::expect_comma(token_iter)?;
                },
                Token::LeftBrace => {
                    token_iter.next();

                    let json_result = Self::parse_object(token_iter)?;
                    values.push(json_result);

                    Self::expect_comma(token_iter)?;
                },
                Token::LeftBracket => {
                    token_iter.next();

                    let json_result = Self::parse_array(token_iter)?;
                    values.push(json_result);

                    Self::expect_comma(token_iter)?;
                },
                Token::RightBracket => {
                    token_iter.next();
                    return Ok(JsonValue::Array(values))
                },
                _ => return Err(JsonError::UnexpectedToken),
            }
        }
        
        Err(JsonError::UnexpectedToken)
    }

    fn parse_string(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        todo!()
    }

    fn parse_number(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        todo!()
    }

    fn parse_boolean(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        todo!()
    }

    pub fn parse(self) -> Result<JsonValue, JsonError> {
        let mut token_iter: Peekable<std::vec::IntoIter<Token>> = self.into_iter();
        let mut json_value: JsonValue = JsonValue::Null;

        while let Some(token) = token_iter.peek() {
            match token {
                Token::LeftBrace => {
                    token_iter.next();

                    let _res: Result<JsonValue, JsonError> = match token_iter.peek() {
                        Some(Token::Colon) => {
                            token_iter.next();
                            Self::parse_object(&mut token_iter)
                        }
                        _ => Err(JsonError::UnexpectedToken),
                    };
                }
                Token::LeftBracket => {
                    token_iter.next();

                    let _res = match token_iter.peek() {
                        Some(_token) => Self::parse_array(&mut token_iter),
                        _ => Err(JsonError::UnexpectedToken),
                    };
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

        Ok(JsonValue::Boolean(true))
    }
}
