use crate::{
    lexer::{Lexer, Token, TokenKind},
    tape::{
        Entry,
        EntryKind::{
            ArrayEnd, ArrayStart, False, Null, NumberEnd, NumberStart, ObjectEnd, ObjectStart,
            StringEnd, StringStart, True,
        },
        Tape,
    },
    JsonError,
};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    input: &'a [u8],
    lexer: Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8], lexer: Lexer) -> Parser<'a> {
        Parser { input, lexer }
    }

    #[inline]
    fn advance_parser(&mut self) -> Result<Token, JsonError> {
        let token = self
            .lexer
            .next_token(self.input)
            .ok_or(JsonError::UnexpectedEof);

        token
    }

    fn expect_colon(parser: &mut Parser<'a>) -> Result<(), JsonError> {
        let token = parser.advance_parser()?;

        match token.kind {
            TokenKind::Colon => Ok(()),
            _ => Err(JsonError::UnexpectedToken {
                line: parser.lexer.last_token_line as usize,
                col: parser.lexer.last_token_col as usize,
            }),
        }
    }

    fn parse_array(parser: &mut Parser<'a>, tape: &mut Tape) -> Result<(), JsonError> {
        let mut last_seen_token: Option<TokenKind> = None;
        tape.add_entry(Entry {
            kind: ArrayStart,
            index: parser.lexer.pos,
        });

        loop {
            let token = parser.advance_parser()?;

            match token.kind {
                TokenKind::Number => {
                    tape.add_entry(Entry {
                        kind: NumberStart,
                        index: token.start,
                    });
                    tape.add_entry(Entry {
                        kind: NumberEnd,
                        index: token.end,
                    })
                }
                TokenKind::String => {
                    tape.add_entry(Entry {
                        kind: StringStart,
                        index: token.start,
                    });
                    tape.add_entry(Entry {
                        kind: StringEnd,
                        index: token.end,
                    });
                }
                TokenKind::Null => tape.add_entry(Entry {
                    kind: Null,
                    index: token.start,
                }),
                TokenKind::True => tape.add_entry(Entry {
                    kind: True,
                    index: token.start,
                }),
                TokenKind::False => tape.add_entry(Entry {
                    kind: False,
                    index: token.start,
                }),
                TokenKind::LeftBrace => Self::parse_object(parser, tape)?,
                TokenKind::LeftBracket => Self::parse_array(parser, tape)?,
                TokenKind::Comma => {
                    if let Some(TokenKind::Comma) = last_seen_token {
                        return Err(JsonError::UnexpectedToken {
                            line: parser.lexer.last_token_line as usize,
                            col: parser.lexer.last_token_col as usize,
                        });
                    }
                }
                TokenKind::RightBracket => {
                    tape.add_entry(Entry {
                        kind: ArrayEnd,
                        index: token.start,
                    });
                    break;
                }
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        line: parser.lexer.last_token_line as usize,
                        col: parser.lexer.last_token_col as usize,
                    })
                }
            }

            last_seen_token = Some(token.kind);
        }

        Ok(())
    }

    fn parse_object(parser: &mut Parser<'a>, tape: &mut Tape) -> Result<(), JsonError> {
        tape.add_entry(Entry {
            kind: ObjectStart,
            index: parser.lexer.pos,
        });

        loop {
            if parser.lexer.pos as usize > parser.input.len() {
                break;
            }

            let token: Token = parser.advance_parser()?;

            match token.kind {
                TokenKind::String => {
                    tape.add_entry(Entry {
                        kind: StringStart,
                        index: token.start,
                    });
                    tape.add_entry(Entry {
                        kind: StringEnd,
                        index: token.end,
                    });

                    Self::expect_colon(parser)?;

                    let next_token = parser.advance_parser()?;

                    match next_token.kind {
                        TokenKind::String => {
                            tape.add_entry(Entry {
                                kind: StringStart,
                                index: next_token.start,
                            });
                            tape.add_entry(Entry {
                                kind: StringEnd,
                                index: next_token.end,
                            })
                        }
                        TokenKind::Number => {
                            tape.add_entry(Entry {
                                kind: NumberStart,
                                index: next_token.start,
                            });
                            tape.add_entry(Entry {
                                kind: NumberEnd,
                                index: next_token.end,
                            })
                        }
                        TokenKind::Null => tape.add_entry(Entry {
                            kind: Null,
                            index: next_token.start,
                        }),
                        TokenKind::True => tape.add_entry(Entry {
                            kind: True,
                            index: next_token.start,
                        }),
                        TokenKind::False => tape.add_entry(Entry {
                            kind: False,
                            index: next_token.start,
                        }),
                        TokenKind::LeftBrace => Self::parse_object(parser, tape)?,
                        TokenKind::LeftBracket => Self::parse_array(parser, tape)?,
                        TokenKind::Comma => continue,
                        _ => {
                            return Err(JsonError::UnexpectedToken {
                                line: parser.lexer.last_token_line as usize,
                                col: parser.lexer.last_token_col as usize,
                            })
                        }
                    };
                }

                TokenKind::Comma => continue,
                TokenKind::RightBrace => {
                    tape.add_entry(Entry {
                        kind: ObjectEnd,
                        index: token.start,
                    });
                    break;
                }
                _ => return Err(JsonError::UnexpectedEof),
            }
        }

        Ok(())
    }

    pub fn parse(mut self) -> Result<Tape, JsonError> {
        let mut tape = Tape::with_capacity(self.input.len());

        let token = self.advance_parser()?;

        match token.kind {
            TokenKind::LeftBrace => Self::parse_object(&mut self, &mut tape)?,
            TokenKind::LeftBracket => Self::parse_array(&mut self, &mut tape)?,
            TokenKind::True => tape.add_entry(Entry {
                kind: True,
                index: token.start,
            }),
            TokenKind::False => tape.add_entry(Entry {
                kind: False,
                index: token.start,
            }),
            TokenKind::Null => tape.add_entry(Entry {
                kind: Null,
                index: token.start,
            }),
            TokenKind::Number => {
                tape.add_entry(Entry {
                    kind: NumberStart,
                    index: token.start,
                });
                tape.add_entry(Entry {
                    kind: NumberEnd,
                    index: token.end,
                });
            }
            TokenKind::String => {
                tape.add_entry(Entry {
                    kind: StringStart,
                    index: token.start,
                });
                tape.add_entry(Entry {
                    kind: StringEnd,
                    index: token.end,
                });
            }
            _ => {
                return Err(JsonError::UnexpectedToken {
                    line: self.lexer.last_token_line as usize,
                    col: self.lexer.last_token_col as usize,
                })
            }
        };

        Ok(tape)
    }
}
