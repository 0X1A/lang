#![allow(non_snake_case)]

use ast::expr::*;
use ast::stmt::*;
use visitor::*;

pub struct ASTPrinter;

impl Default for ASTPrinter {
    fn default() -> ASTPrinter {
        ASTPrinter {}
    }
}

// We just use default impl of Debug to print the AST
impl Visitor<Expr> for ASTPrinter {
    type Value = ();

    fn visit(&mut self, expr: &Expr) -> Self::Value {
        println!("{:#?}", expr);
    }
}

impl Visitor<Stmt> for ASTPrinter {
    type Value = ();

    fn visit(&mut self, stmt: &Stmt) -> Self::Value {
        println!("{:#?}", stmt);
    }
}
