use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::error::*;
use crate::lang::Lang;
use crate::syntax::token::Token;
use crate::token::{TokenType, TypeAnnotation};
use crate::value::{TypedValue, Value};

pub struct Parser<'a> {
    source_lines: Vec<&'a str>,
    tokens: Vec<Token<'a>>,
    cursor_position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenIR {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Value,
    pub line: u32,
    pub offset: usize,
}

impl TokenIR {
    fn from_token_two(token: &Token) -> TokenIR {
        TokenIR {
            token_type: token.token_type.clone(),
            lexeme: token.span.content.input.to_string(),
            value: token.value.clone(),
            line: token.span.begin.line,
            offset: token.span.begin.offset,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            source_lines: source.split('\n').collect(),
            tokens,
            cursor_position: 0,
        }
    }

    fn expression(&mut self) -> Result<Expr, LangError> {
        Ok(self.assignment()?)
    }

    fn and(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.equality()?;
        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(LogicalExpr {
                left: expr,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.and()?;
        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(LogicalExpr {
                left: expr,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, LangError> {
        let expr = self.or()?;
        if self.matches(&[TokenType::Equal]) {
            // Use equals for errors!
            let equals = self.previous();
            let value = self.assignment()?;
            match expr {
                Expr::Variable(variable_expr) => {
                    return Ok(Expr::Assign(Box::new(AssignExpr {
                        name: variable_expr.name,
                        expr: value,
                    })));
                }
                Expr::Get(get_expr) => {
                    return Ok(Expr::Set(Box::new(SetExpr {
                        name: get_expr.name.clone(),
                        object: get_expr.object,
                        value,
                    })));
                }
                Expr::Index(index_expr) => {
                    return Ok(Expr::SetArrayElement(Box::new(SetArrayElementExpr {
                        name: index_expr.from.clone(),
                        index: index_expr.index,
                        value,
                    })));
                }
                _ => {
                    return Err(Lang::error_ir(
                        equals.line,
                        &equals.lexeme,
                        "Invalid assignment target",
                    ));
                }
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LangError> {
        let mut expression = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expression = Expr::Binary(Box::new(BinaryExpr {
                left: expression,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.addition()?;
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.multiplication()?;
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator: operator.token_type,
                right,
            }));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LangError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(UnaryExpr {
                operator: operator.token_type,
                right,
            })));
        }
        Ok(self.call()?)
    }

    fn call(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.primary()?;
        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(&expr)?;
            } else if self.matches(&[TokenType::Dot]) {
                let name =
                    self.pop_expect(&TokenType::Identifier, "Expected property name after '.'")?;
                expr = Expr::Get(Box::new(GetExpr {
                    name: name.lexeme,
                    object: expr,
                }));
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, expr: &Expr) -> Result<Expr, LangError> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        arguments.shrink_to_fit();
        self.pop_expect(
            &TokenType::RightParen,
            "Expect ')' after function arguments",
        )?;
        Ok(Expr::Call(Box::new(CallExpr {
            callee: expr.clone(),
            arguments,
        })))
    }

    fn primary(&mut self) -> Result<Expr, LangError> {
        if self.matches(&[TokenType::SelfIdent]) {
            return Ok(Expr::SelfIdent(Box::new(SelfIdentExpr {
                keyword: self.previous().lexeme,
            })));
        }
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Boolean(false),
                TypeAnnotation::Bool,
            )))));
        } else if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Boolean(true),
                TypeAnnotation::Bool,
            )))));
        } else if self.matches(&[TokenType::Unit]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Unit,
                TypeAnnotation::Unit,
            )))));
        } else if self.matches(&[TokenType::Integer]) {
            let value = self.previous().value;
            // Go back a 3 positions as we're cursor_positionly at the SemiColon
            if self.check(&TokenType::SemiColon) && self.get_previous_index() > 2 {
                if let Some(type_annotation_token) = self.token_at(self.get_previous_index() - 2) {
                    if let TokenType::Type(type_annotation) = type_annotation_token.token_type {
                        match type_annotation {
                            TypeAnnotation::I32 => {
                                return Ok(Expr::Literal(Box::new(LiteralExpr::new(
                                    TypedValue::new(value, TypeAnnotation::I32),
                                ))));
                            }
                            TypeAnnotation::I64 => {
                                return Ok(Expr::Literal(Box::new(LiteralExpr::new(
                                    TypedValue::new(value, TypeAnnotation::I64),
                                ))));
                            }
                            _ => {}
                        }
                    }
                }
            }
            match value {
                Value::Int32(_) => {
                    return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                        value,
                        TypeAnnotation::I32,
                    )))));
                }
                Value::Int64(_) => {
                    return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                        value,
                        TypeAnnotation::I64,
                    )))));
                }
                _ => {}
            }
        } else if self.matches(&[TokenType::Float]) {
            if self.check(&TokenType::SemiColon) && self.get_previous_index() > 2 {
                if let Some(type_annotation_token) = self.token_at(self.get_previous_index() - 2) {
                    if let TokenType::Type(type_annotation) = type_annotation_token.token_type {
                        match type_annotation {
                            TypeAnnotation::F32 => {
                                return Ok(Expr::Literal(Box::new(LiteralExpr::new(
                                    TypedValue::new(self.previous().value, TypeAnnotation::F32),
                                ))));
                            }
                            TypeAnnotation::F64 => {
                                return Ok(Expr::Literal(Box::new(LiteralExpr::new(
                                    TypedValue::new(self.previous().value, TypeAnnotation::F64),
                                ))));
                            }
                            _ => {}
                        }
                    }
                }
            }
            return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                self.previous().value,
                TypeAnnotation::F64,
            )))));
        } else if self.matches(&[TokenType::String]) {
            return Ok(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                self.previous().value,
                TypeAnnotation::String,
            )))));
        } else if self.matches(&[TokenType::Identifier]) {
            if self.check(&TokenType::LeftBracket) {
                let from = self.previous();
                self.pop_expect(&TokenType::LeftBracket, "Expected '[' after identifier")?;
                let index = self.expression()?;
                self.pop_expect(
                    &TokenType::RightBracket,
                    "Expected ']' after index expression",
                )?;
                return Ok(Expr::Index(Box::new(IndexExpr {
                    from: from.lexeme,
                    index,
                })));
            } else if self.matches(&[TokenType::PathSeparator]) {
                let enum_name = self.previous();
                let mut path_elements = Vec::new();
                loop {
                    if self.check(&TokenType::SemiColon) || self.is_at_end() {
                        break;
                    }
                    let path_item = self.pop_expect(&TokenType::Identifier, "expected ident")?;
                    path_elements.push(path_item.lexeme);
                }
                path_elements.shrink_to_fit();
                return Ok(Expr::EnumPath(Box::new(EnumPathExpr {
                    name: enum_name.lexeme,
                    path_items: path_elements,
                })));
            } else {
                return Ok(Expr::Variable(Box::new(VariableExpr {
                    name: self.previous().lexeme,
                })));
            }
        } else if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.pop_expect(&TokenType::RightParen, "Expect ')' after and expression")?;
            return Ok(Expr::Grouping(Box::new(GroupingExpr { expression: expr })));
        } else if self.matches(&[TokenType::LeftBracket]) {
            let mut array_type_annotation: Option<TokenType> = None;
            if let Some(array_type_token) = self.token_at(self.cursor_position - 3) {
                if let TokenType::Type(_) = array_type_token.token_type {
                    array_type_annotation = Some(array_type_token.clone().token_type);
                }
            }
            let mut elements = Vec::new();
            if !self.check(&TokenType::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.matches(&[TokenType::Comma]) {
                        break;
                    }
                }
            }
            self.pop_expect(
                &TokenType::RightBracket,
                "Expect ']' after array expression",
            )?;
            elements.shrink_to_fit();
            return Ok(Expr::Array(Box::new(ArrayExpr {
                type_annotation: array_type_annotation,
                elements,
            })));
        }
        Err(self.parse_error(&self.peek(), "Expected expression"))
    }

    /// Checks if the cursor_position token in source matches `token_type`, errors using the string `string`
    /// on failure.
    fn pop_expect(&mut self, token_type: &TokenType, string: &str) -> Result<TokenIR, LangError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(self.parse_error(
            &self.peek(),
            &format!(
                "{}, expected {} but found {}",
                string,
                token_type,
                &self.peek().token_type
            ),
        ))
    }

    fn parse_error(&self, token: &Token, error_mesg: &str) -> LangError {
        Lang::parser_error(
            self.source_lines[token.span.begin.line as usize - 1],
            token,
            error_mesg,
        )
    }

    /// Checks if next sequence of tokens matches those of the slice `tokens`, in respective order,
    /// advancing the cursor_position position in source on first match
    fn matches(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Checks the cursor_position token to see if it matches `token_type`. Returns false if
    /// the cursor_position token is EoF
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    /// Checks the token in the cursor_position position
    fn peek(&self) -> Token {
        self.tokens[self.cursor_position].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Advances the cursor_position position in source, returning the token at the previously 'cursor_position'
    /// position
    fn advance(&mut self) -> TokenIR {
        if !self.is_at_end() {
            self.cursor_position += 1;
        }
        self.previous()
    }

    fn get_previous_index(&self) -> usize {
        self.cursor_position - 1
    }

    fn token_at(&self, pos: usize) -> Option<Token> {
        if let Some(token) = self.tokens.get(pos) {
            return Some(token.clone());
        }
        None
    }

    /// Returns the token being the `cursor_position` position in source
    fn previous(&self) -> TokenIR {
        TokenIR::from_token_two(&self.tokens[self.get_previous_index()])
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }
            match self.peek().token_type {
                TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, LangError> {
        let value = self.expression()?;
        self.pop_expect(&TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Box::new(PrintStmt { expression: value })))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LangError> {
        let expr = self.expression()?;
        self.pop_expect(&TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(Box::new(ExpressionStmt {
            expression: expr,
        })))
    }

    fn if_statement(&mut self) -> Result<Stmt, LangError> {
        self.pop_expect(&TokenType::LeftParen, "Expect '(' after an 'if'.")?;
        let condition = self.expression()?;
        self.pop_expect(&TokenType::RightParen, "Expect ')' after 'if' condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };
        Ok(Stmt::If(Box::new(IfStmt {
            condition,
            else_branch,
            then_branch,
        })))
    }

    fn while_statement(&mut self) -> Result<Stmt, LangError> {
        self.pop_expect(&TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.pop_expect(&TokenType::RightParen, "Expect ')' after condition")?;
        let body = self.statement()?;
        Ok(Stmt::While(Box::new(WhileStmt { body, condition })))
    }

    fn for_statement(&mut self) -> Result<Stmt, LangError> {
        self.pop_expect(&TokenType::LeftParen, "Expect '(' after 'for'")?;
        let initializer;
        if self.matches(&[TokenType::SemiColon]) {
            initializer = None;
        } else if self.matches(&[TokenType::Let]) {
            initializer = Some(self.let_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = if !self.check(&TokenType::SemiColon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.pop_expect(&TokenType::SemiColon, "Expect ';' after loop condition")?;
        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.pop_expect(&TokenType::RightParen, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = Stmt::Block(Box::new(BlockStmt {
                statements: vec![
                    body,
                    Stmt::Expression(Box::new(ExpressionStmt {
                        expression: increment,
                    })),
                ],
            }));
        }
        if condition.is_none() {
            condition = Some(Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Boolean(true),
                TypeAnnotation::Bool,
            )))));
        }
        // These unwraps are fine as long as they are default set to something above!
        body = Stmt::While(Box::new(WhileStmt {
            condition: condition.unwrap(),
            body,
        }));
        if initializer.is_some() {
            body = Stmt::Block(Box::new(BlockStmt {
                statements: vec![initializer.unwrap(), body],
            }));
        }
        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, LangError> {
        let value = if !self.check(&TokenType::SemiColon) {
            self.expression()?
        } else {
            Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Unit,
                TypeAnnotation::Unit,
            ))))
        };
        self.pop_expect(&TokenType::SemiColon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(Box::new(ReturnStmt {
            keyword: "return".into(),
            value,
        })))
    }

    fn statement(&mut self) -> Result<Stmt, LangError> {
        if self.matches(&[TokenType::Break]) {
            return Ok(self.break_statement()?);
        }
        if self.matches(&[TokenType::For]) {
            return Ok(self.for_statement()?);
        }
        if self.matches(&[TokenType::If]) {
            return Ok(self.if_statement()?);
        }
        if self.matches(&[TokenType::Print]) {
            return Ok(self.print_statement()?);
        }
        if self.matches(&[TokenType::Return]) {
            return Ok(self.return_statement()?);
        }
        if self.matches(&[TokenType::While]) {
            return Ok(self.while_statement()?);
        }
        if self.matches(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Box::new(BlockStmt {
                statements: self.block()?,
            })));
        }
        Ok(self.expression_statement()?)
    }

    fn break_statement(&mut self) -> Result<Stmt, LangError> {
        self.pop_expect(&TokenType::SemiColon, "expected ';' after 'break'")?;
        Ok(Stmt::Break)
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        statements.shrink_to_fit();
        self.pop_expect(&TokenType::RightBrace, "Expect '}' after a block.")?;
        Ok(statements)
    }

    fn let_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self.pop_expect(&TokenType::Identifier, "Expected variable name")?;
        self.pop_expect(&TokenType::Colon, "Expected colon after variable name")?;
        let type_annotation_token = self.advance();
        if TypeAnnotation::check_token_type2(&type_annotation_token).is_err() {
            return Err(self.parse_error(
                &self.peek(),
                &format!(
                    "invalid type annotation, expected a type annotation but found {}",
                    &type_annotation_token.token_type.to_string()
                ),
            ));
        }
        let initializer = if self.matches(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::default_value(&type_annotation_token.token_type.to_type_annotation()?),
                type_annotation_token.token_type.to_type_annotation()?,
            ))))
        };
        self.pop_expect(
            &TokenType::SemiColon,
            "Expect ';' after variable declaration",
        )?;
        Ok(Stmt::Var(Box::new(VarStmt {
            initializer: Some(initializer),
            type_annotation: type_annotation_token.token_type.to_type_annotation()?,
            name: name.lexeme,
        })))
    }

    fn enum_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self.pop_expect(&TokenType::Identifier, "expected identifier")?;
        self.pop_expect(&TokenType::LeftBrace, "expected left brace")?;
        let mut item_list = Vec::new();
        let mut comma_count = 0;
        loop {
            if self.check(&TokenType::RightBrace) || self.is_at_end() {
                break;
            }
            let mut item = EnumItem {
                identifier: self
                    .pop_expect(&TokenType::Identifier, "expected identifier")?
                    .lexeme,
                initializer: None,
            };
            if self.matches(&[TokenType::Equal]) {
                item.initializer = Some(self.expression()?);
            }
            item_list.push(item);
            if self.matches(&[TokenType::Comma]) {
                comma_count += 1;
            }
            if comma_count < item_list.len() - 1 && !item_list.is_empty() {
                return Err(LangErrorType::new_parser_error(
                    "need comma after enum item".to_string(),
                ));
            }
        }
        self.pop_expect(&TokenType::RightBrace, "expected right brace")?;
        item_list.shrink_to_fit();
        Ok(Stmt::Enum(Box::new(EnumStmt {
            name: name.lexeme,
            item_list,
        })))
    }

    fn trait_impl_declaration(&mut self, trait_name: TokenIR) -> Result<Stmt, LangError> {
        let impl_trait_name =
            self.pop_expect(&TokenType::Identifier, "expected identifier after for")?;
        let mut trait_fn_declarations = Vec::new();
        self.pop_expect(
            &TokenType::LeftBrace,
            "trait_impl_decl expected left brace after identifier",
        )?;
        if !self.check(&TokenType::RightBrace) {
            loop {
                self.pop_expect(&TokenType::Fn, "expected fn after left brace")?;
                trait_fn_declarations.push(self.function("method")?);
                if self.check(&TokenType::RightBrace) {
                    break;
                }
            }
        }
        self.pop_expect(
            &TokenType::RightBrace,
            "expected right brace after function declarations",
        )?;
        trait_fn_declarations.shrink_to_fit();
        Ok(Stmt::ImplTrait(Box::new(ImplTraitStmt {
            impl_name: impl_trait_name.lexeme,
            trait_name: trait_name.lexeme,
            fn_declarations: trait_fn_declarations,
        })))
    }

    fn trait_declaration(&mut self) -> Result<Stmt, LangError> {
        let trait_name =
            self.pop_expect(&TokenType::Identifier, "expected identifier for trait")?;
        let mut trait_fn_declarations = Vec::new();
        self.pop_expect(
            &TokenType::LeftBrace,
            "expected left brace after identifier",
        )?;
        if !self.check(&TokenType::SemiColon) {
            loop {
                self.pop_expect(&TokenType::Fn, "expected fn after left brace")?;
                trait_fn_declarations.push(self.trait_function_declaration()?);
                self.pop_expect(
                    &TokenType::SemiColon,
                    "expected ; after trait function declaration",
                )?;
                if self.check(&TokenType::RightBrace) {
                    break;
                }
            }
        }
        self.pop_expect(
            &TokenType::RightBrace,
            "expected right brace after function declarations",
        )?;
        trait_fn_declarations.shrink_to_fit();
        Ok(Stmt::Trait(Box::new(TraitStmt {
            name: trait_name.lexeme,
            trait_fn_declarations,
        })))
    }

    fn impl_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self.pop_expect(&TokenType::Identifier, "expected identifier")?;
        if self.matches(&[TokenType::For]) {
            Ok(self.trait_impl_declaration(name)?)
        } else {
            Ok(self.method_impl_declaration(name)?)
        }
    }

    fn method_impl_declaration(&mut self, name: TokenIR) -> Result<Stmt, LangError> {
        let mut fn_declarations = Vec::new();
        self.pop_expect(
            &TokenType::LeftBrace,
            "expected left brace after identifier",
        )?;
        if !self.check(&TokenType::RightBrace) {
            loop {
                self.pop_expect(&TokenType::Fn, "expected fn after left brace")?;
                fn_declarations.push(self.function("method")?);
                if self.check(&TokenType::RightBrace) {
                    break;
                }
            }
        }
        self.pop_expect(
            &TokenType::RightBrace,
            "expected right brace after function declarations",
        )?;
        fn_declarations.shrink_to_fit();
        Ok(Stmt::Impl(Box::new(ImplStmt {
            name: name.lexeme,
            fn_declarations,
        })))
    }

    fn trait_function_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self.pop_expect(&TokenType::Identifier, "function: Expect function name")?;

        self.pop_expect(&TokenType::LeftParen, "Expect '(' after function name.")?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let identifier =
                    self.pop_expect(&TokenType::Identifier, "Expect parameter name")?;
                self.pop_expect(&TokenType::Colon, "Expected colon after paramter name")?;
                let type_annotation_token = self.advance();
                if TypeAnnotation::check_token_type2(&type_annotation_token).is_err() {
                    return Err(self.parse_error(
                        &self.peek(),
                        &format!(
                            "invalid type annotation, expected a type annotation but found {}",
                            &type_annotation_token.token_type.to_string()
                        ),
                    ));
                }
                // We only pass down the type annotation
                parameters.push(VariableData::new(
                    identifier.lexeme,
                    type_annotation_token.token_type.to_type_annotation()?,
                ));
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.pop_expect(&TokenType::RightParen, "Expect ')' after parameter list.")?;
        self.pop_expect(&TokenType::ReturnType, "Expected '->' after ')'")?;
        let return_type_annotation_token = self.advance();
        if TypeAnnotation::check_token_type2(&return_type_annotation_token).is_err() {
            return Err(self.parse_error(
                &self.peek(),
                &format!(
                    "invalid type annotation, expected a type annotation but found {}",
                    &return_type_annotation_token.token_type.to_string()
                ),
            ));
        }
        parameters.shrink_to_fit();
        Ok(Stmt::TraitFunction(Box::new(TraitFunctionStmt {
            name: name.lexeme,
            return_type: return_type_annotation_token
                .token_type
                .to_type_annotation()?,
            params: parameters,
        })))
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LangError> {
        let name = self.pop_expect(
            &TokenType::Identifier,
            &format!("function: Expect {} name", kind),
        )?;

        self.pop_expect(
            &TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let identifier =
                    self.pop_expect(&TokenType::Identifier, "Expect parameter name")?;
                self.pop_expect(&TokenType::Colon, "Expected colon after paramter name")?;
                let type_annotation_token = self.advance();
                if let Err(_) = TypeAnnotation::check_token_type2(&type_annotation_token) {
                    return Err(self.parse_error(
                        &self.peek(),
                        &format!(
                            "invalid type annotation, expected a type annotation but found {}",
                            &type_annotation_token.token_type.to_string()
                        ),
                    ));
                }
                // We only pass down the type annotation
                parameters.push(VariableData::new(
                    identifier.lexeme,
                    type_annotation_token.token_type.to_type_annotation()?,
                ));
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.pop_expect(&TokenType::RightParen, "Expect ')' after parameter list.")?;
        self.pop_expect(&TokenType::ReturnType, "Expected '->' after ')'")?;
        let return_type_annotation_token = self.advance();
        if TypeAnnotation::check_token_type2(&return_type_annotation_token).is_err() {
            return Err(self.parse_error(
                &self.peek(),
                &format!(
                    "invalid type annotation, expected a type annotation but found {}",
                    &return_type_annotation_token.token_type.to_string()
                ),
            ));
        }
        self.pop_expect(
            &TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        parameters.shrink_to_fit();
        Ok(Stmt::Function(Box::new(FunctionStmt {
            name: name.lexeme,
            return_type: return_type_annotation_token.token_type,
            params: parameters,
            body,
        })))
    }

    fn struct_declaration(&mut self) -> Result<Stmt, LangError> {
        let name = self.pop_expect(&TokenType::Identifier, "Expected struct name")?;
        self.pop_expect(&TokenType::LeftBrace, "Expected '{' before struct body")?;

        let mut fields = Vec::new();
        let mut comma_count = 0;
        loop {
            if self.check(&TokenType::RightBrace) || self.is_at_end() {
                break;
            }
            let field = self.pop_expect(&TokenType::Identifier, "Expected identifier")?;
            self.pop_expect(&TokenType::Colon, "Expected ':' after field identifier")?;
            let type_annotation = self.advance();
            if TypeAnnotation::check_token_type2(&type_annotation).is_err() {
                return Err(self.parse_error(
                    &self.peek(),
                    &format!(
                        "invalid type annotation, expected a type annotation but found {}",
                        &type_annotation.token_type.to_string()
                    ),
                ));
            }
            if self.matches(&[TokenType::Comma]) {
                comma_count += 1;
            }
            fields.push(VariableData::new(
                field.lexeme,
                type_annotation.token_type.to_type_annotation()?,
            ));
            if comma_count < fields.len() - 1 && !fields.is_empty() {
                return Err(LangErrorType::new_parser_error(
                    "need comma after field declaration".to_string(),
                ));
            }
        }

        self.pop_expect(&TokenType::RightBrace, "Expected '}' after struct body")?;
        fields.shrink_to_fit();
        Ok(Stmt::Struct(Box::new(StructStmt {
            fields,
            name: name.lexeme,
        })))
    }

    fn declaration(&mut self) -> Result<Stmt, LangError> {
        if self.matches(&[TokenType::Struct]) {
            match self.struct_declaration() {
                Ok(decl) => {
                    return Ok(decl);
                }
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        if self.matches(&[TokenType::Enum]) {
            self.enum_declaration()?;
        }
        if self.matches(&[TokenType::Trait]) {
            return self.trait_declaration();
        }
        if self.matches(&[TokenType::Impl]) {
            return self.impl_declaration();
        }
        if self.matches(&[TokenType::Fn]) {
            return self.function("function");
        }
        if self.matches(&[TokenType::Let]) {
            match self.let_declaration() {
                Ok(decl) => {
                    return Ok(decl);
                }
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        match self.statement() {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }
}
