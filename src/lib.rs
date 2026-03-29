use crate::{
    lexer::{Lexer, Token},
    parser::Parser,
};
use std::{collections::HashMap, fmt};

mod lexer;
mod parser;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Self::pretty_print(f, self, 0)
    }
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(str) => Some(str),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    fn pretty_print(
        f: &mut fmt::Formatter<'_>,
        json_value: &JsonValue,
        indent: usize,
    ) -> fmt::Result {
        let cur_indent = "    ".repeat(indent);
        let next_indent = "    ".repeat(indent + 1);

        match json_value {
            JsonValue::String(str) => write!(f, "\"{}\"", str),
            JsonValue::Number(num) => write!(f, "{}", num),
            JsonValue::Boolean(val) => write!(f, "{}", val),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Array(array) => {
                if array.is_empty() {
                    return write!(f, "[]");
                }

                write!(f, "[\n")?;
                for (i, val) in array.iter().enumerate() {
                    write!(f, "{next_indent}")?;
                    Self::pretty_print(f, val, indent + 1)?;
                    if i != array.len() - 1 {
                        write!(f, ",")?;
                    }
                    write!(f, "\n")?;
                }
                write!(f, "{cur_indent}")?;
                write!(f, "]")?;
                Ok(())
            }
            JsonValue::Object(object) => {
                if object.is_empty() {
                    return write!(f, "{{}}");
                }

                write!(f, "{{\n")?;
                for (i, record) in object.iter().enumerate() {
                    write!(f, "{next_indent}")?;
                    write!(f, "\"{}\": ", record.0)?;
                    Self::pretty_print(f, record.1, indent + 1)?;
                    if i != object.len() - 1 {
                        write!(f, ",")?;
                    }
                    write!(f, "\n")?;
                }
                write!(f, "{cur_indent}")?;
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken { line: usize, col: usize },
    UnexpectedEof,
    InvalidNumber(std::num::ParseFloatError),
    DuplicateKey(String),
}

impl std::error::Error for JsonError {}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken { line,col } => write!(f, "unexpeted token at line {line}, col {col}"),
            JsonError::UnexpectedEof => write!(f, "unexpected end of input"),
            JsonError::InvalidNumber(err) => write!(f, "invalid number : {err}"),
            JsonError::DuplicateKey(key) => write!(f, "Duplicate key : {key}"),
        }
    }
}

impl From<std::num::ParseFloatError> for JsonError {
    fn from(err: std::num::ParseFloatError) -> Self {
        JsonError::InvalidNumber(err)
    }
}

pub fn parse(input: &str) -> Result<JsonValue, JsonError> {
    let tokens: Vec<Token> = Lexer::new(input).into_tokens();
    let parser: Parser = Parser::new(tokens);

    Ok(parser.parse()?)
}

#[cfg(test)]
mod parser_tests {
    use crate::parse;

    #[test]
    fn parse_array() {
        let result = parse("[1,2,3]").expect("should parse array");
        assert_eq!(result.get_index(0).and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(result.get_index(1).and_then(|v| v.as_f64()), Some(2.0));
        assert_eq!(result.get_index(2).and_then(|v| v.as_f64()), Some(3.0));
    }

    #[test]
    fn parse_number() {
        let result = parse("123.45").expect("should parse number");
        assert_eq!(result.as_f64(), Some(123.45));
    }

    #[test]
    fn parse_string() {
        let result = parse(r#""hello""#).expect("should parse string");
        assert_eq!(result.as_str(), Some("hello"));
    }

    #[test]
    fn parse_object() {
        let result = parse(r#"{"name":"prudhvi"}"#).expect("should parse object");
        assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("prudhvi"));
    }

    #[test]
    fn parse_empty_array() {
        let result = parse("[]").expect("should parse empty array");
        assert_eq!(result.get_index(0), None);
    }

    #[test]
    fn parse_nested_json() {
        let result = parse(r#"{"a":[1,2,{"b":true}]}"#).expect("should parse nested json");
        let b = result
            .get("a")
            .and_then(|v| v.get_index(2))
            .and_then(|v| v.get("b"))
            .and_then(|v| v.as_bool());
        assert_eq!(b, Some(true));
    }

    #[test]
    fn parse_empty_object() {
        let result = parse("{}").expect("should parse empty object");
        assert_eq!(result.get("any"), None);
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
        let result = parse(input).expect("should parse complex json");
        assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("prudhvi"));
        assert_eq!(result.get("age").and_then(|v| v.as_f64()), Some(25.0));
        assert_eq!(
            result.get("is_student").and_then(|v| v.as_bool()),
            Some(false)
        );
        assert_eq!(
            result
                .get("skills")
                .and_then(|v| v.get_index(0))
                .and_then(|v| v.as_str()),
            Some("rust")
        );
        assert_eq!(
            result
                .get("address")
                .and_then(|v| v.get("city"))
                .and_then(|v| v.as_str()),
            Some("hyderabad")
        );
    }

    #[test]
    fn parse_invalid_json() {
        assert!(parse(r#"{"a":}"#).is_err());
    }

    #[test]
    fn parse_true() {
        let result = parse("true").expect("should parse true");
        assert_eq!(result.as_bool(), Some(true));
    }

    #[test]
    fn parse_false() {
        let result = parse("false").expect("should parse false");
        assert_eq!(result.as_bool(), Some(false));
    }

    #[test]
    fn parse_null() {
        let result = parse("null").expect("should parse null");
        assert!(result.is_null());
    }

    #[test]
    fn parse_nested_arrays() {
        let result = parse("[[1,2],[3,4]]").expect("should parse nested arrays");
        assert_eq!(
            result
                .get_index(0)
                .and_then(|v| v.get_index(0))
                .and_then(|v| v.as_f64()),
            Some(1.0)
        );
        assert_eq!(
            result
                .get_index(1)
                .and_then(|v| v.get_index(1))
                .and_then(|v| v.as_f64()),
            Some(4.0)
        );
    }

    #[test]
    fn parse_array_of_objects() {
        let result = parse(r#"[{"a":1},{"b":2}]"#).expect("should parse array of objects");
        assert_eq!(
            result
                .get_index(0)
                .and_then(|v| v.get("a"))
                .and_then(|v| v.as_f64()),
            Some(1.0)
        );
        assert_eq!(
            result
                .get_index(1)
                .and_then(|v| v.get("b"))
                .and_then(|v| v.as_f64()),
            Some(2.0)
        );
    }

    #[test]
    fn parse_mixed_type_array() {
        let result =
            parse(r#"[1,"hello",true,false,null]"#).expect("should parse mixed type array");
        assert_eq!(result.get_index(0).and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(result.get_index(1).and_then(|v| v.as_str()), Some("hello"));
        assert_eq!(result.get_index(2).and_then(|v| v.as_bool()), Some(true));
        assert_eq!(result.get_index(3).and_then(|v| v.as_bool()), Some(false));
        assert!(result.get_index(4).map(|v| v.is_null()).unwrap_or(false));
    }

    #[test]
    fn parse_deeply_nested_objects() {
        let result =
            parse(r#"{"a":{"b":{"c":"deep"}}}"#).expect("should parse deeply nested objects");
        assert_eq!(
            result
                .get("a")
                .and_then(|v| v.get("b"))
                .and_then(|v| v.get("c"))
                .and_then(|v| v.as_str()),
            Some("deep")
        );
    }

    #[test]
    fn parse_object_with_null_value() {
        let result = parse(r#"{"key":null}"#).expect("should parse object with null value");
        assert!(result.get("key").map(|v| v.is_null()).unwrap_or(false));
    }

    #[test]
    fn parse_unclosed_brace() {
        assert!(parse(r#"{"a":1"#).is_err());
    }

    #[test]
    fn parse_unclosed_bracket() {
        assert!(parse("[1,2,3").is_err());
    }

    #[test]
    fn test_display_null() {
        let result = parse("null").unwrap();
        assert_eq!(result.to_string(), "null");
    }

    #[test]
    fn test_display_boolean_true() {
        let result = parse("true").unwrap();
        assert_eq!(result.to_string(), "true");
    }

    #[test]
    fn test_display_boolean_false() {
        let result = parse("false").unwrap();
        assert_eq!(result.to_string(), "false");
    }

    #[test]
    fn test_display_number() {
        let result = parse("123").unwrap();
        assert_eq!(result.to_string(), "123");
    }

    #[test]
    fn test_display_string() {
        let result = parse(r#""hello""#).unwrap();
        println!("{result}");
        assert_eq!(result.to_string(), "\"hello\"");
    }

    #[test]
    fn test_display_empty_array() {
        let result = parse("[]").unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("["));
        assert!(output.contains("]"));
    }

    #[test]
    fn test_display_empty_object() {
        let result = parse("{}").unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("{"));
        assert!(output.contains("}"));
    }

    #[test]
    fn test_display_simple_array() {
        let result = parse("[1,2,3]").unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("3"));
    }

    #[test]
    fn test_display_simple_object() {
        let result = parse(r#"{"name":"john"}"#).unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("name"));
        assert!(output.contains("john"));
    }

    #[test]
    fn test_display_nested_object() {
        let result = parse(r#"{"a":{"b":"c"}}"#).unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("a"));
        assert!(output.contains("b"));
        assert!(output.contains("c"));
    }

    #[test]
    fn test_display_array_with_objects() {
        let result = parse(r#"[{"a":1},{"b":2}]"#).unwrap();
        println!("{result}");
        let output = result.to_string();
        assert!(output.contains("a"));
        assert!(output.contains("b"));
    }

    #[test]
    fn test_display_complex() {
        let input = r#"{"name":"prudhvi","age":25,"skills":["rust","python"]}"#;
        let result = parse(input).unwrap();
        println!("{result}");

        let output = result.to_string();

        assert!(output.contains("name"));
        assert!(output.contains("prudhvi"));
        assert!(output.contains("age"));
        assert!(output.contains("25"));
        assert!(output.contains("skills"));
        assert!(output.contains("rust"));
        assert!(output.contains("python"));
    }
}
