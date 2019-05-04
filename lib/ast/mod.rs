pub mod expr;
pub mod stmt;

use self::expr::*;
use crate::token::*;

pub enum ASTTypes {
    Expr(Expr),
    Token(Token),
    Stmt,
}
