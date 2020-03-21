#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

extern crate inkwell;

pub mod ast;
pub mod ast_printer;
pub mod depresolver;
pub mod env;
pub mod error;
pub mod interpreter;
pub mod lang;
pub mod mem;
pub mod resolver;
pub mod syntax;
pub mod token;
pub mod type_checker;
pub mod value;
pub mod value_traits;
pub mod visitor;
