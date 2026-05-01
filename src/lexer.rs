use std::iter::Peekable;

#[derive(Debug)]
pub(crate) struct Lexer {
    tokens: Vec<Token>,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub(crate) enum TokenKind {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    String,
    Number,
    Colon,
    Comma,
    True,
    False,
    Null,
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) line: u32,
    pub(crate) col: u32,
}

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        let mut start_pos = 0;
        let mut line = 1;
        let mut col = 1;

        while let Some(&c) = chars.peek() {
            match c {
                '\n' => {
                    chars.next();
                    start_pos += 1;
                    line += 1;
                    col = 1;
                }
                ' ' | '\t' => {
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                '{' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftBrace,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                '}' => {
                    tokens.push(Token {
                        kind: TokenKind::RightBrace,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                '[' => {
                    tokens.push(Token {
                        kind: TokenKind::LeftBracket,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                ']' => {
                    tokens.push(Token {
                        kind: TokenKind::RightBracket,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                ':' => {
                    tokens.push(Token {
                        kind: TokenKind::Colon,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                ',' => {
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        start: start_pos,
                        end: start_pos + 1,
                        line,
                        col,
                    });
                    chars.next();
                    start_pos += 1;
                    col += 1;
                }
                'n' => {
                    let init_pos = start_pos;
                    let col_pos: u32 = col;
                    if Self::match_keyword(&mut chars, "null", &mut start_pos, &mut col) {
                        tokens.push(Token {
                            kind: TokenKind::Null,
                            start: init_pos,
                            end: start_pos,
                            line,
                            col: col_pos,
                        });
                    }
                }
                't' => {
                    let init_pos: u32 = start_pos;
                    let col_pos: u32 = col;
                    if Self::match_keyword(&mut chars, "true", &mut start_pos, &mut col) {
                        tokens.push(Token {
                            kind: TokenKind::True,
                            start: init_pos,
                            end: start_pos,
                            line,
                            col: col_pos,
                        });
                    }
                }
                'f' => {
                    let init_pos: u32 = start_pos;
                    let col_pos: u32 = col;
                    if Self::match_keyword(&mut chars, "false", &mut start_pos, &mut col) {
                        tokens.push(Token {
                            kind: TokenKind::False,
                            start: init_pos,
                            end: start_pos,
                            line,
                            col: col_pos,
                        });
                    }
                }
                '"' => {
                    //Consume the quote
                    let col_pos: u32 = col;
                    let mut last_char: char = '\0';

                    chars.next();
                    col += 1;
                    start_pos += 1;
                    let init_pos: u32 = start_pos;
                    let mut content_end: u32 = start_pos;

                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            //Consume the quote
                            content_end = start_pos;
                            chars.next();
                            col += 1;
                            start_pos += 1;

                            if last_char != '\\' {
                                break;
                            } else {
                                let mut back_slash_count = 0;
                                for c in input[init_pos as usize..start_pos as usize].chars().rev()
                                {
                                    if c == '\\' {
                                        back_slash_count += 1;
                                    } else {
                                        break;
                                    }
                                }

                                if back_slash_count % 2 != 0 {
                                    continue;
                                } else {
                                    break;
                                }
                            }
                        }

                        last_char = c;
                        chars.next();
                        col += 1;
                        start_pos += 1;
                    }

                    tokens.push(Token {
                        kind: TokenKind::String,
                        start: init_pos,
                        end: content_end,
                        line,
                        col: col_pos,
                    });
                }
                '0'..='9' | '-' => {
                    let col_pos = col;
                    let init_pos = start_pos;
                    while let Some(&c) = chars.peek() {
                        match c {
                            '.' | '+' | '-' | 'E' | 'e' | '0'..='9' => {
                                chars.next();
                                col += 1;
                                start_pos += 1
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    tokens.push(Token {
                        kind: TokenKind::Number,
                        start: init_pos,
                        end: start_pos,
                        line,
                        col: col_pos,
                    });
                }
                _ => {
                    chars.next();
                    col += 1;
                    start_pos += 1;
                }
            }
        }

        Lexer { tokens }
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn match_keyword(
        chars: &mut Peekable<std::str::Chars>,
        keyword: &str,
        start_pos: &mut u32,
        col: &mut u32,
    ) -> bool {
        for (i, expected_char) in keyword.chars().enumerate() {
            if i == 0 {
                chars.next();
                *start_pos += 1;
                *col += 1;
                continue;
            }

            match chars.peek() {
                Some(&c) if c == expected_char => {
                    chars.next();
                    *start_pos += 1;
                    *col += 1;
                }
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        Lexer::new(input).into_tokens()
    }

    fn lexeme<'a>(input: &'a str, t: &Token) -> &'a str {
        &input[t.start as usize..t.end as usize]
    }

    fn kinds_eq(actual: &TokenKind, expected: &TokenKind) -> bool {
        std::mem::discriminant(actual) == std::mem::discriminant(expected)
    }

    #[test]
    fn empty_objects() {
        let tokens = tokenize("{}");
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn empty_array() {
        let tokens = tokenize("[]");
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn single_comma() {
        let tokens = tokenize(",");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Comma));
    }

    #[test]
    fn single_colon() {
        let tokens = tokenize(":");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Colon));
    }

    #[test]
    fn all_bracket_types() {
        let tokens = tokenize("{}[]");
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0].kind, TokenKind::LeftBrace));
        assert!(matches!(tokens[1].kind, TokenKind::RightBrace));
        assert!(matches!(tokens[2].kind, TokenKind::LeftBracket));
        assert!(matches!(tokens[3].kind, TokenKind::RightBracket));
    }

    #[test]
    fn keyword_true() {
        let tokens = tokenize("true");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::True));
    }

    #[test]
    fn all_keywords_together() {
        let tokens = tokenize("true false null");
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0].kind, TokenKind::True));
        assert!(matches!(tokens[1].kind, TokenKind::False));
        assert!(matches!(tokens[2].kind, TokenKind::Null));
    }

    #[test]
    fn simple_string() {
        let tokens = tokenize(r#""hello""#);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::String));
    }

    #[test]
    fn empty_string() {
        let tokens = tokenize(r#""""#);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::String));
    }

    #[test]
    fn string_with_spaces() {
        let tokens = tokenize(r#""hello world""#);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::String));
    }

    #[test]
    fn string_with_numbers() {
        let tokens = tokenize(r#""test123""#);
        assert!(matches!(tokens[0].kind, TokenKind::String));
    }

    #[test]
    fn multiple_strings() {
        let tokens = tokenize(r#""first" "second" "third""#);
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0].kind, TokenKind::String));
        assert!(matches!(tokens[1].kind, TokenKind::String));
        assert!(matches!(tokens[2].kind, TokenKind::String));
    }

    #[test]
    fn positive_integer() {
        let input = "42";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "42");
    }

    #[test]
    fn negative_integer() {
        let input = "-17";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "-17");
    }

    #[test]
    fn zero() {
        let input = "0";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "0");
    }

    #[test]
    fn decimal_number() {
        let input = "3.14";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "3.14");
    }

    #[test]
    fn negative_decimal() {
        let input = "-0.5";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "-0.5");
    }

    #[test]
    fn scientific_notation() {
        let input = "1e10";
        let tokens = tokenize(input);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "1e10");
    }

    #[test]
    fn scientific_negative_exponent() {
        let input = "2.5e-3";
        let tokens = tokenize(input);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "2.5e-3");
    }

    #[test]
    fn scientific_uppercase_e() {
        let input = "1E5";
        let tokens = tokenize(input);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "1E5");
    }

    #[test]
    fn simple_key_value_pair() {
        let tokens = tokenize(r#"{"key": "value"}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].kind, TokenKind::LeftBrace));
        assert!(matches!(tokens[2].kind, TokenKind::Colon));
        assert!(matches!(tokens[4].kind, TokenKind::RightBrace));
    }

    #[test]
    fn object_with_number_value() {
        let input = r#"{"age": 30}"#;
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[1].kind, TokenKind::String));
        assert!(matches!(tokens[3].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[3]), "30");
    }

    #[test]
    fn object_with_boolean() {
        let tokens = tokenize(r#"{"active": true}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[3].kind, TokenKind::True));
    }

    #[test]
    fn object_with_null() {
        let tokens = tokenize(r#"{"data": null}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[3].kind, TokenKind::Null));
    }

    #[test]
    fn object_with_multiple_pairs() {
        let tokens = tokenize(r#"{"a": 1, "b": 2, "c": 3}"#);
        assert_eq!(tokens.len(), 13);

        let comma_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::Comma))
            .count();
        assert_eq!(comma_count, 2);
    }

    #[test]
    fn array_of_numbers() {
        let tokens = tokenize("[1, 2, 3]");
        assert_eq!(tokens.len(), 7);
        assert!(matches!(tokens[0].kind, TokenKind::LeftBracket));
        assert!(matches!(tokens[6].kind, TokenKind::RightBracket));
    }

    #[test]
    fn array_of_strings() {
        let tokens = tokenize(r#"["a", "b", "c"]"#);
        assert_eq!(tokens.len(), 7);

        let string_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::String))
            .count();
        assert_eq!(string_count, 3);
    }

    #[test]
    fn array_of_mixed_types() {
        let tokens = tokenize(r#"[1, "two", true, null]"#);
        assert_eq!(tokens.len(), 9);

        assert!(matches!(tokens[1].kind, TokenKind::Number));
        assert!(matches!(tokens[3].kind, TokenKind::String));
        assert!(matches!(tokens[5].kind, TokenKind::True));
        assert!(matches!(tokens[7].kind, TokenKind::Null));
    }

    #[test]
    fn nested_objects() {
        let tokens = tokenize(r#"{"outer": {"inner": 123}}"#);

        let left_braces = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBrace))
            .count();
        let right_braces = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::RightBrace))
            .count();

        assert_eq!(left_braces, 2);
        assert_eq!(right_braces, 2);
    }

    #[test]
    fn nested_arrays() {
        let tokens = tokenize("[[1, 2], [3, 4]]");

        let left_brackets = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBracket))
            .count();
        let right_brackets = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::RightBracket))
            .count();

        assert_eq!(left_brackets, 3);
        assert_eq!(right_brackets, 3);
    }

    #[test]
    fn object_with_array_value() {
        let tokens = tokenize(r#"{"numbers": [1, 2, 3]}"#);

        assert!(tokens
            .iter()
            .any(|t| matches!(t.kind, TokenKind::LeftBrace)));
        assert!(tokens
            .iter()
            .any(|t| matches!(t.kind, TokenKind::LeftBracket)));
    }

    #[test]
    fn array_of_objects() {
        let tokens = tokenize(r#"[{"a": 1}, {"b": 2}]"#);

        let brace_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBrace))
            .count();
        assert_eq!(brace_count, 2);
    }

    #[test]
    fn whitespace_between_tokens() {
        let tokens = tokenize(r#"{  "key"  :  "value"  }"#);
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn multiline_json() {
        let input = r#"{
        "name": "Alice",
        "age": 30
    }"#;
        let tokens = tokenize(input);

        assert!(matches!(tokens[0].kind, TokenKind::LeftBrace));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Comma)));
    }

    #[test]
    fn tabs_and_newlines() {
        let tokens = tokenize("{\n\t\"key\":\t\"value\"\n}");
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn no_whitespace() {
        let tokens = tokenize(r#"{"a":1,"b":2}"#);
        assert_eq!(tokens.len(), 9);
    }

    #[test]
    fn deeply_nested_structure() {
        let tokens = tokenize(r#"{"l1": {"l2": {"l3": {"l4": 123}}}}"#);

        let brace_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBrace))
            .count();
        assert_eq!(brace_count, 4);
    }

    #[test]
    fn real_world_user_object() {
        let input = r#"{
        "id": 12345,
        "username": "alice",
        "email": "alice@example.com",
        "active": true,
        "balance": null
    }"#;

        let tokens = tokenize(input);

        let colon_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::Colon))
            .count();
        assert_eq!(colon_count, 5);

        let comma_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::Comma))
            .count();
        assert_eq!(comma_count, 4);
    }

    #[test]
    fn array_with_many_elements() {
        let tokens = tokenize("[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");

        let number_count = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::Number))
            .count();
        assert_eq!(number_count, 10);
    }

    #[test]
    fn balanced_braces_in_complex_json() {
        let input = r#"{"a": [{"b": 1}, {"c": 2}], "d": {"e": 3}}"#;
        let tokens = tokenize(input);

        let left_braces = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBrace))
            .count();
        let right_braces = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::RightBrace))
            .count();
        let left_brackets = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::LeftBracket))
            .count();
        let right_brackets = tokens
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::RightBracket))
            .count();

        assert_eq!(left_braces, right_braces);
        assert_eq!(left_brackets, right_brackets);
    }

    #[test]
    fn token_order_in_simple_object() {
        let tokens = tokenize(r#"{"key": "value"}"#);

        let expected = [
            TokenKind::LeftBrace,
            TokenKind::String,
            TokenKind::Colon,
            TokenKind::String,
            TokenKind::RightBrace,
        ];
        assert_eq!(tokens.len(), expected.len());
        for (t, e) in tokens.iter().zip(expected.iter()) {
            assert!(kinds_eq(&t.kind, e));
        }
    }

    #[test]
    fn large_number() {
        let input = "123456789";
        let tokens = tokenize(input);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "123456789");
    }

    #[test]
    fn decimal_with_many_digits() {
        let input = "3.141592653589793";
        let tokens = tokenize(input);
        assert!(matches!(tokens[0].kind, TokenKind::Number));
        assert_eq!(lexeme(input, &tokens[0]), "3.141592653589793");
    }

    #[test]
    fn deeply_nested_mixed_structure() {
        let input = r#"{"a":[{"b":[{"c":[{"d":[{"e":[{"f":"end"}]}]}]}]}],"x":{"y":{"z":[{"k1":1},{"k2":[2,3,{"k3":"v3"}]}]}},"misc":"\n\t\r\"\\","bools":[true,false,true],"nullish":null}"#;
        let tokens = tokenize(input);
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    #[test]
    fn escaped_quotes_and_special_chars() {
        let input = r#"{"text\"dsds\"": "Line1\nLine2\tTabbed \"quoted\" text \\ backslash", "unicode": "☃ ❤", "valid": true}"#;
        let tokens = tokenize(input);
        for token in &tokens {
            println!("{:?}", token);
        }
    }
}
