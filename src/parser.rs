use crate::{
    lexer::{Token, TokenKind},
    JsonError, JsonValue,
};
use std::{collections::HashMap, iter::Peekable};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    input: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, tokens: Vec<Token>) -> Parser<'a> {
        Parser { input, tokens }
    }

    fn resolve_number(num_str: &str) -> Result<f64, JsonError> {
        let num = num_str.parse()?;
        Ok(num)
    }

    fn resolve_string(token: &Token, input: &'a str) -> &'a str {
        let start = token.start as usize;
        let end = token.end as usize;

        &input[start..end]
    }

    fn expect_colon(
        token_iter: &mut Peekable<std::slice::Iter<'_, Token>>,
    ) -> Result<(), JsonError> {
        match token_iter.peek() {
            Some(&token) => match token.kind {
                TokenKind::Colon => {
                    token_iter.next();
                    Ok(())
                }
                _ => Err(JsonError::UnexpectedToken {
                    line: token.line as usize,
                    col: token.col as usize,
                }),
            },
            None => Err(JsonError::UnexpectedEof),
        }
    }

    fn parse_string(token: &Token, input: &'a str) -> Result<JsonValue, JsonError> {
        match token.kind {
            TokenKind::String => {
                let start = token.start as usize;
                let end = token.end as usize;

                Ok(JsonValue::String(input[start..end].to_string()))
            }
            _ => Err(JsonError::UnexpectedToken {
                line: token.line as usize,
                col: token.col as usize,
            }),
        }
    }

    fn parse_number(token: &Token, input: &'a str) -> Result<JsonValue, JsonError> {
        match token.kind {
            TokenKind::Number => {
                let start = token.start as usize;
                let end = token.end as usize;

                Ok(JsonValue::Number(Self::resolve_number(&input[start..end])?))
            }
            _ => Err(JsonError::UnexpectedToken {
                line: token.line as usize,
                col: token.col as usize,
            }),
        }
    }

    fn parse_boolean(token: &Token) -> Result<JsonValue, JsonError> {
        match token.kind {
            TokenKind::True => Ok(JsonValue::Boolean(true)),
            TokenKind::False => Ok(JsonValue::Boolean(false)),
            _ => Err(JsonError::UnexpectedToken {
                line: token.line as usize,
                col: token.col as usize,
            }),
        }
    }

    fn parse_array(
        token_iter: &mut Peekable<std::slice::Iter<'_, Token>>,
        input: &'a str,
    ) -> Result<JsonValue, JsonError> {
        let mut values: Vec<JsonValue> = Vec::new();
        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
            match token.kind {
                TokenKind::Null => values.push(JsonValue::Null),
                TokenKind::True => values.push(JsonValue::Boolean(true)),
                TokenKind::False => values.push(JsonValue::Boolean(false)),
                TokenKind::Number => {
                    let start = token.start as usize;
                    let end = token.end as usize;
                    values.push(JsonValue::Number(Self::resolve_number(&input[start..end])?))
                }
                TokenKind::String => {
                    let start = token.start as usize;
                    let end = token.end as usize;
                    values.push(JsonValue::String(input[start..end].to_string()))
                }
                TokenKind::LeftBrace => values.push(Self::parse_object(token_iter, input)?),
                TokenKind::LeftBracket => values.push(Self::parse_array(token_iter, input)?),
                TokenKind::RightBracket => {
                    break;
                }
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        line: token.line as usize,
                        col: token.col as usize,
                    })
                }
            }

            match token_iter.peek() {
                Some(token) => match token.kind {
                    TokenKind::Comma => {
                        token_iter.next();
                    }
                    TokenKind::RightBracket => {
                        token_iter.next();
                        break;
                    }
                    _ => {
                        return Err(JsonError::UnexpectedToken {
                            line: token.line as usize,
                            col: token.col as usize,
                        });
                    }
                },
                None => {
                    return Err(JsonError::UnexpectedEof);
                }
            }
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_object(
        token_iter: &mut Peekable<std::slice::Iter<'_, Token>>,
        input: &'a str,
    ) -> Result<JsonValue, JsonError> {
        let mut object: HashMap<String, JsonValue> = HashMap::new();
        loop {
            let token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;
            match token.kind {
                TokenKind::String => {
                    let key = Self::resolve_string(token, input);

                    if let Some(_object_key) = object.get(key) {
                        return Err(JsonError::DuplicateKey(key.to_string()));
                    }

                    Self::expect_colon(token_iter)?;
                    let next_token = token_iter.next().ok_or(JsonError::UnexpectedEof)?;

                    let value = match next_token.kind {
                        TokenKind::Null => JsonValue::Null,
                        TokenKind::String => {
                            let start = next_token.start as usize;
                            let end = next_token.end as usize;
                            JsonValue::String(input[start..end].to_string())
                        }
                        TokenKind::Number => {
                            let start = next_token.start as usize;
                            let end = next_token.end as usize;
                            JsonValue::Number(Self::resolve_number(&input[start..end])?)
                        }
                        TokenKind::True | TokenKind::False => {
                            JsonValue::Boolean(matches!(next_token.kind, TokenKind::True))
                        }
                        TokenKind::LeftBrace => Self::parse_object(token_iter, input)?,
                        TokenKind::LeftBracket => Self::parse_array(token_iter, input)?,
                        _ => {
                            return Err(JsonError::UnexpectedToken {
                                line: token.line as usize,
                                col: token.col as usize,
                            })
                        }
                    };
                    object.insert(key.to_string(), value);

                    match token_iter.peek() {
                        Some(&token) => match token.kind {
                            TokenKind::Comma => {
                                token_iter.next();
                            }
                            TokenKind::RightBrace => {
                                token_iter.next();
                                break;
                            }
                            _ => {
                                return Err(JsonError::UnexpectedToken {
                                    line: token.line as usize,
                                    col: token.col as usize,
                                });
                            }
                        },
                        None => return Err(JsonError::UnexpectedEof),
                    }
                }

                TokenKind::RightBrace => break,
                _ => return Err(JsonError::UnexpectedEof),
            }
        }
        Ok(JsonValue::Object(object))
    }

    pub fn parse(self) -> Result<JsonValue, JsonError> {
        let mut value = JsonValue::Null;
        let mut iter = self.tokens.iter().peekable();

        while let Some(token) = iter.next() {
            match token.kind {
                TokenKind::LeftBrace => {
                    value = Self::parse_object(&mut iter, &self.input)?;
                }
                TokenKind::LeftBracket => {
                    value = Self::parse_array(&mut iter, &self.input)?;
                }
                TokenKind::Number => {
                    value = Self::parse_number(&token, self.input)?;
                }
                TokenKind::False | TokenKind::True => {
                    value = Self::parse_boolean(&token)?;
                }
                TokenKind::String => {
                    value = Self::parse_string(&token, self.input)?;
                }
                TokenKind::Null => {
                    iter.next();
                    value = JsonValue::Null;
                }
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        line: token.line as usize,
                        col: token.col as usize,
                    });
                }
            }
        }
        Ok(value)
    }
}
