extern crate lazy_static;
extern crate nom;

use crate::error::*;
use crate::syntax::span::*;
use crate::syntax::token::*;
use crate::token::{TokenType, TypeAnnotation};
use crate::value::{Value, ValueType};

use nom::branch::*;
use nom::bytes::complete::*;
use nom::multi::many1;
use nom::{
    bytes::complete::take_while1, character::complete::multispace0, character::is_alphanumeric,
    character::is_digit, sequence::preceded, IResult,
};
use std::collections::HashMap;

// A very convoluted iterator, this is essentially like a mutable iter::windows fixed to size 3
struct TokenIter<'a, T> {
    slice: &'a mut [T],
    len: usize,
    position: usize,
}

impl<'a, T> TokenIter<'a, T> {
    fn new(data: &'a mut [T]) -> TokenIter<T> {
        TokenIter {
            slice: data,
            len: 3,
            position: 0,
        }
    }

    fn next(&mut self) -> Option<&mut [T]> {
        let mut upper_bound = self.slice.len() + (self.len / 2);
        if upper_bound >= self.slice.len() {
            upper_bound = self.slice.len() - 1;
        }
        if self.position + (self.len / 2) < upper_bound {
            self.position += 1;
            Some(&mut self.slice[(self.position - 1)..(self.position - 1) + self.len])
        } else if self.position + (self.len / 2) <= upper_bound {
            self.position += 1;
            Some(&mut self.slice[(self.position - 1)..(self.position - 1) + (self.len - 1)])
        } else {
            None
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("let", TokenType::Let);
        keywords.insert("struct", TokenType::Struct);
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);
        keywords.insert("break", TokenType::Break);
        keywords.insert("enum", TokenType::Enum);
        keywords.insert("for", TokenType::For);
        keywords.insert("while", TokenType::While);
        keywords.insert("fn", TokenType::Fn);
        keywords.insert("or", TokenType::Or);
        keywords.insert("and", TokenType::And);
        keywords.insert("impl", TokenType::Impl);
        keywords.insert("trait", TokenType::Trait);
        keywords.insert("true", TokenType::True);
        keywords.insert("false", TokenType::False);
        keywords.insert("self", TokenType::SelfIdent);
        keywords.insert("return", TokenType::Return);
        keywords.insert("print", TokenType::Print);
        keywords.insert("import", TokenType::Import);
        keywords
    };
}

macro_rules! gen_lex_token {
    ($token_name:ident, $t:tt, $token_type:expr) => {
        fn $token_name<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
            let (input, begin) = preceded(multispace0, position)(input)?;
            let (input, output) = preceded(multispace0, tag($t))(input)?;
            let (input, end) = preceded(multispace0, position)(input)?;
            let value = match Value::from_str(ValueType::String, output.input) {
                Ok(v) => v,
                Err(e) => return Err(nom::Err::Failure::<LangError>(e.into())),
            };
            Ok((
                input,
                Token {
                    token_type: $token_type,
                    span: SourceSpan::new(begin, output, end),
                    value: value,
                },
            ))
        }
    };
}

// Keyword lexrs
gen_lex_token!(lex_let, "let", TokenType::Let);
gen_lex_token!(lex_struct, "struct", TokenType::Struct);
gen_lex_token!(lex_if, "if", TokenType::If);
gen_lex_token!(lex_else, "else", TokenType::Else);
gen_lex_token!(lex_break, "break", TokenType::Break);
gen_lex_token!(lex_enum, "enum", TokenType::Enum);
gen_lex_token!(lex_for, "for", TokenType::For);
gen_lex_token!(lex_while, "while", TokenType::While);
gen_lex_token!(lex_fn, "fn", TokenType::Fn);
gen_lex_token!(lex_or, "or", TokenType::Or);
gen_lex_token!(lex_and, "and", TokenType::And);
gen_lex_token!(lex_impl, "impl", TokenType::Impl);
gen_lex_token!(lex_trait, "trait", TokenType::Trait);
gen_lex_token!(lex_true, "true", TokenType::True);
gen_lex_token!(lex_false, "false", TokenType::False);
gen_lex_token!(lex_self, "self", TokenType::SelfIdent);
gen_lex_token!(lex_return, "return", TokenType::Return);
gen_lex_token!(lex_print, "print", TokenType::Print);
gen_lex_token!(lex_import, "import", TokenType::Import);

// Symbol lexrs
gen_lex_token!(lex_left_brace, "{", TokenType::LeftBrace);
gen_lex_token!(lex_right_brace, "}", TokenType::RightBrace);
gen_lex_token!(lex_left_bracket, "[", TokenType::LeftBracket);
gen_lex_token!(lex_right_bracket, "]", TokenType::RightBracket);
gen_lex_token!(lex_right_paren, ")", TokenType::RightParen);
gen_lex_token!(lex_left_paren, "(", TokenType::LeftParen);
gen_lex_token!(lex_comma, ",", TokenType::Comma);
gen_lex_token!(lex_dot, ".", TokenType::Dot);
gen_lex_token!(lex_minus, "-", TokenType::Minus);
gen_lex_token!(lex_plus, "+", TokenType::Plus);
gen_lex_token!(lex_semi_colon, ";", TokenType::SemiColon);
gen_lex_token!(lex_colon, ":", TokenType::Colon);
gen_lex_token!(lex_path_separator, "::", TokenType::PathSeparator);
gen_lex_token!(lex_star, "*", TokenType::Star);
gen_lex_token!(lex_equal, "=", TokenType::Equal);
gen_lex_token!(lex_slash, "/", TokenType::Slash);
gen_lex_token!(lex_return_type, "->", TokenType::ReturnType);
gen_lex_token!(lex_double_quote, "\"", TokenType::DoubleQuote);

// Logical
gen_lex_token!(lex_bang, "!", TokenType::Bang);
gen_lex_token!(lex_and_symbol, "&", TokenType::And);
gen_lex_token!(lex_or_symbol, "|", TokenType::Or);
gen_lex_token!(lex_ternary, "?", TokenType::Ternary);

// Comparisons
gen_lex_token!(lex_bang_equal, "!=", TokenType::BangEqual);
gen_lex_token!(lex_less_than, "<", TokenType::Less);
gen_lex_token!(lex_less_eq, "<=", TokenType::LessEqual);
gen_lex_token!(lex_greater_than, ">", TokenType::Greater);
gen_lex_token!(lex_greater_eq, ">=", TokenType::GreaterEqual);
gen_lex_token!(lex_equal_equal, "==", TokenType::EqualEqual);

// Types
gen_lex_token!(lex_i32_type, "i32", TokenType::Type(TypeAnnotation::I32));
gen_lex_token!(lex_i64_type, "i64", TokenType::Type(TypeAnnotation::I64));
gen_lex_token!(lex_f32_type, "f32", TokenType::Type(TypeAnnotation::F32));
gen_lex_token!(lex_f64_type, "f64", TokenType::Type(TypeAnnotation::F64));
gen_lex_token!(lex_bool_type, "bool", TokenType::Type(TypeAnnotation::Bool));
//gen_lex_token!(lex_fn_type, "fn", TokenType::Type(TypeAnnotation::Fn));
gen_lex_token!(
    lex_string_type,
    "String",
    TokenType::Type(TypeAnnotation::String)
);

fn entry<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, result) = alt((
        lex_digit,
        lex_type,
        lex_ident,
        lex_keyword,
        lex_string,
        lex_symbol,
    ))(input)?;
    Ok((input, result))
}

fn lex_program<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Vec<Token>, LangError> {
    let (input, mut output) = many1(entry)(input)?;
    output.push(Token::new2(TokenType::Eof, "EoF"));
    Ok((input, output))
}

fn lex_keyword<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, token) = alt((
        lex_let, lex_struct, lex_if, lex_else, lex_break, lex_enum, lex_fn, lex_for, lex_while,
        lex_or, lex_impl, lex_trait, lex_true, lex_false, lex_self, lex_print, lex_return, lex_and,
        lex_import,
    ))(input)?;
    Ok((input, token))
}

fn lex_symbol<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, token) = alt((
        lex_left_brace,
        lex_right_brace,
        lex_right_paren,
        lex_left_paren,
        lex_left_bracket,
        lex_right_bracket,
        lex_comparison,
        lex_comma,
        lex_dot,
        lex_return_type,
        lex_minus,
        lex_plus,
        lex_path_separator,
        lex_colon,
        lex_semi_colon,
        lex_star,
        lex_slash,
        lex_or_symbol,
        lex_and_symbol,
        lex_ternary,
    ))(input)?;
    Ok((input, token))
}

fn lex_comparison<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, token) = alt((
        lex_bang_equal,
        lex_bang,
        lex_equal_equal,
        lex_equal,
        lex_less_eq,
        lex_less_than,
        lex_greater_eq,
        lex_greater_than,
    ))(input)?;
    Ok((input, token))
}

fn allowable_ident_char(input: char) -> bool {
    is_alphanumeric(input as u8) || input == '_'
}

// TODO: Revisit, allow non-ascii identifiers. This isn't something I want to bite off right now
fn lex_ident<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, identifier) = preceded(multispace0, take_while1(allowable_ident_char))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    if KEYWORDS.get(identifier.input).is_some() {
        return Err(nom::Err::<LangError>::Error(LangError::from(
            LangErrorType::ParserError {
                reason: "Identifiers may not be a keyword".into(),
            },
        )));
    }
    if is_digit(identifier.input.chars().nth(0).unwrap() as u8) {
        return Err(nom::Err::<LangError>::Error(LangError::from(
            LangErrorType::ParserError {
                reason: "Identifiers may not begin with digits".into(),
            },
        )));
    }
    let value = match Value::from_str(ValueType::String, identifier.input) {
        Ok(v) => v,
        Err(e) => return Err(nom::Err::Failure::<LangError>(e)),
    };
    Ok((
        input,
        Token {
            token_type: TokenType::Identifier,
            span: SourceSpan::new(begin, identifier, end),
            value,
        },
    ))
}

fn is_allowable_string_content(input: char) -> bool {
    input != '"'
}

fn lex_string_content<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, content) = take_while1(is_allowable_string_content)(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    let value = match Value::from_str(ValueType::String, content.input) {
        Ok(v) => v,
        Err(e) => return Err(nom::Err::Failure::<LangError>(e)),
    };
    Ok((
        input,
        Token {
            token_type: TokenType::String,
            span: SourceSpan::new(begin, content, end),
            value,
        },
    ))
}

fn lex_string<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, _) = lex_double_quote(input)?;
    let (input, string) = lex_string_content(input)?;
    let (input, _) = lex_double_quote(input)?;
    Ok((input, string))
}

fn lex_array<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, arr) = preceded(multispace0, tag("Array<"))(input)?;
    let (input, array_type) = lex_type(input)?;
    let (input, _) = preceded(multispace0, tag(">"))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    let type_annotation = match array_type.token_type.to_type_annotation() {
        Ok(v) => v,
        Err(e) => return Err(nom::Err::Failure::<LangError>(e)),
    };
    let value = match Value::from_str(
        ValueType::String,
        &format!("Array<{}>", type_annotation.to_string()),
    ) {
        Ok(v) => v,
        Err(e) => return Err(nom::Err::Failure::<LangError>(e)),
    };
    Ok((
        input,
        Token {
            token_type: TokenType::Type(TypeAnnotation::Array(Box::new(type_annotation))),
            span: SourceSpan::new(begin, arr, end),
            value,
        },
    ))
}

fn lex_type<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, type_annotation) = alt((
        lex_i32_type,
        lex_i64_type,
        lex_f32_type,
        lex_f64_type,
        lex_bool_type,
        // lex_fn_type,
        lex_array,
        lex_string_type,
    ))(input)?;
    Ok((input, type_annotation))
}

fn lis_digit(i: char) -> bool {
    is_digit(i as u8) || i == '.'
}

fn lex_digit<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Token, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, digit) = preceded(multispace0, take_while1(lis_digit))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    // TODO: Fix this parser function, we shouldn't have to waste cycles on goofy shit like this
    if digit.input.len() == 1 && digit.input.contains('.') {
        return Err(nom::Err::<LangError>::Error(LangError::from(
            LangErrorType::ParserError {
                reason: "Attempted to lex a digit that was only '.'".into(),
            },
        )));
    }
    let is_float = digit.input.contains('.');
    let value_type = if is_float {
        ValueType::Float
    } else {
        ValueType::Integer
    };
    let token_type = if is_float {
        TokenType::Float
    } else {
        TokenType::Integer
    };
    let value = match Value::from_str(value_type.clone(), digit.input) {
        Ok(v) => v,
        Err(e) => return Err(nom::Err::Failure::<LangError>(e)),
    };
    Ok((
        input,
        Token {
            token_type,
            span: SourceSpan::new(begin, digit, end),
            value,
        },
    ))
}

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(script_content: &'a str) -> Scanner<'a> {
        Scanner {
            source: script_content,
            tokens: Vec::new(),
        }
    }

    fn fixup_types(&self, tokens: &mut Vec<Token>) -> Result<(), LangError> {
        // Contain's the index of a left paren that should be converted to Unit, and Right paren removed
        let mut unit_type_indicies = Vec::new();
        let mut window = TokenIter::new(tokens);
        while let Some(slice) = window.next() {
            let is_type_annotation = slice[0].token_type == TokenType::Colon
                || slice[0].token_type == TokenType::ReturnType;
            if is_type_annotation && slice[1].token_type == TokenType::Identifier {
                slice[1].token_type =
                    TokenType::Type(TypeAnnotation::User(slice[1].value.to_string()));
            }
            if is_type_annotation
                && slice[1].token_type == TokenType::LeftParen
                && slice[2].token_type == TokenType::RightParen
            {
                slice[1].token_type = TokenType::Type(TypeAnnotation::Unit);
                unit_type_indicies.push(window.position + 1);
            }
        }
        let mut index = 0;
        tokens.retain(|token| {
            let should_retain = {
                !(token.token_type == TokenType::RightParen
                    && unit_type_indicies.contains(&(index as usize)))
            };
            (should_retain, index += 1).0
        });
        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LangError> {
        let root_span: Span<&str> = Span::new(self.source, 0, 1, 0);
        match lex_program(root_span) {
            Ok(mut t) => {
                self.fixup_types(&mut t.1)?;
                Ok(t.1)
            }
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod scanner_two_tests {
    macro_rules! gen_lex_token_test {
        ($test_name:ident, $fn_name:ident, $test_string:literal, $token_type:expr, $result:literal) => {
            #[test]
            fn $test_name() {
                let span = Span::new($test_string, 0, 1, 0);
                let result = $fn_name(span);
                assert_eq!(result.is_ok(), $result);
                if $result {
                    assert_eq!(result.unwrap().1.token_type, $token_type);
                }
            }
        };
    }

    use super::*;

    #[test]
    fn test_lex_ident() {
        let test_string = "   \nvalid_ident\n;\ntest\n;;";
        let string = Span::new(test_string, 0, 1, 0);
        let result = lex_ident(string);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().1.span.content.input, "valid_ident");

        let test_string = "420\n";
        let string = Span::new(test_string, 0, 1, 0);
        let result = lex_ident(string);
        assert_eq!(result.is_ok(), false);

        let test_string = "?!90$*\n";
        let string = Span::new(test_string, 0, 1, 0);
        let result = lex_ident(string);
        assert_eq!(result.is_ok(), false);
    }

    gen_lex_token_test!(
        test_long_lex,
        entry,
        "letShouldLexAsIdent",
        TokenType::Identifier,
        true
    );
    gen_lex_token_test!(test_lex_let, lex_keyword, "let", TokenType::Let, true);
    gen_lex_token_test!(
        test_lex_struct,
        lex_keyword,
        "struct",
        TokenType::Struct,
        true
    );
    gen_lex_token_test!(test_lex_if, lex_keyword, "if", TokenType::If, true);
    gen_lex_token_test!(test_lex_else, lex_keyword, "else", TokenType::Else, true);
    gen_lex_token_test!(test_lex_break, lex_keyword, "break", TokenType::Break, true);
    gen_lex_token_test!(test_lex_enum, lex_keyword, "enum", TokenType::Enum, true);
    gen_lex_token_test!(test_lex_for, lex_keyword, "for", TokenType::For, true);
    gen_lex_token_test!(test_lex_while, lex_keyword, "while", TokenType::While, true);
    gen_lex_token_test!(test_lex_fn, lex_keyword, "fn", TokenType::Fn, true);
    gen_lex_token_test!(test_lex_or, lex_keyword, "or", TokenType::Or, true);
    gen_lex_token_test!(test_lex_and, lex_keyword, "and", TokenType::And, true);
    gen_lex_token_test!(test_lex_impl, lex_keyword, "impl", TokenType::Impl, true);
    gen_lex_token_test!(test_lex_trait, lex_keyword, "trait", TokenType::Trait, true);
    gen_lex_token_test!(test_lex_true, lex_keyword, "true", TokenType::True, true);
    gen_lex_token_test!(test_lex_false, lex_keyword, "false", TokenType::False, true);
    gen_lex_token_test!(
        test_lex_self,
        lex_keyword,
        "self",
        TokenType::SelfIdent,
        true
    );
    gen_lex_token_test!(
        test_lex_return,
        lex_keyword,
        "return",
        TokenType::Return,
        true
    );
    gen_lex_token_test!(test_lex_print, lex_keyword, "print", TokenType::Print, true);
    gen_lex_token_test!(
        test_lex_import,
        lex_keyword,
        "import",
        TokenType::Import,
        true
    );

    gen_lex_token_test!(
        test_lex_keyword_fail,
        lex_keyword,
        "l",
        TokenType::Let,
        false
    );

    gen_lex_token_test!(
        test_lex_left_brace,
        lex_left_brace,
        "{",
        TokenType::LeftBrace,
        true
    );
    gen_lex_token_test!(
        test_lex_right_brace,
        lex_right_brace,
        "}",
        TokenType::RightBrace,
        true
    );
    gen_lex_token_test!(
        test_lex_right_paren,
        lex_right_paren,
        ")",
        TokenType::RightParen,
        true
    );
    gen_lex_token_test!(
        test_lex_left_paren,
        lex_left_paren,
        "(",
        TokenType::LeftParen,
        true
    );
    gen_lex_token_test!(
        test_lex_left_bracket,
        lex_left_bracket,
        "[",
        TokenType::LeftBracket,
        true
    );
    gen_lex_token_test!(
        test_lex_right_bracket,
        lex_right_bracket,
        "]",
        TokenType::RightBracket,
        true
    );
    // gen_lex_token_test!(test_lex_comparison, lex_comparison, "", TokenType::, true);
    gen_lex_token_test!(test_lex_comma, lex_comma, ",", TokenType::Comma, true);
    gen_lex_token_test!(test_lex_dot, lex_dot, ".", TokenType::Dot, true);
    gen_lex_token_test!(
        test_lex_return_type,
        lex_return_type,
        "->",
        TokenType::ReturnType,
        true
    );
    gen_lex_token_test!(test_lex_minus, lex_minus, "-", TokenType::Minus, true);
    gen_lex_token_test!(test_lex_plus, lex_plus, "+", TokenType::Plus, true);
    gen_lex_token_test!(
        test_lex_path_separator,
        lex_path_separator,
        "::",
        TokenType::PathSeparator,
        true
    );
    gen_lex_token_test!(test_lex_colon, lex_colon, ":", TokenType::Colon, true);
    gen_lex_token_test!(
        test_lex_semi_colon,
        lex_semi_colon,
        ";",
        TokenType::SemiColon,
        true
    );
    gen_lex_token_test!(test_lex_star, lex_star, "*", TokenType::Star, true);
    gen_lex_token_test!(test_lex_slash, lex_slash, "/", TokenType::Slash, true);
    gen_lex_token_test!(test_lex_or_symbol, lex_or_symbol, "|", TokenType::Or, true);
    gen_lex_token_test!(
        test_lex_and_symbol,
        lex_and_symbol,
        "&",
        TokenType::And,
        true
    );
    gen_lex_token_test!(test_lex_ternary, lex_ternary, "?", TokenType::Ternary, true);
    gen_lex_token_test!(
        test_lex_double_quote,
        lex_double_quote,
        "\"",
        TokenType::DoubleQuote,
        true
    );

    gen_lex_token_test!(test_lex_bang, lex_comparison, "!", TokenType::Bang, true);
    gen_lex_token_test!(
        test_lex_bang_equal,
        lex_comparison,
        "!=",
        TokenType::BangEqual,
        true
    );
    gen_lex_token_test!(
        test_lex_equal_equal,
        lex_comparison,
        "==",
        TokenType::EqualEqual,
        true
    );
    gen_lex_token_test!(test_lex_equal, lex_comparison, "=", TokenType::Equal, true);
    gen_lex_token_test!(
        test_lex_less_eq,
        lex_comparison,
        "<=",
        TokenType::LessEqual,
        true
    );
    gen_lex_token_test!(
        test_lex_less_than,
        lex_comparison,
        "<",
        TokenType::Less,
        true
    );
    gen_lex_token_test!(
        test_lex_greater_eq,
        lex_comparison,
        ">=",
        TokenType::GreaterEqual,
        true
    );
    gen_lex_token_test!(
        test_lex_greater_than,
        lex_comparison,
        ">",
        TokenType::Greater,
        true
    );
}
