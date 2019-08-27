extern crate nom;

use crate::error::*;
use crate::syntax::span::*;
use crate::syntax::token::*;
use crate::token::{TokenType, TypeAnnotation};

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
            let t = TokenTwo::get_value(ValueType::String, output.input)?;
            Ok((
                input,
                TokenTwo {
                    token_type: $token_type,
                    span: SourceSpan::new(begin, end),
                    value: t,
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
gen_lex_token!(lex_print, "print", TokenType::Print);

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
    let (input, result) = alt((lex_digit, lex_keyword, lex_type, lex_ident, lex_symbol))(input)?;
    Ok((input, result))
}

fn lex_program<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, Vec<TokenTwo>, LangError> {
    let (input, output) = many1(entry)(input)?;
    Ok((input, output))
}

fn lex_keyword<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, token) = alt((
        lex_let, lex_struct, lex_if, lex_else, lex_break, lex_enum, lex_fn, lex_for, lex_while,
        lex_or, lex_impl, lex_trait, lex_true, lex_false, lex_self, lex_print,
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
    let t = TokenTwo::get_value(ValueType::String, idientifier.input)?;
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
        "i32" => TokenType::Type(TypeAnnotation::I32),
        "i64" => TokenType::Type(TypeAnnotation::I64),
        "f32" => TokenType::Type(TypeAnnotation::F32),
        "f64" => TokenType::Type(TypeAnnotation::F64),
        "bool" => TokenType::Type(TypeAnnotation::Bool),
        "String" => TokenType::Type(TypeAnnotation::String),
        "()" => TokenType::Type(TypeAnnotation::Unit),
        "fn" => TokenType::Type(TypeAnnotation::Fn),
        _ => TokenType::Identifier,
    };
    let t = TokenTwo::get_value(ValueType::String, type_str.input)?;
    Ok((
        input,
        TokenTwo {
            token_type: type_annotation,
            span: SourceSpan::new(begin, end),
            value: t,
        },
    ))
}

fn lis_digit(i: char) -> bool {
    is_digit(i as u8) || i == '.'
}

fn lex_digit<'a>(input: Span<&'a str>) -> IResult<Span<&'a str>, TokenTwo, LangError> {
    let (input, begin) = preceded(multispace0, position)(input)?;
    let (input, digit) = preceded(multispace0, take_while1(lis_digit))(input)?;
    let (input, end) = preceded(multispace0, position)(input)?;
    // TODO: Fix this parser function, we shouldn't have to waste cycles on goofy shit like this
    if digit.input.len() == 1 && digit.input.contains('.') {
        return Err(nom::Err::<LangError>::Error(LangError::from(
            LangErrorType::ParserError { reason: "Attempted to lex a digit that was only '.'".into() },
        )));
    }
    let value_type;
    let token_type;
    if digit.input.contains('.') {
        value_type = ValueType::Float;
        token_type = TokenType::Float;
    } else {
        value_type = ValueType::Integer;
        token_type = TokenType::Integer;
    }
    let t = TokenTwo::get_value(value_type.clone(), digit.input)?;
    Ok((
        input,
        TokenTwo {
            token_type: token_type,
            span: SourceSpan::new(begin, end),
            value: t,
        },
    ))
}

pub struct ScannerTwo<'a> {
    source: &'a str,
    pub tokens: Vec<TokenTwo<'a>>,
}

impl<'a> ScannerTwo<'a> {
    pub fn new(script_content: &'a str) -> ScannerTwo<'a> {
        ScannerTwo {
            source: script_content,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<TokenTwo>, LangError> {
        let root_span: Span<&str> = Span::new(self.source, 0, 1, 0);
        match lex_program(root_span) {
            Ok(t) => Ok(t.1),
            Err(e) => return Err(e.into()),
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
