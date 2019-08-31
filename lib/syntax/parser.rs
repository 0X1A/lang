use crate::ast::stmt::*;
use crate::error::*;
use crate::lang::Lang;
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

    fn is_at_end(&self) -> bool {
        self.tokens[self.cursor_position].token_type == TokenType::Eof
    }

    fn pop_advance(&mut self) -> TokenTwo {
        if !self.is_at_end() {
            self.cursor_position += 1;
        }
        self.previous()
    }

    fn previous(&self) -> TokenTwo {
        self.tokens[self.cursor_position - 1].clone()
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.tokens[self.cursor_position].token_type == token_type
    }

    fn peek(&self) -> TokenTwo {
        self.tokens[self.cursor_position].clone()
    }

    fn pop_expect(&mut self, token_type: TokenType, err_str: &str) -> Result<TokenTwo, LangError> {
        if self.check(token_type) {
            return Ok(self.pop_advance());
        }
        Err(Lang::error2(&self.peek(), err_str))
    }

    fn expression(&mut self) -> Result<Expr, LangError> {
        Ok(self.assignment()?)
    }

    fn assignment(&mut self) -> Result<Expr, LangError> {
        let expr = self.or()?;
        if self.check(TokenType::Equal) {}
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LangError> {
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
