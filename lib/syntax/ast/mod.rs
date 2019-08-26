pub mod expr;
pub mod stmt;

use self::expr::*;
use crate::token::*;

pub enum ASTTypes {
    Token(Token),
    Stmt,
}
