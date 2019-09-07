use crate::syntax::span::Span;
use crate::token::TokenType;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceSpan<'a> {
    pub begin: Span<&'a str>,
    pub content: Span<&'a str>,
    pub end: Span<&'a str>,
}

impl<'a> SourceSpan<'a> {
    pub fn new(begin: Span<&'a str>, content: Span<&'a str>, end: Span<&'a str>) -> SourceSpan<'a> {
        SourceSpan {
            begin,
            content,
            end,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub span: SourceSpan<'a>,
    pub value: Value,
}

impl<'a> Token<'a> {
    pub fn new2(token_type: TokenType, lexeme: &str) -> Token {
        let value = match token_type {
            _ => Value::String(token_type.to_string()),
        };
        Token {
            token_type,
            span: SourceSpan::new(
                Span::new(lexeme, 0, 0, 0),
                Span::new(lexeme, 0, 0, 0),
                Span::new(lexeme, 0, 0, 0),
            ),
            value,
        }
    }
}
