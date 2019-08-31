use crate::error::*;
use crate::syntax::span::Span;
use crate::token::TokenType;
use crate::value::{Float32, Float64, Value};

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

#[derive(Debug, Clone)]
pub enum ValueType {
    String,
    Integer,
    Float,
    Boolean,
}

impl<'a> Token<'a> {
    pub fn get_value(value_type: ValueType, lexeme: &str) -> Result<Value, nom::Err<LangError>> {
        let value = match value_type {
            ValueType::String => Value::String(lexeme.to_string()),
            ValueType::Integer => {
                let integer_value = match lexeme.to_string().parse::<i64>() {
                    Ok(i) => i,
                    Err(e) => return Err(nom::Err::Failure::<LangError>(e.into())),
                };
                if integer_value as i32 <= std::i32::MAX && integer_value as i32 >= std::i32::MIN {
                    Value::Int32(integer_value as i32)
                } else {
                    Value::Int64(integer_value)
                }
            }
            ValueType::Float => {
                let value = match lexeme.to_string().parse::<f64>() {
                    Ok(i) => i,
                    Err(e) => return Err(nom::Err::Failure::<LangError>(e.into())),
                };
                if value as f32 <= std::f32::MAX && value as f32 >= std::f32::MIN {
                    Value::Float32(Float32 {
                        inner: value as f32,
                    })
                } else {
                    Value::Float64(Float64 { inner: value })
                }
            }
            ValueType::Boolean => {
                let value = match lexeme.to_string().parse::<bool>() {
                    Ok(i) => i,
                    Err(e) => return Err(nom::Err::Failure::<LangError>(e.into())),
                };
                Value::Boolean(value)
            }
        };
        Ok(value)
    }

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
            value: value,
        }
    }
}
