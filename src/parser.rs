use crate::{lexer::Token, JsonError, JsonValue};
use std::{collections::HashMap, iter::Peekable};

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<Token>,
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
        Parser { tokens }
    }

    fn resolve_number(num_str: &str) -> Result<f64, JsonError> {
        let num = num_str.parse()?;
        Ok(num)
    }

    fn expect_colon(token_iter: &mut Peekable<std::vec::IntoIter<Token>>) -> Result<(), JsonError> {
        if let Some(Token::Colon) = token_iter.peek() {
            token_iter.next();
            Ok(())
        } else {
            Err(JsonError::UnexpectedToken)
        }
    }

    fn parse_string(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;

        match token {
            Token::String(str) => return Ok(JsonValue::String(str.to_string())),
            _ => return Err(JsonError::UnexpectedToken),
        }
    }

    fn parse_number(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;

        match token {
            Token::Number(num) => return Ok(JsonValue::Number(Self::resolve_number(&num)?)),
            _ => Err(JsonError::UnexpectedToken),
        }
    }

    fn parse_boolean(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;

        match token {
            Token::True => Ok(JsonValue::Boolean(true)),
            Token::False => Ok(JsonValue::Boolean(false)),
            _ => Err(JsonError::UnexpectedToken),
        }
    }

    fn parse_array(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let mut values: Vec<JsonValue> = Vec::new();

        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;
            match token {
                Token::Null => values.push(JsonValue::Null),
                Token::True => values.push(JsonValue::Boolean(true)),
                Token::False => values.push(JsonValue::Boolean(false)),
                Token::Number(num_str) => {
                    values.push(JsonValue::Number(Self::resolve_number(&num_str)?))
                }
                Token::String(str) => values.push(JsonValue::String(str.to_string())),
                Token::LeftBrace => values.push(Self::parse_object(token_iter)?),
                Token::LeftBracket => values.push(Self::parse_array(token_iter)?),
                Token::RightBracket => {
                    break;
                }
                Token::Comma => {
                    token_iter.next();
                }
                _ => return Err(JsonError::UnexpectedToken),
            }

            match token_iter.peek() {
                Some(Token::Comma) => {
                    token_iter.next();
                }
                Some(Token::RightBracket) => {
                    token_iter.next();
                    break;
                }
                _ => return Err(JsonError::UnexpectedToken),
            }
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_object(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let mut object: HashMap<String, JsonValue> = HashMap::new();

        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;

            match token {
                Token::String(str) => {
                    let key = str.to_string();
                    Self::expect_colon(token_iter)?;
                    let next_token = token_iter.next().ok_or(JsonError::UnexpectedToken)?;
                    let value = match next_token {
                        Token::Null => JsonValue::Null,
                        Token::String(str) => JsonValue::String(str),
                        Token::Number(num_str) => JsonValue::Number(Self::resolve_number(&num_str)?),
                        Token::True | Token::False => {
                            JsonValue::Boolean(matches!(next_token, Token::True))
                        }
                        Token::LeftBrace => Self::parse_object(token_iter)?,
                        Token::LeftBracket => Self::parse_array(token_iter)?,
                        _ => return Err(JsonError::UnexpectedToken),
                    };
                    object.insert(key, value);

                    match token_iter.peek() {
                        Some(Token::Comma) => {
                            token_iter.next();
                        }
                        Some(Token::RightBrace) => {
                            token_iter.next();
                            break;
                        }
                        _ => return Err(JsonError::UnexpectedToken),
                    }
                }
                Token::RightBrace => break,
                _ => return Err(JsonError::UnexpectedToken),
            }
        }
        Ok(JsonValue::Object(object))
    }

    pub fn parse(self) -> Result<JsonValue, JsonError> {
        let mut value = JsonValue::Null;
        let mut token_iter: Peekable<std::vec::IntoIter<Token>> = self.into_iter();

        while let Some(token) = token_iter.peek() {
            match token {
                Token::LeftBrace => {
                    token_iter.next();
                    value = Self::parse_object(&mut token_iter)?;
                }
                Token::LeftBracket => {
                    token_iter.next();
                    value = Self::parse_array(&mut token_iter)?;
                }
                Token::Number(_number) => {
                    value = Self::parse_number(&mut token_iter)?;
                }
                Token::False | Token::True => {
                    value = Self::parse_boolean(&mut token_iter)?;
                }
                Token::String(_str) => {
                    value = Self::parse_string(&mut token_iter)?;
                }
                Token::Null => {
                    token_iter.next();
                    value = JsonValue::Null;
                }
                _ => {
                    return Err(JsonError::UnexpectedToken);
                }
            }
        }
        Ok(value)
    }
}
