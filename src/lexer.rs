#[derive(Debug)]
pub(crate) struct Lexer {
    pub(crate) pos: u32,
    pub(crate) line: u32,
    pub(crate) col: u32,
    pub(crate) prev_token_line: u32,
    pub(crate) prev_token_col: u32,
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
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            pos: 0,
            line: 1,
            col: 1,
            prev_token_line: 1,
            prev_token_col: 1,
        }
    }

    pub fn next_token(&mut self, input: &[u8]) -> Option<Token> {
        loop {
            if self.pos as usize >= input.len() {
                return None;
            }

            self.prev_token_col = self.col;
            self.prev_token_line = self.line;

            match input[self.pos as usize] {
                b'\n' => {
                    self.pos += 1;
                    self.line += 1;
                    self.col = 1;

                    continue;
                }
                b' ' | b'\t' => {
                    self.pos += 1;
                    self.col += 1;

                    continue;
                }
                b'{' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::LeftBrace,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b'}' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::RightBrace,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b'[' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::LeftBracket,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b']' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::RightBracket,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b':' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::Colon,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b',' => {
                    let init_pos = self.pos;

                    self.pos += 1;
                    self.col += 1;

                    return Some(Token {
                        kind: TokenKind::Comma,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                b'n' => {
                    let init_pos = self.pos;

                    if &input[self.pos as usize..self.pos as usize + 4] == b"null" {
                        self.pos += 4;
                        self.col += 4;

                        return Some(Token {
                            kind: TokenKind::Null,
                            start: init_pos,
                            end: self.pos,
                        });
                    } else {
                        self.pos += 1;
                        self.col += 1;
                    }
                }
                b't' => {
                    let init_pos: u32 = self.pos;

                    if &input[self.pos as usize..self.pos as usize + 4] == b"true" {
                        self.pos += 4;
                        self.col += 4;

                        return Some(Token {
                            kind: TokenKind::True,
                            start: init_pos,
                            end: self.pos,
                        });
                    } else {
                        self.pos += 1;
                        self.col += 1;
                    }
                }
                b'f' => {
                    let init_pos: u32 = self.pos;

                    if &input[self.pos as usize..self.pos as usize + 5] == b"false" {
                        self.pos += 5;
                        self.col += 5;

                        return Some(Token {
                            kind: TokenKind::False,
                            start: init_pos,
                            end: self.pos,
                        });
                    } else {
                        self.pos += 1;
                        self.col += 1;
                    }
                }
                b'"' => {
                    //Consume the quote
                    self.col += 1;
                    self.pos += 1;

                    let mut str_end: u32;
                    let mut last_byte: u8 = b'\0';
                    let str_start: u32 = self.pos;

                    loop {
                        if input[self.pos as usize] == b'"' {
                            //Consume the quote
                            str_end = self.pos;
                            self.col += 1;
                            self.pos += 1;

                            if last_byte != b'\\' {
                                break;
                            } else {
                                let mut back_slash_count: i32 = 0;
                                let mut last_byte_idx: u32 = self.pos - 2;

                                loop {
                                    if input[last_byte_idx as usize] == b'\\' {
                                        back_slash_count += 1;
                                    } else {
                                        break;
                                    }

                                    last_byte_idx -= 1;
                                }

                                if back_slash_count % 2 != 0 {
                                    continue;
                                } else {
                                    break;
                                }
                            }
                        }

                        last_byte = input[self.pos as usize];
                        self.col += 1;
                        self.pos += 1;
                    }

                    return Some(Token {
                        kind: TokenKind::String,
                        start: str_start,
                        end: str_end,
                    });
                }
                b'0'..=b'9' | b'-' => {
                    let init_pos: u32 = self.pos;
                    loop {
                        if self.pos as usize >= input.len() {
                            break;
                        }

                        match input[self.pos as usize] {
                            b'.' | b'+' | b'-' | b'E' | b'e' | b'0'..=b'9' => {
                                self.col += 1;
                                self.pos += 1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    return Some(Token {
                        kind: TokenKind::Number,
                        start: init_pos,
                        end: self.pos,
                    });
                }
                _ => {
                    self.col += 1;
                    self.pos += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let bytes = input.as_bytes();
        let mut lexer = Lexer::new();
        let mut tokens = Vec::new();
        while (lexer.pos as usize) < bytes.len() {
            if let Some(t) = lexer.next_token(bytes) {
                tokens.push(t);
            }
        }
        tokens
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

    fn tokenize_with_pos(input: &str) -> Vec<(Token, u32, u32)> {
        let bytes = input.as_bytes();
        let mut lexer = Lexer::new();
        let mut out = Vec::new();
        while (lexer.pos as usize) < bytes.len() {
            if let Some(t) = lexer.next_token(bytes) {
                out.push((t, lexer.prev_token_line, lexer.prev_token_col));
            }
        }
        out
    }

    #[test]
    fn lexer_initial_state() {
        let lexer = Lexer::new();
        assert_eq!(lexer.pos, 0);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 1);
        assert_eq!(lexer.prev_token_line, 1);
        assert_eq!(lexer.prev_token_col, 1);
    }

    #[test]
    fn single_token_records_start_position() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"{");
        assert_eq!(lexer.prev_token_line, 1);
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.line, 1);
        assert_eq!(lexer.col, 2);
    }

    #[test]
    fn leading_whitespace_advances_col_to_token_start() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"   {");
        assert_eq!(lexer.prev_token_line, 1);
        assert_eq!(lexer.prev_token_col, 4);
    }

    #[test]
    fn newline_advances_line_and_resets_col() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"\n{");
        assert_eq!(lexer.prev_token_line, 2);
        assert_eq!(lexer.prev_token_col, 1);
    }

    #[test]
    fn multiple_newlines_each_advance_line() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"\n\n\n{");
        assert_eq!(lexer.prev_token_line, 4);
        assert_eq!(lexer.prev_token_col, 1);
    }

    #[test]
    fn tab_advances_col_by_one() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"\t{");
        assert_eq!(lexer.prev_token_line, 1);
        assert_eq!(lexer.prev_token_col, 2);
    }

    #[test]
    fn string_advances_col_per_byte() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(br#""abc""#);
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 6);
        assert_eq!(lexer.line, 1);
    }

    #[test]
    fn string_with_spaces_advances_col_per_byte() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(br#""hello world""#);
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 14);
    }

    #[test]
    fn number_advances_col_per_digit() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"12345");
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 6);
        assert_eq!(lexer.line, 1);
    }

    #[test]
    fn keyword_true_advances_col_by_four() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"true");
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 5);
    }

    #[test]
    fn keyword_false_advances_col_by_five() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"false");
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 6);
    }

    #[test]
    fn keyword_null_advances_col_by_four() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"null");
        assert_eq!(lexer.prev_token_col, 1);
        assert_eq!(lexer.col, 5);
    }

    #[test]
    fn tracks_position_of_each_token_inline() {
        let positions = tokenize_with_pos(r#"{"k":1}"#);
        let lines: Vec<u32> = positions.iter().map(|(_, l, _)| *l).collect();
        let cols: Vec<u32> = positions.iter().map(|(_, _, c)| *c).collect();
        assert_eq!(lines, vec![1, 1, 1, 1, 1]);
        assert_eq!(cols, vec![1, 2, 5, 6, 7]);
    }

    #[test]
    fn tracks_position_across_lines() {
        let input = "{\n  \"k\": 1\n}";
        let positions = tokenize_with_pos(input);
        let line_col: Vec<(u32, u32)> = positions.iter().map(|(_, l, c)| (*l, *c)).collect();
        assert_eq!(line_col, vec![(1, 1), (2, 3), (2, 6), (2, 8), (3, 1)]);
    }

    #[test]
    fn second_token_after_newline_token() {
        let input = b"1\n2";
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(input);
        assert_eq!(lexer.prev_token_line, 1);
        assert_eq!(lexer.prev_token_col, 1);
        let _ = lexer.next_token(input);
        assert_eq!(lexer.prev_token_line, 2);
        assert_eq!(lexer.prev_token_col, 1);
    }

    #[test]
    fn prev_token_position_unchanged_at_eof() {
        let mut lexer = Lexer::new();
        let bytes = b"  {";
        let _ = lexer.next_token(bytes);
        let saved_line = lexer.prev_token_line;
        let saved_col = lexer.prev_token_col;
        assert!(lexer.next_token(bytes).is_none());
        assert_eq!(lexer.prev_token_line, saved_line);
        assert_eq!(lexer.prev_token_col, saved_col);
    }

    #[test]
    fn line_does_not_advance_within_token() {
        let mut lexer = Lexer::new();
        let _ = lexer.next_token(b"12345");
        assert_eq!(lexer.line, 1);
    }

    #[test]
    fn tracks_position_through_nested_structure() {
        let input = "[\n  {\n    \"k\": true\n  }\n]";
        let positions = tokenize_with_pos(input);
        let line_col: Vec<(u32, u32)> = positions.iter().map(|(_, l, c)| (*l, *c)).collect();
        assert_eq!(
            line_col,
            vec![
                (1, 1), // [
                (2, 3), // {
                (3, 5), // "k"
                (3, 8), // :
                (3, 10), // true
                (4, 3), // }
                (5, 1), // ]
            ]
        );
    }
}
