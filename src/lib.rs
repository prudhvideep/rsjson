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
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(String),
    Boolean(bool),
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

    Ok(parser.parse()?)
}

#[cfg(test)]
mod parser_tests {
    use crate::{JsonValue, Lexer, Parser};

    #[test]
    fn parse_array() {
        let input = "[1,2,3]";

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse array");

        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn parse_number() {
        let input = "123.45";

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse number");

        assert!(matches!(result, JsonValue::Number(_)));
    }

    #[test]
    fn parse_string() {
        let input = r#""hello""#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse string");

        assert!(matches!(result, JsonValue::String(_)));
    }

    #[test]
    fn parse_object() {
        let input = r#"{"name":"prudhvi"}"#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse object");

        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_empty_array() {
        let input = "[]";

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse empty array");

        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn parse_nested_json() {
        let input = r#"{"a":[1,2,{"b":true}]}"#;

        let tokens = Lexer::new(input).into_tokens();
        dbg!(tokens);

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse nested json");

        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_empty_object() {
        let input = "{}";

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse empty object");

        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_complex_json() {
        let input = r#"
    {
        "name": "prudhvi",
        "age": 25,
        "is_student": false,
        "skills": ["rust", "c++", "python"],
        "address": {
            "city": "hyderabad"
        }
    }
    "#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse complex json");

        dbg!(&result);
        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_invalid_json() {
        let input = r#"{"a":}"#;

        let result = Parser::new(Lexer::new(input).into_tokens()).parse();

        assert!(result.is_err());
    }

    #[test]
    fn parse_true() {
        let result = Parser::new(Lexer::new("true").into_tokens())
            .parse()
            .expect("should parse true");

        assert!(matches!(result, JsonValue::Boolean(true)));
    }

    #[test]
    fn parse_false() {
        let result = Parser::new(Lexer::new("false").into_tokens())
            .parse()
            .expect("should parse false");

        assert!(matches!(result, JsonValue::Boolean(false)));
    }

    #[test]
    fn parse_null() {
        let input = "null";

        let tokens = Lexer::new(input).into_tokens();
        dbg!("{:?}",tokens);

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse null");

        assert!(matches!(result, JsonValue::Null));
    }

    #[test]
    fn parse_nested_arrays() {
        let input = "[[1,2],[3,4]]";

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse nested arrays");

        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn parse_array_of_objects() {
        let input = r#"[{"a":1},{"b":2}]"#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse array of objects");

        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn parse_mixed_type_array() {
        let input = r#"[1,"hello",true,false,null]"#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse mixed type array");

        assert!(matches!(result, JsonValue::Array(_)));
    }

    #[test]
    fn parse_deeply_nested_objects() {
        let input = r#"{"a":{"b":{"c":"deep"}}}"#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse deeply nested objects");

        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_object_with_null_value() {
        let input = r#"{"key":null}"#;

        let result = Parser::new(Lexer::new(input).into_tokens())
            .parse()
            .expect("should parse object with null value");

        assert!(matches!(result, JsonValue::Object(_)));
    }

    #[test]
    fn parse_unclosed_brace() {
        let input = r#"{"a":1"#;

        let result = Parser::new(Lexer::new(input).into_tokens()).parse();

        assert!(result.is_err());
    }

    #[test]
    fn parse_unclosed_bracket() {
        let input = "[1,2,3";

        let result = Parser::new(Lexer::new(input).into_tokens()).parse();

        assert!(result.is_err());
    }
}
