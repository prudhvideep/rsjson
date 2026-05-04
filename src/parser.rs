use crate::{
    lexer::{Lexer, Token, TokenKind},
    JsonError, JsonValue,
};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    input: &'a [u8],
    lexer: Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8], lexer: Lexer) -> Parser<'a> {
        Parser { input, lexer }
    }

    fn resolve_number(num_str: &[u8]) -> Result<f64, JsonError> {
        let num: f64 = std::str::from_utf8(num_str)?.parse()?;
        Ok(num)
    }

    fn resolve_string(token: &Token, input: &'a [u8]) -> &'a str {
        let start = token.start as usize;
        let end = token.end as usize;

        unsafe { std::str::from_utf8_unchecked(&input[start as usize..end as usize]) }
    }

    fn expect_colon(parser: &mut Parser<'a>) -> Result<(), JsonError> {
        let token = parser
            .lexer
            .next_token(parser.input)
            .ok_or(JsonError::UnexpectedEof)?;

        match token.kind {
            TokenKind::Colon => Ok(()),
            _ => Err(JsonError::UnexpectedToken {
                line: token.line as usize,
                col: token.col as usize,
            }),
        }
    }

    fn parse_string(token: &Token, input: &'a [u8]) -> Result<JsonValue, JsonError> {
        match token.kind {
            TokenKind::String => {
                let start = token.start as usize;
                let end = token.end as usize;

                Ok(JsonValue::String(
                    std::str::from_utf8(&input[start as usize..end as usize])
                        .unwrap()
                        .to_string(),
                ))
            }
            _ => Err(JsonError::UnexpectedToken {
                line: token.line as usize,
                col: token.col as usize,
            }),
        }
    }

    fn parse_number(token: &Token, input: &'a [u8]) -> Result<JsonValue, JsonError> {
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

    fn parse_array(parser: &mut Parser<'a>) -> Result<JsonValue, JsonError> {
        let mut values: Vec<JsonValue> = Vec::new();
        let mut last_seen_token: Option<TokenKind> = None;
        loop {
            let token = parser
                .lexer
                .next_token(parser.input)
                .ok_or(JsonError::UnexpectedEof)?;

            match token.kind {
                TokenKind::Null => values.push(JsonValue::Null),
                TokenKind::True => values.push(JsonValue::Boolean(true)),
                TokenKind::False => values.push(JsonValue::Boolean(false)),
                TokenKind::Number => {
                    let start = token.start as usize;
                    let end = token.end as usize;
                    values.push(JsonValue::Number(Self::resolve_number(
                        &parser.input[start..end],
                    )?))
                }
                TokenKind::String => {
                    let start = token.start as usize;
                    let end = token.end as usize;
                    values.push(JsonValue::String(
                        std::str::from_utf8(&parser.input[start as usize..end as usize])
                            .unwrap()
                            .to_string(),
                    ))
                }
                TokenKind::LeftBrace => values.push(Self::parse_object(parser)?),
                TokenKind::LeftBracket => values.push(Self::parse_array(parser)?),
                TokenKind::Comma => {
                    if let Some(TokenKind::Comma) = last_seen_token {
                        return Err(JsonError::UnexpectedToken {
                            line: token.line as usize,
                            col: token.col as usize,
                        });
                    }
                }
                TokenKind::RightBracket => break,
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        line: token.line as usize,
                        col: token.col as usize,
                    })
                }
            }

            last_seen_token = Some(token.kind);
        }

        Ok(JsonValue::Array(values))
    }

    fn parse_object(parser: &mut Parser<'a>) -> Result<JsonValue, JsonError> {
        let mut object: HashMap<String, JsonValue> = HashMap::new();
        println!("Inside parse object");
        loop {
            if parser.lexer.pos as usize > parser.input.len() {
                break;
            }

            let token = parser
                .lexer
                .next_token(parser.input)
                .ok_or(JsonError::UnexpectedEof)?;

            println!("Current token in parse object {:?}", token);
            match token.kind {
                TokenKind::String => {
                    let key = Self::resolve_string(&token, parser.input).to_string();
                    println!("Key in parse object {:?}", key);
                    if let Some(_object_key) = object.get(&key) {
                        return Err(JsonError::DuplicateKey(key.to_string()));
                    }

                    Self::expect_colon(parser)?;
                    let next_token = parser
                        .lexer
                        .next_token(parser.input)
                        .ok_or(JsonError::UnexpectedEof)?;

                    let value = match next_token.kind {
                        TokenKind::Null => JsonValue::Null,
                        TokenKind::String => {
                            let start = next_token.start as usize;
                            let end = next_token.end as usize;
                            JsonValue::String(
                                std::str::from_utf8(&parser.input[start as usize..end as usize])
                                    .unwrap()
                                    .to_string(),
                            )
                        }
                        TokenKind::Number => {
                            let start = next_token.start as usize;
                            let end = next_token.end as usize;
                            JsonValue::Number(Self::resolve_number(&parser.input[start..end])?)
                        }
                        TokenKind::True | TokenKind::False => {
                            JsonValue::Boolean(matches!(next_token.kind, TokenKind::True))
                        }
                        TokenKind::LeftBrace => Self::parse_object(parser)?,
                        TokenKind::LeftBracket => Self::parse_array(parser)?,
                        TokenKind::Comma => continue,
                        _ => {
                            return Err(JsonError::UnexpectedToken {
                                line: token.line as usize,
                                col: token.col as usize,
                            })
                        }
                    };
                    println!("Value in parse object {:?}", value);
                    object.insert(key.to_string(), value);
                }
                TokenKind::Comma => continue,
                TokenKind::RightBrace => break,
                _ => return Err(JsonError::UnexpectedEof),
            }
        }
        Ok(JsonValue::Object(object))
    }

    pub fn parse(mut self) -> Result<JsonValue, JsonError> {
        let token = self
            .lexer
            .next_token(self.input)
            .ok_or(JsonError::UnexpectedEof)?;
        let value = match token.kind {
            TokenKind::LeftBrace => Self::parse_object(&mut self)?,
            TokenKind::LeftBracket => Self::parse_array(&mut self)?,
            TokenKind::Number => Self::parse_number(&token, self.input)?,
            TokenKind::True | TokenKind::False => Self::parse_boolean(&token)?,
            TokenKind::String => Self::parse_string(&token, self.input)?,
            TokenKind::Null => JsonValue::Null,
            _ => {
                return Err(JsonError::UnexpectedToken {
                    line: token.line as usize,
                    col: token.col as usize,
                })
            }
        };
        Ok(value)
    }
}
