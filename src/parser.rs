use crate::{
    lexer::{Lexer, Token, TokenKind},
    tape::{
        EntryKind::{ArrayEnd, ArrayStart, False, Null, ObjectEnd, ObjectStart, True},
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

    #[inline(always)]
    fn advance_parser(&mut self) -> Result<Token, JsonError> {
        let token = self
            .lexer
            .next_token(self.input)
            .ok_or(JsonError::UnexpectedEof);

        token
    }

    #[inline(always)]
    fn expect_colon(parser: &mut Parser<'a>) -> Result<(), JsonError> {
        let token = parser.advance_parser()?;

        match token.kind {
            TokenKind::Colon => Ok(()),
            _ => Err(JsonError::UnexpectedToken),
        }
    }

    fn parse_array(parser: &mut Parser<'a>, tape: &mut Tape) -> Result<(), JsonError> {
        let mut last_seen_token: Option<TokenKind> = None;
        tape.add_entry(ArrayStart, parser.lexer.pos);

        loop {
            let token = parser.advance_parser()?;

            match token.kind {
                TokenKind::String => tape.add_string_markers(token.start, token.end),
                TokenKind::Number => tape.add_number_markers(token.start, token.end),
                TokenKind::Null => tape.add_entry(Null, token.start),
                TokenKind::True => tape.add_entry(True, token.start),
                TokenKind::False => tape.add_entry(False, token.start),
                TokenKind::LeftBrace => Self::parse_object(parser, tape)?,
                TokenKind::LeftBracket => Self::parse_array(parser, tape)?,
                TokenKind::Comma => {
                    if let Some(TokenKind::Comma) = last_seen_token {
                        return Err(JsonError::UnexpectedToken);
                    }
                }
                TokenKind::RightBracket => {
                    tape.add_entry(ArrayEnd, token.start);
                    break;
                }
                _ => return Err(JsonError::UnexpectedToken),
            }

            last_seen_token = Some(token.kind);
        }

        Ok(())
    }

    fn parse_object(parser: &mut Parser<'a>, tape: &mut Tape) -> Result<(), JsonError> {
        tape.add_entry(ObjectStart, parser.lexer.pos);

        loop {
            if parser.lexer.pos as usize > parser.input.len() {
                break;
            }

            let token: Token = parser.advance_parser()?;

            match token.kind {
                TokenKind::String => {
                    tape.add_string_markers(token.start, token.end);
                    Self::expect_colon(parser)?;
                    let next_token = parser.advance_parser()?;

                    match next_token.kind {
                        TokenKind::String => tape.add_string_markers(next_token.start, next_token.end),
                        TokenKind::Number => tape.add_number_markers(next_token.start, next_token.end),
                        TokenKind::Null => tape.add_entry(Null, next_token.start),
                        TokenKind::True => tape.add_entry(True, next_token.start),
                        TokenKind::False => tape.add_entry(False, next_token.start),
                        TokenKind::LeftBrace => Self::parse_object(parser, tape)?,
                        TokenKind::LeftBracket => Self::parse_array(parser, tape)?,
                        TokenKind::Comma => continue,
                        _ => return Err(JsonError::UnexpectedToken),
                    };
                }
                TokenKind::Comma => continue,
                TokenKind::RightBrace => {
                    tape.add_entry(ObjectEnd, token.start);
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
            TokenKind::True => tape.add_entry(True, token.start),
            TokenKind::False => tape.add_entry(False, token.start),
            TokenKind::Null => tape.add_entry(Null, token.start),
            TokenKind::Number => tape.add_number_markers(token.start, token.end),
            TokenKind::String => tape.add_string_markers(token.start, token.end),
            _ => return Err(JsonError::UnexpectedToken),
        };

        Ok(tape)
    }
}
