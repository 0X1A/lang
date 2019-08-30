use crate::ast::stmt::*;
use crate::error::*;
use crate::syntax::ast::expr::*;
use crate::syntax::token::TokenTwo;
use crate::token::{TokenType, TypeAnnotation};
use crate::value::{TypedValue, Value};

pub struct Parser<'a> {
    tokens: Vec<TokenTwo<'a>>,
    cursor_position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<TokenTwo<'a>>) -> Parser<'a> {
        Parser {
            tokens,
            cursor_position: 0,
        }
    }

    fn expression(&mut self) -> Result<Expr, LangError> {
        Ok(self.assignment()?)
    }

    fn and(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn or(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn assignment(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn equality(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn comparison(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn addition(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn multiplication(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn unary(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn call(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn finish_call(&mut self, expr: &Expr) -> Result<Expr, LangError> {
        unimplemented!()
    }

    fn primary(&mut self) -> Result<Expr, LangError> {
        unimplemented!()
    }

    /// Checks if the current token in source matches `token_type`, errors using the string `string`
    /// on failure.
    fn pop_expect(&mut self, token_type: &TokenType, string: &str) -> Result<TokenTwo, LangError> {
        unimplemented!()
    }

    /// Checks if next sequence of tokens matches those of the slice `tokens`, in respective order,
    /// advancing the current position in source on first match
    fn matches(&mut self, tokens: &[TokenType]) -> bool {
        unimplemented!()
    }

    /// Checks the current token to see if it matches `token_type`. Returns false if
    /// the current token is EoF
    fn check(&self, token_type: &TokenType) -> bool {
        unimplemented!()
    }

    /// Checks the token in the current position
    fn peek(&self) -> TokenTwo {
        unimplemented!()
    }

    fn is_at_end(&self) -> bool {
        unimplemented!()
    }

    /// Advances the current position in source, returning the token at the previously 'current'
    /// position
    fn advance(&mut self) -> TokenTwo {
        unimplemented!()
    }

    fn get_previous_index(&self) -> usize {
        self.cursor_position - 1
    }

    fn token_at(&self, pos: usize) -> Option<TokenTwo> {
        unimplemented!()
    }

    /// Returns the token being the `current` position in source
    fn previous(&self) -> TokenTwo {
        unimplemented!()
    }

    fn synchronize(&mut self) {
        unimplemented!()
    }

    fn print_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn expression_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn if_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn while_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn for_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn return_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn break_statement(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LangError> {
        unimplemented!()
    }

    fn let_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn enum_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn trait_impl_declaration(&mut self, trait_name: TokenTwo) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn trait_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn impl_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn method_impl_declaration(&mut self, name: TokenTwo) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn trait_function_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn struct_declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    fn declaration(&mut self) -> Result<Stmt, LangError> {
        unimplemented!()
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }
}
