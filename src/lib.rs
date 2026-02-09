use crate::{lexer::Lexer, parser::Parser};
use std::collections::HashMap;

pub mod lexer;
pub mod parser;

#[derive(Debug, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Quote,
    String(String),
    Number(String),
    Colon,
    Comma,
    True,
    False,
    Null,
}

#[derive(Debug)]
pub struct Object(HashMap<String, JsonValue>);

#[derive(Debug)]
pub struct Array(Vec<JsonValue>);

#[derive(Debug)]
pub enum JsonValue {
    Object(Object),
    Array(Array),
    String,
    Number,
    Boolean,
    Null,
}

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken,
    UnexpectedEof,
    InvalidLiteral,
}

pub fn parse(input: &str) -> Result<JsonValue, JsonError> {
    let tokens: Vec<Token> = Lexer::new(input).into_tokens();
    let parser: Parser = Parser::new(tokens);

    let _json_value: Result<JsonValue, JsonError> = parser.parse();

    Ok(JsonValue::Null)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parser() {
        let input: &str = r##"{"a" : [1,2.9,"Prudhvi"]}"##;
        let _result: Result<crate::JsonValue, crate::JsonError> = crate::parse(input);

        ()
    }
}
