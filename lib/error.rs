use std::{
    error::Error,
    fmt::{self, Debug},
    io, num, str, time,
};

use nom::error::ErrorKind;
use nom::error::ParseError;

use failure::{Backtrace, Context, Fail};
const ISSUES_URL: &str = "https://github.com/0X1A/lang/issues";

#[derive(Hash, PartialEq, Eq)]
pub enum ErrMessage {
    ExpectValueType(String),
    ExpectExpr(String),
    ExpectStmt(String),
    IncorrectIndexType(String),
}

#[inline]
pub fn error_message(msg_type: &ErrMessage) -> String {
    use crate::error::ErrMessage::*;
    match msg_type {
        ExpectValueType(type_string) => format!("expected a {} value", type_string),
        ExpectExpr(expr_string) => format!("expected {} expression", expr_string),
        ExpectStmt(stmt_string) => format!("expected {} statement", stmt_string),
        IncorrectIndexType(type_string) => format!(
            "tried to index an array with an incorrect type '{}'",
            type_string
        ),
    }
}

#[derive(Fail, Clone, PartialEq, PartialOrd)]
pub enum RuntimeErrorType {
    #[fail(display = "Undefined variable: {}", reason)]
    UndefinedVariable { reason: String },
    #[fail(display = "Resolution error: {}", reason)]
    ResolutionError { reason: String },
    #[fail(display = "Call error: {}", reason)]
    CallError { reason: String },
    #[fail(display = "IO error: {}", reason)]
    IoError { reason: String },
    #[fail(display = "Log error: {}", reason)]
    LogError { reason: String },
    #[fail(display = "Function arity error: {}", reason)]
    FnArityError { reason: String },
    #[fail(display = "Invalid assignment type target: {}", reason)]
    InvalidTypeAssignmentError { reason: String },
    #[fail(display = "Invalid function argument type: {}", reason)]
    InvalidFunctionArgumentType { reason: String },
    #[fail(display = "Invalid function return type: {}", reason)]
    InvalidFunctionReturnType { reason: String },
    #[fail(display = "{}", reason)]
    GenericError { reason: String },
}

#[derive(Fail, Debug, Clone, PartialEq, PartialOrd)]
pub enum ControlFlow {
    #[fail(display = "Break")]
    Break,
}

impl Debug for RuntimeErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeErrorType::UndefinedVariable { reason }
            | RuntimeErrorType::ResolutionError { reason }
            | RuntimeErrorType::CallError { reason }
            | RuntimeErrorType::IoError { reason }
            | RuntimeErrorType::LogError { reason }
            | RuntimeErrorType::FnArityError { reason }
            | RuntimeErrorType::InvalidTypeAssignmentError { reason }
            | RuntimeErrorType::InvalidFunctionArgumentType { reason }
            | RuntimeErrorType::InvalidFunctionReturnType { reason }
            | RuntimeErrorType::GenericError { reason } => write!(f, "{}", reason),
        }
    }
}

#[derive(Fail, Clone, PartialEq, PartialOrd)]
pub enum LangErrorType {
    #[fail(display = "Parser error: {}", reason)]
    ParserError { reason: String },
    #[fail(display = "IIE: {}", reason)]
    InternalError { reason: String },
    #[fail(display = "Runtime error: {}", subtype)]
    RuntimeError { subtype: RuntimeErrorType },
    #[fail(display = "Control flow: {}", subtype)]
    ControlFlow { subtype: ControlFlow },
}

impl Debug for LangErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangErrorType::RuntimeError { subtype } => write!(f, "runtime error {:?}", subtype),
            LangErrorType::InternalError { reason } => write!(
                f,
                "IIE {:?}\nPlease report this issue at: {}",
                reason, ISSUES_URL
            ),
            LangErrorType::ParserError { reason } => write!(f, "parser error {:?}", reason),
            LangErrorType::ControlFlow { subtype } => {
                write!(f, "{:?} must be used within a loop", subtype)
            }
        }
    }
}

impl LangErrorType {
    pub fn new_parser_error(reason: String) -> LangError {
        LangError::from(LangErrorType::ParserError { reason })
    }

    pub fn new_iie_error(reason: String) -> LangError {
        LangError::from(LangErrorType::InternalError { reason })
    }

    pub fn new_runtime_error(subtype: RuntimeErrorType) -> LangError {
        LangError::from(LangErrorType::RuntimeError { subtype })
    }
}

impl From<io::Error> for LangError {
    fn from(err: io::Error) -> LangError {
        LangError::from(LangErrorType::RuntimeError {
            subtype: {
                RuntimeErrorType::IoError {
                    reason: err.description().to_string(),
                }
            },
        })
    }
}

impl From<log::SetLoggerError> for LangError {
    fn from(err: log::SetLoggerError) -> LangError {
        LangError::from(LangErrorType::RuntimeError {
            subtype: {
                RuntimeErrorType::LogError {
                    reason: err.description().to_string(),
                }
            },
        })
    }
}

impl From<num::ParseFloatError> for LangError {
    fn from(err: num::ParseFloatError) -> LangError {
        LangError::from(LangErrorType::ParserError {
            reason: err.description().to_string(),
        })
    }
}

impl From<num::ParseIntError> for LangError {
    fn from(err: num::ParseIntError) -> LangError {
        LangError::from(LangErrorType::ParserError {
            reason: err.description().to_string(),
        })
    }
}

impl From<str::ParseBoolError> for LangError {
    fn from(err: str::ParseBoolError) -> LangError {
        LangError::from(LangErrorType::ParserError {
            reason: err.description().to_string(),
        })
    }
}

impl From<time::SystemTimeError> for LangErrorType {
    fn from(err: time::SystemTimeError) -> LangErrorType {
        LangErrorType::RuntimeError {
            subtype: {
                RuntimeErrorType::GenericError {
                    reason: err.description().to_string(),
                }
            },
        }
    }
}

impl From<nom::Err<LangError>> for LangError {
    fn from(err: nom::Err<LangError>) -> LangError {
        match err {
            nom::Err::Incomplete(e) => {
                match e {
                    nom::Needed::Size(s) =>
                LangError::from(LangErrorType::ParserError {
                    reason: format!("parser did not have enough data for parsing, required a buffer of size: {}", s)
                }),
                    nom::Needed::Unknown => LangError::from(LangErrorType::ParserError { reason: "parser did not have enough data for parsing, required a buffer of unknown size".into() }),
                }
            }
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}

pub struct LangError {
    pub context: Context<LangErrorType>,
}

impl fmt::Debug for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.context.get_context())
    }
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.context.get_context())
    }
}

impl LangError {
    pub fn from(e: LangErrorType) -> LangError {
        LangError {
            context: Context::new(e),
        }
    }
}

impl Fail for LangError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.context.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.context.backtrace()
    }
}

// TODO: Flesh this out for better errors
impl<Span: fmt::Display> ParseError<Span> for LangError {
    fn from_error_kind(input: Span, _: ErrorKind) -> Self {
        LangError::from(LangErrorType::ParserError {
            reason: format!("{}", input),
        })
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(input: Span, _: char) -> Self {
        Self::from_error_kind(input, ErrorKind::Char)
    }

    fn or(self, other: Self) -> Self {
        other
    }

    fn add_context(_input: Span, _ctx: &'static str, other: Self) -> Self {
        other
    }
}
