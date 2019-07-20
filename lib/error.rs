use std::{
    error::Error,
    fmt::{self, Debug},
    io, num, str, time,
};

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

#[derive(Fail, Clone)]
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

#[derive(Fail, Debug, Clone)]
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

#[derive(Fail, Clone)]
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
            LangError::InternalError { reason } => write!(
                f,
                "IIE {:?}\nPlease report this issue at: {}",
                reason, ISSUES_URL
            ),
            LangError::ParserError { reason } => write!(f, "parser error {:?}", reason),
            LangError::ControlFlow { subtype } => {
                write!(f, "{} must be used within a loop", subtype)
            }
        }
    }
}

impl LangError {
    pub fn new_parser_error(reason: String) -> LangErrorTwo {
        LangErrorTwo::from(LangError::ParserError { reason })
    }

    pub fn new_iie_error(reason: String) -> LangErrorTwo {
        LangErrorTwo::from(LangError::InternalError { reason })
    }

    pub fn new_runtime_error(subtype: RuntimeErrorType) -> LangErrorTwo {
        LangErrorTwo::from(LangError::RuntimeError { subtype })
    }
}

impl From<io::Error> for LangErrorTwo {
    fn from(err: io::Error) -> LangErrorTwo {
        LangErrorTwo::from(LangError::RuntimeError {
            subtype: {
                RuntimeErrorType::IoError {
                    reason: err.description().to_string(),
                }
            },
        })
    }
}

impl From<log::SetLoggerError> for LangErrorTwo {
    fn from(err: log::SetLoggerError) -> LangErrorTwo {
        LangErrorTwo::from(LangError::RuntimeError {
            subtype: {
                RuntimeErrorType::LogError {
                    reason: err.description().to_string(),
                }
            },
        })
    }
}

impl From<num::ParseFloatError> for LangErrorTwo {
    fn from(err: num::ParseFloatError) -> LangErrorTwo {
        LangErrorTwo::from(LangError::ParserError {
            reason: err.description().to_string(),
        })
    }
}

impl From<num::ParseIntError> for LangErrorTwo {
    fn from(err: num::ParseIntError) -> LangErrorTwo {
        LangErrorTwo::from(LangError::ParserError {
            reason: err.description().to_string(),
        })
    }
}

impl From<str::ParseBoolError> for LangErrorTwo {
    fn from(err: str::ParseBoolError) -> LangErrorTwo {
        LangErrorTwo::from(LangError::ParserError {
            reason: err.description().to_string(),
        })
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

#[derive(Debug)]
pub struct LangErrorTwo {
    pub context: Context<LangError>,
}

impl fmt::Display for LangErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.context.get_context(), f)
    }
}

impl LangErrorTwo {
    pub fn from(e: LangError) -> LangErrorTwo {
        LangErrorTwo {
            context: Context::new(e),
        }
    }
}

impl Fail for LangErrorTwo {
    fn cause(&self) -> Option<&Fail> {
        self.context.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.context.backtrace()
    }
}
