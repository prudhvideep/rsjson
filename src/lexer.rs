use std::iter::Peekable;

#[derive(Debug)]
pub(crate) struct Lexer {
    tokens: Vec<Token>,
}

#[derive(Debug,Clone)]
pub(crate) struct Span {
    line : usize,
    col : usize,
}

#[derive(Debug, Clone)]
pub(crate) enum Token {
    LeftBrace(Span),
    RightBrace(Span),
    LeftBracket(Span),
    RightBracket(Span),
    String(String,Span),
    Number(String,Span),
    Colon(Span),
    Comma(Span),
    True(Span),
    False(Span),
    Null(Span),
}

impl Token {
    pub(crate) fn span(&self) -> (usize,usize) {
        match self {
            Token::LeftBrace(s) | Token::RightBrace(s) | Token::LeftBracket(s)
            | Token::RightBracket(s) | Token::Colon(s) | Token::Comma(s)
            | Token::True(s) | Token::False(s) | Token::Null(s) => (s.line, s.col),
            Token::String(_, s) | Token::Number(_, s) => (s.line, s.col),
        }
    } 
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
        let mut line = 1;
        let mut col = 1;

        while let Some(&c) = chars.peek() {
            match c {
                '\n' => {
                    chars.next();
                    line+=1;
                    col=1;
                }
                ' ' | '\t' => {
                    chars.next();
                    col+=1;
                }
                '{' => {
                    tokens.push(Token::LeftBrace(Span { line,col }));
                    chars.next();
                    col+=1;
                }
                '}' => {
                    tokens.push(Token::RightBrace(Span {line,col}));
                    chars.next();
                    col+=1;
                }
                '[' => {
                    tokens.push(Token::LeftBracket(Span {line,col}));
                    chars.next();
                    col+=1;
                }
                ']' => {
                    tokens.push(Token::RightBracket(Span {line,col}));
                    chars.next();
                    col+=1;
                }
                ':' => {
                    tokens.push(Token::Colon(Span { line, col }));
                    chars.next();
                    col +=1;
                }
                ',' => {
                    tokens.push(Token::Comma(Span { line, col }));
                    chars.next();
                    col +=1;
                }
                'n' => {
                    if Self::match_keyword(&mut chars, "null") {
                        let start_pos = col;
                        col+=4;
                        tokens.push(Token::Null(Span { line, col: start_pos }));
                    } 
                }
                't' => {
                    if Self::match_keyword(&mut chars, "true") {
                        let start_pos = col;
                        col+=4;
                        tokens.push(Token::True(Span { line, col: start_pos }));
                    }
                }
                'f' => {
                    if Self::match_keyword(&mut chars, "false") {
                        let start_pos = col;
                        col+=5;
                        tokens.push(Token::False(Span { line, col: start_pos }));
                    }
                }
                '"' => {
                    //Consume the quote
                    let mut s = String::new();
                    let start_pos = col;
                    chars.next();col+=1;
                    while let Some(&c) = chars.peek() {
                        if c == '"' {
                            //Consume the quote
                            chars.next();
                            col+=1;
                            break;
                        }

                        s.push(c);
                        chars.next();
                        col+=1;
                    }

                    tokens.push(Token::String(s,Span { line, col:start_pos }));
                }
                '0'..='9' | '-' => {
                    let mut s = String::new();
                    let start_pos = col;
                    while let Some(&c) = chars.peek() {
                        match c {
                            '.' | '+' | '-' | 'E' | 'e' | '0'..='9' => {
                                s.push(c);
                                chars.next();
                                col+=1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    tokens.push(Token::Number(s,Span { line, col :start_pos}));
                }
                _ => {
                    chars.next();
                    col+=1
                }
            }
        }

        Lexer { tokens }
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }

    fn match_keyword(chars: &mut Peekable<std::str::Chars>, keyword: &str) -> bool {
        for (i, expected_char) in keyword.chars().enumerate() {
            if i == 0 {
                chars.next();
                continue;
            }

            match chars.peek() {
                Some(&c) if c == expected_char => {
                    chars.next();
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

    #[test]
    fn empty_objects() {
        let empty_object = r##"{}"##;

        let tokens = tokenize(empty_object);
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn empty_array() {
        let empty_array = r##"[]"##;

        let tokens = tokenize(empty_array);
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn single_comma() {
        let comma_str = r##","##;

        let tokens = tokenize(comma_str);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Comma(_)));
    }

    #[test]
    fn single_colon() {
        let colon_str = r##":"##;

        let tokens = tokenize(colon_str);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Colon(_)));
    }

    #[test]
    fn all_bracket_types() {
        let tokens = tokenize("{}[]");
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::LeftBrace(_)));
        assert!(matches!(tokens[1], Token::RightBrace(_)));
        assert!(matches!(tokens[2], Token::LeftBracket(_)));
        assert!(matches!(tokens[3], Token::RightBracket(_)));
    }

    #[test]
    fn keyword_true() {
        let tokens = tokenize("true");
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::True(_)));
    }

    #[test]
    fn all_keywords_together() {
        let tokens = tokenize("true false null");
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::True(_)));
        assert!(matches!(tokens[1], Token::False(_)));
        assert!(matches!(tokens[2], Token::Null(_)));
    }

    #[test]
    fn simple_string() {
        let tokens = tokenize(r#""hello""#);
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::String(s, _) => assert_eq!(s,"hello"),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn empty_string() {
        let tokens = tokenize(r#""""#);
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::String(s, _) => assert_eq!(s,""),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn string_with_spaces() {
        let tokens = tokenize(r#""hello world""#);
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::String(s, _) => assert_eq!(s,"hello world"),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn string_with_numbers() {
        let tokens = tokenize(r#""test123""#);
        match &tokens[0] {
            Token::String(s, _) => assert_eq!(s,"test123"),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn multiple_strings() {
        let tokens = tokenize(r#""first" "second" "third""#);
        assert_eq!(tokens.len(), 3);
        match &tokens[0] {
            Token::String(s, _) => assert_eq!(s,"first"),
            _ => panic!("Expected String token"),
        }
        match &tokens[1] {
            Token::String(s, _) => assert_eq!(s,"second"),
            _ => panic!("Expected String token"),
        }
    }

    #[test]
    fn positive_integer() {
        let tokens = tokenize("42");
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"42"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn negative_integer() {
        let tokens = tokenize("-17");
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"-17"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn zero() {
        let tokens = tokenize("0");
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"0"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn decimal_number() {
        let tokens = tokenize("3.14");
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"3.14"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn negative_decimal() {
        let tokens = tokenize("-0.5");
        
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"-0.5"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn scientific_notation() {
        let tokens = tokenize("1e10");
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"1e10"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn scientific_negative_exponent() {
        let tokens = tokenize("2.5e-3");
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"2.5e-3"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn scientific_uppercase_e() {
        let tokens = tokenize("1E5");
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"1E5"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn simple_key_value_pair() {
        let tokens = tokenize(r#"{"key": "value"}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::LeftBrace(_)));
        assert!(matches!(tokens[2], Token::Colon(_)));
        assert!(matches!(tokens[4], Token::RightBrace(_)));
    }

    #[test]
    fn object_with_number_value() {
        let tokens = tokenize(r#"{"age": 30}"#);
        assert_eq!(tokens.len(), 5);
        match &tokens[1] {
            Token::String(s, _) => assert_eq!(s,"age"),
            _ => panic!("Expected String token"),
        }
        match &tokens[3] {
            Token::Number(n, _) => assert_eq!(n,"30"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn object_with_boolean() {
        let tokens = tokenize(r#"{"active": true}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[3], Token::True(_)));
    }

    #[test]
    fn object_with_null() {
        let tokens = tokenize(r#"{"data": null}"#);
        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[3], Token::Null(_)));
    }

    #[test]
    fn object_with_multiple_pairs() {
        let tokens = tokenize(r#"{"a": 1, "b": 2, "c": 3}"#);
        assert_eq!(tokens.len(), 13);

        // Count commas
        let comma_count = tokens.iter().filter(|t| matches!(t, Token::Comma(_))).count();
        assert_eq!(comma_count, 2);
    }

    #[test]
    fn array_of_numbers() {
        let tokens = tokenize("[1, 2, 3]");
        assert_eq!(tokens.len(), 7);

        assert!(matches!(tokens[0], Token::LeftBracket(_)));
        assert!(matches!(tokens[6], Token::RightBracket(_)));
    }

    #[test]
    fn array_of_strings() {
        let tokens = tokenize(r#"["a", "b", "c"]"#);
        assert_eq!(tokens.len(), 7);

        let string_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::String(_, _)))
            .count();
        assert_eq!(string_count, 3);
    }

    #[test]
    fn array_of_mixed_types() {
        let tokens = tokenize(r#"[1, "two", true, null]"#);
        assert_eq!(tokens.len(), 9);

        assert!(matches!(tokens[1], Token::Number(_, _)));
        assert!(matches!(tokens[3], Token::String(_, _)));
        assert!(matches!(tokens[5], Token::True(_)));
        assert!(matches!(tokens[7], Token::Null(_)));
    }

    #[test]
    fn nested_objects() {
        let tokens = tokenize(r#"{"outer": {"inner": 123}}"#);

        let left_braces = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBrace(_)))
            .count();
        let right_braces = tokens
            .iter()
            .filter(|t| matches!(t, Token::RightBrace(_)))
            .count();

        assert_eq!(left_braces, 2);
        assert_eq!(right_braces, 2);
    }

    #[test]
    fn nested_arrays() {
        let tokens = tokenize("[[1, 2], [3, 4]]");

        let left_brackets = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBracket(_)))
            .count();
        let right_brackets = tokens
            .iter()
            .filter(|t| matches!(t, Token::RightBracket(_)))
            .count();

        assert_eq!(left_brackets, 3);
        assert_eq!(right_brackets, 3);
    }

    #[test]
    fn object_with_array_value() {
        let tokens = tokenize(r#"{"numbers": [1, 2, 3]}"#);

        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBrace(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket(_))));
    }

    #[test]
    fn array_of_objects() {
        let tokens = tokenize(r#"[{"a": 1}, {"b": 2}]"#);

        let brace_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBrace(_)))
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

        assert!(matches!(tokens[0], Token::LeftBrace(_)));
        assert!(tokens.iter().any(|t| matches!(t, Token::Comma(_))));
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
            .filter(|t| matches!(t, Token::LeftBrace(_)))
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

        // Should have 5 colons (one per key-value pair)
        let colon_count = tokens.iter().filter(|t| matches!(t, Token::Colon(_))).count();
        assert_eq!(colon_count, 5);

        // Should have 4 commas (between pairs)
        let comma_count = tokens.iter().filter(|t| matches!(t, Token::Comma(_))).count();
        assert_eq!(comma_count, 4);
    }

    #[test]
    fn array_with_many_elements() {
        let tokens = tokenize("[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");

        let number_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Number(_, _)))
            .count();
        assert_eq!(number_count, 10);
    }

    #[test]
    fn balanced_braces_in_complex_json() {
        let input = r#"{"a": [{"b": 1}, {"c": 2}], "d": {"e": 3}}"#;
        let tokens = tokenize(input);

        let left_braces = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBrace(_)))
            .count();
        let right_braces = tokens
            .iter()
            .filter(|t| matches!(t, Token::RightBrace(_)))
            .count();
        let left_brackets = tokens
            .iter()
            .filter(|t| matches!(t, Token::LeftBracket(_)))
            .count();
        let right_brackets = tokens
            .iter()
            .filter(|t| matches!(t, Token::RightBracket(_)))
            .count();

        assert_eq!(left_braces, right_braces);
        assert_eq!(left_brackets, right_brackets);
    }

    #[test]
    fn token_order_in_simple_object() {
        let tokens = tokenize(r#"{"key": "value"}"#);

        assert!(matches!(tokens[0], Token::LeftBrace(_)));
        assert!(matches!(tokens[1], Token::String(_, _)));
        assert!(matches!(tokens[2], Token::Colon(_)));
        assert!(matches!(tokens[3], Token::String(_, _)));
        assert!(matches!(tokens[4], Token::RightBrace(_)));
    }

    #[test]
    fn large_number() {
        let tokens = tokenize("123456789");
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"123456789"),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn decimal_with_many_digits() {
        let tokens = tokenize("3.141592653589793");
        match &tokens[0] {
            Token::Number(n, _) => assert_eq!(n,"3.141592653589793"),
            _ => panic!("Expected Number token"),
        }
    }
}
