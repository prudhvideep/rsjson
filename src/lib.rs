use crate::{lexer::Lexer, parser::Parser};

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
pub enum JsonValue {
    JsonObject,
    JsonArray,
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
    let tokens = Lexer::new(input).into_tokens();
    let parser = Parser::new(tokens);

    let _json_value: Result<JsonValue, JsonError> = parser.parse();


    Ok(JsonValue::JsonArray)
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn test_parser() {
        let input = r##"{"a" : [1,2.9,"Prudhvi"]}"##;
        let _result: Result<crate::JsonValue, crate::JsonError> = parse(input);

        ()
    }
}
