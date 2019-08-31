pub mod expr;
pub mod stmt;

use self::expr::*;

pub enum ASTTypes {
    Expr(Expr),
    Stmt,
}
