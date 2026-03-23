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
        match token_iter.peek() {
            Some(Token::Colon(_)) => {
                token_iter.next();
                Ok(())
            }
            Some(token) => {
                let (line, col) = token.span();
                Err(JsonError::UnexpectedToken { line, col })
            }
            None => Err(JsonError::UnexpectedEof),
        }
    }

    fn parse_string(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;

        match token {
            Token::String(str, _) => return Ok(JsonValue::String(str.to_string())),
            _ => {
                let (line, col) = token.span();
                return Err(JsonError::UnexpectedToken { line, col });
            }
        }
    }

    fn parse_number(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
        let (line, col) = token.span();
        match token {
            Token::Number(num, _) => return Ok(JsonValue::Number(Self::resolve_number(&num)?)),
            _ => Err(JsonError::UnexpectedToken { line, col }),
        }
    }

    fn parse_boolean(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
        let (line, col) = token.span();
        match token {
            Token::True(_) => Ok(JsonValue::Boolean(true)),
            Token::False(_) => Ok(JsonValue::Boolean(false)),
            _ => Err(JsonError::UnexpectedToken { line, col }),
        }
    }

    fn parse_array(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let mut values: Vec<JsonValue> = Vec::new();

        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
            let (line, col) = token.span();
            match token {
                Token::Null(_) => values.push(JsonValue::Null),
                Token::True(_) => values.push(JsonValue::Boolean(true)),
                Token::False(_) => values.push(JsonValue::Boolean(false)),
                Token::Number(num_str, _) => {
                    values.push(JsonValue::Number(Self::resolve_number(&num_str)?))
                }
                Token::String(str, _) => values.push(JsonValue::String(str.to_string())),
                Token::LeftBrace(_) => values.push(Self::parse_object(token_iter)?),
                Token::LeftBracket(_) => values.push(Self::parse_array(token_iter)?),
                Token::RightBracket(_) => {
                    break;
                }
                Token::Comma(_) => {
                    token_iter.next();
                }
                _ => return Err(JsonError::UnexpectedToken { line, col }),
            }

            match token_iter.peek() {
                Some(Token::Comma(_)) => {
                    token_iter.next();
                }
                Some(Token::RightBracket(_)) => {
                    token_iter.next();
                    break;
                }
                Some(token) => {
                    let (line, col) = token.span();
                    return Err(JsonError::UnexpectedToken { line, col });
                }
                _ => return Err(JsonError::UnexpectedEof),
            }
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_object(
        token_iter: &mut Peekable<std::vec::IntoIter<Token>>,
    ) -> Result<JsonValue, JsonError> {
        let mut object: HashMap<String, JsonValue> = HashMap::new();

        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
            let (line, col) = token.span();
            match token {
                Token::String(str, _) => {
                    let key = str.to_string();
                    Self::expect_colon(token_iter)?;
                    let next_token = token_iter
                        .next()
                        .ok_or(JsonError::UnexpectedToken { line, col })?;
                    let value = match next_token {
                        Token::Null(_) => JsonValue::Null,
                        Token::String(str, _) => JsonValue::String(str),
                        Token::Number(num_str, _) => {
                            JsonValue::Number(Self::resolve_number(&num_str)?)
                        }
                        Token::True(_) | Token::False(_) => {
                            JsonValue::Boolean(matches!(next_token, Token::True(_)))
                        }
                        Token::LeftBrace(_) => Self::parse_object(token_iter)?,
                        Token::LeftBracket(_) => Self::parse_array(token_iter)?,
                        _ => return Err(JsonError::UnexpectedToken { line, col }),
                    };
                    object.insert(key, value);

                    match token_iter.peek() {
                        Some(Token::Comma(_)) => {
                            token_iter.next();
                        }
                        Some(Token::RightBrace(_)) => {
                            token_iter.next();
                            break;
                        }
                        Some(token) => {
                            let (line, col) = token.span();
                            return Err(JsonError::UnexpectedToken { line, col });
                        }
                        _ => return Err(JsonError::UnexpectedEof),
                    }
                }
                Token::RightBrace(_) => break,
                _ => return Err(JsonError::UnexpectedEof),
            }
        }
        Ok(JsonValue::Object(object))
    }

    pub fn parse(self) -> Result<JsonValue, JsonError> {
        let mut value = JsonValue::Null;
        let mut token_iter: Peekable<std::vec::IntoIter<Token>> = self.into_iter();

        while let Some(token) = token_iter.peek() {
            match token {
                Token::LeftBrace(_) => {
                    token_iter.next();
                    value = Self::parse_object(&mut token_iter)?;
                }
                Token::LeftBracket(_) => {
                    token_iter.next();
                    value = Self::parse_array(&mut token_iter)?;
                }
                Token::Number(_, _) => {
                    value = Self::parse_number(&mut token_iter)?;
                }
                Token::False(_) | Token::True(_) => {
                    value = Self::parse_boolean(&mut token_iter)?;
                }
                Token::String(_, _) => {
                    value = Self::parse_string(&mut token_iter)?;
                }
                Token::Null(_) => {
                    token_iter.next();
                    value = JsonValue::Null;
                }
                _ => {
                    let (line, col) = token.span();
                    return Err(JsonError::UnexpectedToken { line, col });
                }
            }
        }
        Ok(value)
    }
}
