pub mod expr;
pub mod stmt;

use self::expr::*;
use token::*;

pub enum ASTTypes {
    Expr(Expr),
    Token(Token),
    Stmt,
}
