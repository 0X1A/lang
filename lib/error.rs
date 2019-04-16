extern crate failure;
extern crate log;

#[derive(Hash, PartialEq, Eq)]
pub enum ErrMessage {
    ExpectValueType(String),
    ExpectExpr(String),
    ExpectStmt(String),
    IncorrectIndexType(String),
}

#[inline]
pub fn error_message(msg_type: &ErrMessage) -> String {
    use error::ErrMessage::*;
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

use std::{
    error::Error,
    fmt::{self, Debug},
    io, num, str, time,
};

#[derive(Fail)]
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

#[derive(Fail, Debug)]
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

#[derive(Fail)]
pub enum LangError {
    #[fail(display = "Parser error: {}", reason)]
    ParserError { reason: String },
    #[fail(display = "IIE: {}", reason)]
    InternalError { reason: String },
    #[fail(display = "Runtime error: {}", subtype)]
    RuntimeError { subtype: RuntimeErrorType },
    #[fail(display = "Control flow: {}", subtype)]
    ControlFlow { subtype: ControlFlow },
}

impl Debug for LangError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LangError::RuntimeError { subtype } => write!(f, "runtime error {:?}", subtype),
            LangError::InternalError { reason } => write!(f, "IIE {:?}", reason),
            LangError::ParserError { reason } => write!(f, "parser error {:?}", reason),
            LangError::ControlFlow { subtype } => {
                write!(f, "{} must be used within a loop", subtype)
            }
        }
    }
}

pub enum LangErrorType {
    ParserError,
    RuntimeError,
}

impl LangError {
    pub fn new_parser_error(reason: String) -> LangError {
        LangError::ParserError { reason }
    }

    pub fn new_iie_error(reason: String) -> LangError {
        LangError::InternalError { reason }
    }

    pub fn new_runtime_error(subtype: RuntimeErrorType) -> LangError {
        LangError::RuntimeError { subtype }
    }
}

impl From<io::Error> for LangError {
    fn from(err: io::Error) -> LangError {
        LangError::RuntimeError {
            subtype: {
                RuntimeErrorType::IoError {
                    reason: err.description().to_string(),
                }
            },
        }
    }
}

impl From<log::SetLoggerError> for LangError {
    fn from(err: log::SetLoggerError) -> LangError {
        LangError::RuntimeError {
            subtype: {
                RuntimeErrorType::LogError {
                    reason: err.description().to_string(),
                }
            },
        }
    }
}

impl From<num::ParseFloatError> for LangError {
    fn from(err: num::ParseFloatError) -> LangError {
        LangError::ParserError {
            reason: err.description().to_string(),
        }
    }
}

impl From<num::ParseIntError> for LangError {
    fn from(err: num::ParseIntError) -> LangError {
        LangError::ParserError {
            reason: err.description().to_string(),
        }
    }
}

impl From<str::ParseBoolError> for LangError {
    fn from(err: str::ParseBoolError) -> LangError {
        LangError::ParserError {
            reason: err.description().to_string(),
        }
    }
}

impl From<time::SystemTimeError> for LangError {
    fn from(err: time::SystemTimeError) -> LangError {
        LangError::RuntimeError {
            subtype: {
                RuntimeErrorType::GenericError {
                    reason: err.description().to_string(),
                }
            },
        }
    }
}
