pub mod expr;

use self::expr::*;
use crate::token::*;

pub enum ASTTypes {
    Token(Token),
    Stmt,
}
