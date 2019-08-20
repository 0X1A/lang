extern crate nom;

use std::collections::HashMap;

use crate::error::*;
use crate::syntax::span::*;
use crate::syntax::token::*;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::multi::many1;
use nom::{
    bytes::complete::take_while1, character::complete::multispace0, character::is_alphanumeric,
    character::is_digit, sequence::preceded, IResult,
};

macro_rules! gen_lex_token {
    ($token_name:ident, $t:tt, $token_type:expr) => {
        fn $token_name<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
            let (input, begin) = preceded(multispace0, position)(input)?;
            let (input, output) = preceded(multispace0, tag($t))(input)?;
            let (input, end) = preceded(multispace0, position)(input)?;
            let t = TokenTwo::from($token_type, output.input);
            Ok((
                input,
                TokenTwo {
                    token_type: $token_type,
                    span: SourceSpan::new(begin, end),
                    value: t.unwrap(),
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
gen_lex_token!(lex_fn, "fn", TokenType::Enum);
gen_lex_token!(lex_or, "or", TokenType::Or);
gen_lex_token!(lex_impl, "impl", TokenType::Impl);
gen_lex_token!(lex_trait, "trait", TokenType::Trait);
gen_lex_token!(lex_true, "true", TokenType::True);
gen_lex_token!(lex_false, "false", TokenType::False);
gen_lex_token!(lex_self, "self", TokenType::SelfIdent);

// Symbol lexrs
gen_lex_token!(lex_left_brace, "{", TokenType::LeftBrace);
gen_lex_token!(lex_right_brace, "}", TokenType::RightBrace);
gen_lex_token!(lex_left_bracket, "[", TokenType::LeftBracket);
gen_lex_token!(lex_right_bracket, "]", TokenType::LeftBracket);
gen_lex_token!(lex_right_paren, ")", TokenType::RightParen);
gen_lex_token!(lex_left_paren, "(", TokenType::LeftParen);
gen_lex_token!(lex_comma, ",", TokenType::Comma);
gen_lex_token!(lex_dot, ".", TokenType::Dot);
gen_lex_token!(lex_minus, "-", TokenType::Minus);
gen_lex_token!(lex_plus, "+", TokenType::Plus);
gen_lex_token!(lex_semi_colon, ";", TokenType::SemiColon);
gen_lex_token!(lex_colon, ":", TokenType::Colon);
gen_lex_token!(lex_star, "*", TokenType::Star);
gen_lex_token!(lex_bang, "!", TokenType::Bang);
gen_lex_token!(lex_equal, "=", TokenType::Equal);
gen_lex_token!(lex_less_than, "<", TokenType::Less);
gen_lex_token!(lex_greater_than, ">", TokenType::Greater);
gen_lex_token!(lex_slash, "/", TokenType::Slash);
gen_lex_token!(lex_double_quote, "\"", TokenType::DoubleQuote);

fn entry<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, result) = alt((lex_type, lex_keyword, lex_digit, lex_ident, lex_symbol))(input)?;
    Ok((input, result))
}

fn lex_program<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Vec<TokenTwo>, LangError> {
    let (input, output) = dbg_dmp(many1(entry), "lex_program")(input)?;
    Ok((input, output))
}

fn lex_keyword<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, token) = alt((
        lex_let, lex_struct, lex_if, lex_else, lex_break, lex_enum, lex_fn, lex_for, lex_while,
        lex_or, lex_impl, lex_trait, lex_true, lex_false, lex_self,
    ))(input)?;
    Ok((input, token))
}

fn lex_symbol<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, token) = alt((
        lex_left_brace,
        lex_right_brace,
        lex_right_paren,
        lex_left_paren,
        lex_left_bracket,
        lex_right_bracket,
        lex_comma,
        lex_dot,
        lex_minus,
        lex_plus,
        lex_colon,
        lex_semi_colon,
        lex_star,
        lex_bang,
        lex_equal,
        lex_less_than,
        lex_greater_than,
        lex_slash,
        lex_double_quote,
    ))(input)?;
    Ok((input, token))
}

fn allowable_ident_char(input: char) -> bool {
    return is_alphanumeric(input as u8) || input == '_';
}

// TODO: Revisit, allow non-ascii identifiers. This isn't something I want to bite off right now
fn lex_ident<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, idientifier) = preceded(multispace0, take_while1(allowable_ident_char))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    let t = TokenTwo::from2(TokenType::Identifier, idientifier.input)?;
    Ok((
        input,
        TokenTwo {
            token_type: TokenType::Identifier,
            span: SourceSpan::new(begin, end),
            value: t,
        },
    ))
}

fn lex_type<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, type_str) = preceded(
        multispace0,
        alt((
            tag("i32"),
            tag("i64"),
            tag("f32"),
            tag("f64"),
            tag("bool"),
            tag("()"),
            tag("fn"),
            tag("String"),
            tag("Array"),
            take_while1(allowable_ident_char),
        )),
    )(input)?;
    let (input, end) = position(input)?;
    let type_annotation = match type_str.input {
        "i32" => TypeAnnotation::I32,
        "i64" => TypeAnnotation::I64,
        "f32" => TypeAnnotation::F32,
        "f64" => TypeAnnotation::F64,
        "bool" => TypeAnnotation::Bool,
        "String" => TypeAnnotation::String,
        "()" => TypeAnnotation::Unit,
        "fn" => TypeAnnotation::Fn,
        other @ _ => TypeAnnotation::User(other.to_string()),
    };
    let t = TokenTwo::from(TokenType::Identifier, type_str.input);
    Ok((
        input,
        TokenTwo {
            token_type: TokenType::Type(type_annotation),
            span: SourceSpan::new(begin, end),
            value: t.unwrap(),
        },
    ))
}

fn lis_digit(i: char) -> bool {
    is_digit(i as u8)
}

fn lex_digit<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, digit) = preceded(multispace0, take_while1(lis_digit))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    let t = TokenTwo::from(TokenType::Integer, digit.input);
    Ok((
        input,
        TokenTwo {
            token_type: TokenType::Integer,
            span: SourceSpan::new(begin, end),
            value: t.unwrap(),
        },
    ))
}

trait SubStr {
    fn substr(&self, beg: usize, end: usize) -> String;
}

impl SubStr for String {
    /// Creates a substring from a string using indices `from` and `to`
    fn substr(&self, from: usize, to: usize) -> String {
        self.chars().skip(from).take(to - from).collect()
    }
}

impl SubStr for str {
    /// Creates a substring from a string using indices `from` and `to`
    fn substr(&self, from: usize, to: usize) -> String {
        self.chars().skip(from).take(to - from).collect()
    }
}

pub struct ScannerTwo<'a> {
    source: &'a str,
    pub tokens: Vec<TokenTwo<'a>>,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> ScannerTwo<'a> {
    pub fn new(script_content: &'a str) -> ScannerTwo<'a> {
        let mut keywords = HashMap::new();
        keywords.insert("break", TokenType::Break);
        keywords.insert("enum", TokenType::Enum);
        keywords.insert("and", TokenType::And);
        keywords.insert("struct", TokenType::Struct);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fn", TokenType::Fn);
        keywords.insert("if", TokenType::If);
        keywords.insert("unit", TokenType::Unit);
        keywords.insert("or", TokenType::Or);
        keywords.insert("impl", TokenType::Impl);
        keywords.insert("trait", TokenType::Trait);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("true", TokenType::True);
        keywords.insert("let", TokenType::Let);
        keywords.insert("while", TokenType::While);
        keywords.insert("self", TokenType::SelfIdent);
        ScannerTwo {
            source: script_content,
            tokens: Vec::new(),
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<TokenTwo>, LangError> {
        let root_span: Span<&str> = Span::new(self.source, 0, 1, 0);
        println!("root span: {:?}", root_span);
        println!("root span: {:?}", root_span.input);
        match lex_program(root_span) {
            Ok(t) => {
                println!("{:?}", t);
                Ok(t.1)
            }
            Err(e) => {
                match e {
                    nom::Err::Error(err) => {
                        println!("Error: {:?}", err);
                    }
                    nom::Err::Failure(err) => {
                        println!("Failure: {:?}", err);
                    }
                    _ => println!("Incomplete: {:?}", e),
                }
                Ok(vec![])
            }
        }
    }
}

#[cfg(test)]
mod scanner_two_tests {
    use super::*;

    #[test]
    fn test_lex_ident() {
        let test_string = "   \nvalid_ident\n;\ntest\n;;";
        let string = Span::new(test_string, 0, 1, 0);
        let string2 = Span::new(test_string, 0, 1, 0);
        let result = lex_ident(string);
        let result2 = lex_ident(string2);
        //assert_eq!(result.is_ok(), true);
        println!("{:?}", result);
        println!("{:?}", result2);
    }
}
