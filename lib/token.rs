use crate::error::*;
use crate::lang::*;
use crate::value::{Float64, Value};
use std::fmt::{self, Debug, Display};

// TODO: Revisit hashing Token
#[allow(clippy::derive_hash_xor_eq)]
#[derive(Clone, Debug, Hash, PartialOrd)]
/// Types we currently support
/// TODO: Check user types to make sure they're actually defined
pub enum TypeAnnotation {
    I32,
    I64,
    F32,
    F64,
    String,
    Bool,
    Trait,
    Unit,
    Fn,
    Array(Box<TypeAnnotation>),
    SelfIndex,
    User(String),
}

impl TypeAnnotation {
    pub fn is_array(&self) -> bool {
        if let TypeAnnotation::Array(_) = self {
            return true;
        }
        return false;
    }

    pub fn get_array_element_type(array: &TypeAnnotation) -> Result<TypeAnnotation, LangError> {
        if let TypeAnnotation::Array(element_type) = array {
            return Ok(*element_type.clone());
        }
        Err(LangError::new_iie_error(
            "failed to get array element type".to_string(),
        ))
    }

    pub fn from_token_type(token_type: &TokenType) -> Result<TypeAnnotation, LangError> {
        match token_type {
            TokenType::Type(type_annotation) => Ok(type_annotation.clone()),
            _ => Err(LangError::new_parser_error(
                "failed type annotation extraction".to_string(),
            )),
        }
    }

    /// Checks `token`'s `token_type` to ensure that it has been lexed as a type annotation
    pub fn check_token_type(token: &Token) -> Result<(), LangError> {
        match token.token_type {
            TokenType::Type(_) => Ok(()),
            _ => Err(Lang::error(
                &token,
                &format!(
                    "invalid type annotation expected a type annotation, found '{}'",
                    token.token_type.to_string()
                ),
            )),
        }
    }
}

impl GetTypeAnnotation for TypeAnnotation {
    fn get_type_annotation(&self) -> &TypeAnnotation {
        self
    }
}

pub trait GetTypeAnnotation {
    fn get_type_annotation(&self) -> &TypeAnnotation;
}

impl Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeAnnotation::I32 => write!(f, "i32"),
            TypeAnnotation::SelfIndex => write!(f, "self"),
            TypeAnnotation::I64 => write!(f, "i64"),
            TypeAnnotation::F32 => write!(f, "f32"),
            TypeAnnotation::F64 => write!(f, "f64"),
            TypeAnnotation::Bool => write!(f, "bool"),
            TypeAnnotation::Trait => write!(f, "trait"),
            TypeAnnotation::Unit => write!(f, "()"),
            TypeAnnotation::Fn => write!(f, "fn"),
            TypeAnnotation::String => write!(f, "String"),
            TypeAnnotation::Array(array) => write!(f, "Array<{}>", array.to_string()),
            TypeAnnotation::User(user_type) => write!(f, "{}", user_type.clone()),
        }
    }
}

impl PartialEq for TypeAnnotation {
    fn eq(&self, other: &TypeAnnotation) -> bool {
        match self {
            TypeAnnotation::I32 => match other {
                TypeAnnotation::I32 => true,
                _ => false,
            },
            TypeAnnotation::SelfIndex => match other {
                _ => false,
            },
            TypeAnnotation::I64 => match other {
                TypeAnnotation::I64 => true,
                _ => false,
            },
            TypeAnnotation::F32 => match other {
                TypeAnnotation::F32 => true,
                _ => false,
            },
            TypeAnnotation::F64 => match other {
                TypeAnnotation::F64 => true,
                _ => false,
            },
            TypeAnnotation::String => match other {
                TypeAnnotation::String => true,
                _ => false,
            },
            TypeAnnotation::Bool => match other {
                TypeAnnotation::Bool => true,
                _ => false,
            },
            TypeAnnotation::Fn => match other {
                _ => false,
            },
            TypeAnnotation::Trait => match other {
                _ => false,
            },
            TypeAnnotation::Array(lhs) => match other {
                TypeAnnotation::Array(rhs) => lhs == rhs,
                _ => false,
            },
            // TODO: User types don't have default values,
            // we initialize them as unit and then hope for the best
            // Best thing to do is not allow uninitialized structs.
            // That means we need struct initialization syntax
            TypeAnnotation::User(lhs) => match other {
                TypeAnnotation::User(rhs) => lhs == rhs,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::Unit => match other {
                TypeAnnotation::User(_) => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
        }
    }
}

impl Eq for TypeAnnotation {}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
/// All token types we currently extract from source text
pub enum TokenType {
    Break,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    Enum,
    RightBracket,
    Colon,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    ReturnType,
    LessEqual,
    Identifier,
    String,
    Float,
    Integer,
    And,
    Struct,
    Else,
    False,
    Fn,
    For,
    If,
    Unit,
    Ternary,
    Or,
    Print,
    Return,
    Impl,
    Trait,
    True,
    Let,
    While,
    PathSeparator,
    Type(TypeAnnotation),
    SelfIdent,
    Eof,
}

impl TokenType {
    pub fn to_type_annotation(&self) -> Result<TypeAnnotation, LangError> {
        match self {
            TokenType::Type(type_annotation) => Ok(type_annotation.clone()),
            _ => Err(LangError::new_parser_error(
                "Failed type annotation extraction".to_string(),
            )),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenType::Break => write!(f, "break"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Colon => write!(f, ":"),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::ReturnType => write!(f, "->"),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier => write!(f, "identifier"),
            TokenType::String => write!(f, "string"),
            TokenType::Float => write!(f, "float"),
            TokenType::Integer => write!(f, "integer"),
            TokenType::And => write!(f, "and"),
            TokenType::Struct => write!(f, "struct"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::PathSeparator => write!(f, "::"),
            TokenType::Fn => write!(f, "fn"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Unit => write!(f, "()"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Ternary => write!(f, "?"),
            TokenType::Impl => write!(f, "impl"),
            TokenType::Trait => write!(f, "trait"),
            TokenType::True => write!(f, "true"),
            TokenType::Let => write!(f, "let"),
            TokenType::While => write!(f, "while"),
            TokenType::SelfIdent => write!(f, "self"),
            TokenType::Type(type_annotation) => write!(f, "Type({})", type_annotation.to_string()),
            TokenType::Eof => write!(f, "EoF"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
/// Tokenized version of our source text
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Value,
    pub line: u64,
}

impl Token {
    /// Takes the toke type `token_type` and creates a Token with lexeme `lexeme` and line `line`
    pub fn from(token_type: TokenType, lexeme: &str, line: u64) -> Result<Token, LangError> {
        let value = match token_type {
            TokenType::String => Value::String(lexeme.to_string()),
            TokenType::Integer => {
                let integer_value = lexeme.to_string().parse::<i64>()?;
                if integer_value as i32 <= std::i32::MAX && integer_value as i32 >= std::i32::MIN {
                    Value::Int32(integer_value as i32)
                } else {
                    Value::Int64(integer_value)
                }
            }
            TokenType::Float => Value::Float64(Float64 {
                inner: lexeme.to_string().parse::<f64>()?,
            }),
            TokenType::Identifier => Value::Ident(lexeme.to_string()),
            TokenType::True | TokenType::False => {
                Value::Boolean(lexeme.to_string().parse::<bool>()?)
            }
            _ => Value::String(token_type.to_string()),
        };
        Ok(Token {
            token_type,
            lexeme: lexeme.to_string(),
            value,
            line,
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.token_type.to_string(), self.lexeme)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(TokenType '{:?}', Lexeme '{}', Value '{:?}')",
            self.token_type, self.lexeme, self.value
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
