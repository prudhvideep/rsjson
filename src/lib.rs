use crate::{lexer::Lexer, parser::Parser};
use std::fmt;

mod lexer;
mod parser;
mod tape;

pub use tape::{Entry, EntryKind, Tape};

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken,
    UnexpectedEof,
    InvalidNumber(std::num::ParseFloatError),
    InvalidUtf8(std::str::Utf8Error),
    DuplicateKey(String),
}

impl std::error::Error for JsonError {}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken => write!(f, "unexpected token"),
            JsonError::UnexpectedEof => write!(f, "unexpected end of input"),
            JsonError::InvalidNumber(err) => write!(f, "invalid number : {err}"),
            JsonError::DuplicateKey(key) => write!(f, "Duplicate key : {key}"),
            JsonError::InvalidUtf8(key) => write!(f, "Invalid utf8 : {key}"),
        }
    }
}

impl From<std::num::ParseFloatError> for JsonError {
    fn from(err: std::num::ParseFloatError) -> Self {
        JsonError::InvalidNumber(err)
    }
}

impl From<std::str::Utf8Error> for JsonError {
    fn from(err: std::str::Utf8Error) -> Self {
        JsonError::InvalidUtf8(err)
    }
}

pub fn parse(input: &str) -> Result<Tape, JsonError> {
    let lexer = Lexer::new();
    let parser = Parser::new(input.as_bytes(), lexer);

    parser.parse()
}

#[cfg(test)]
mod parser_tests {

    use crate::parse;

    fn dump(input: &str) {
        let tape = parse(input).expect("parse failed");

        println!("\ninput: {input}");
        for (i, e) in tape.entries().iter().enumerate() {
            println!("  [{i:>3}] {:?} @ {}", e.kind, e.index);
        }
    }

    #[test]
    fn empty_object() {
        dump("{}");
    }

    #[test]
    fn empty_array() {
        dump("[]");
    }

    #[test]
    fn root_scalars() {
        dump("42");
        dump("-3.14e2");
        dump(r#""hello""#);
        dump("true");
        dump("false");
        dump("null");
    }

    #[test]
    fn flat_object() {
        dump(r#"{"name": "alice", "age": 30, "active": true, "extra": null}"#);
    }

    #[test]
    fn flat_array() {
        dump(r#"[1, "two", true, false, null]"#);
    }

    #[test]
    fn nested_objects() {
        dump(r#"{"outer": {"inner": {"deep": 123}}}"#);
    }

    #[test]
    fn nested_arrays() {
        dump("[[1, 2], [3, [4, 5]]]");
    }

    #[test]
    fn array_of_objects() {
        dump(r#"{"items": [{"id": 1}, {"id": 2}], "total": 2}"#);
    }

    #[test]
    fn escaped_strings() {
        dump(r#"{"text\"quoted\"": "line1\nline2\ttabbed \\ backslash", "unicode": "☃ ❤"}"#);
    }

    #[test]
    fn multiline_input() {
        dump(
            r#"{
                "id": 12345,
                "username": "alice",
                "tags": ["a", "b"],
                "meta": { "active": true, "score": 9.5 }
            }"#,
        );
    }

    #[test]
    fn real_world_blob() {
        dump(
            r#"{"a":[{"b":[{"c":[{"d":"end"}]}]}],"x":{"y":{"z":[{"k1":1},{"k2":[2,3,{"k3":"v3"}]}]}},"bools":[true,false,true],"nullish":null}"#,
        );
    }
}
