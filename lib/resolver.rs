extern crate log;

use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::error::*;
use crate::interpreter::*;
use crate::lang::*;
use crate::token::*;
use crate::value::*;
use crate::visitor::*;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum FunctionType {
    None,
    Function,
}

#[derive(Debug)]
pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function_type: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        let mut scopes = Vec::new();
        scopes.push(HashMap::new());
        Resolver {
            interpreter,
            scopes,
            current_function_type: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), LangError> {
        for stmt in stmts {
            self.resolve_statement(&stmt)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<(), LangError> {
        Ok(self.visit_stmt(stmt)?)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LangError> {
        Ok(self.visit_expr(expr)?)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
        debug!("{}:{} Begin_scope: {:?}", file!(), line!(), self.scopes);
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
        debug!("{}:{} End_scope: {:?}", file!(), line!(), self.scopes);
    }

    // TODO: Figure out how to handle trait implementations' declarations
    /// Declare token in latest scope
    fn declare(&mut self, name: &str) -> Result<(), LangError> {
        assert!(!self.scopes.is_empty());
        // Our scopes are a stack, check the last element for the token being declared
        self.scopes.last_mut().map_or(
            Err(LangErrorType::new_iie_error(
                "Tried to declare with no scopes 🤔".to_string(),
            )),
            |last| {
                if last.contains_key(name) {
                    // See TODO above!
                    /*                     Err(LangErrorType::new_iie_error(format!(
                        "Variable with the name '{}' already declared in this scope",
                        name.lexeme
                    ))) */
                    Ok(())
                } else {
                    last.insert(name.into(), false);
                    Ok(())
                }
            },
        )
    }

    /// Defines `name` in current scope
    fn define(&mut self, name: &str) {
        debug!("{}:{} Defining {:?} as in scope", file!(), line!(), name);
        assert!(!self.scopes.is_empty());
        if let Some(ref mut last) = self.scopes.last_mut() {
            last.insert(name.into(), true);
        }
    }

    fn resolve_function(
        &mut self,
        function: &FunctionStmt,
        fn_type: FunctionType,
    ) -> Result<(), LangError> {
        let enclosing = self.current_function_type.clone();
        self.current_function_type = fn_type.clone();
        self.begin_scope();
        for param in &function.params {
            self.declare(&param.identifier)?;
            self.define(&param.identifier);
        }

        self.resolve(&function.body)?;
        self.end_scope();
        self.current_function_type = enclosing;
        Ok(())
    }

    fn resolve_local(&mut self, name: &str) {
        debug!(
            "{}:{} Attempting to resolve expr '{:?}' within scopes '{:?}'",
            file!(),
            line!(),
            name,
            self.scopes
        );
        for scope_index in (0..self.scopes.len()).rev() {
            if self.scopes[scope_index].contains_key(name) {
                self.interpreter.resolve(name, scope_index);
                return;
            }
        }
    }
}

impl<'a> Visitor<()> for Resolver<'a> {
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), LangError> {
        Ok(visit_expr(self, expr)?)
    }
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), LangError> {
        Ok(visit_stmt(self, stmt)?)
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> Result<(), LangError> {
        self.resolve_expr(&assign.expr)?;
        self.resolve_local(&assign.name);
        Ok(())
    }
    fn visit_binary(&mut self, binary: &BinaryExpr) -> Result<(), LangError> {
        self.resolve_expr(&binary.left)?;
        Ok(self.resolve_expr(&binary.right)?)
    }
    fn visit_call(&mut self, call: &CallExpr) -> Result<(), LangError> {
        self.resolve_expr(&call.callee)?;
        for arg in &call.arguments {
            self.resolve_expr(&arg)?;
        }
        Ok(())
    }
    fn visit_get(&mut self, get: &GetExpr) -> Result<(), LangError> {
        Ok(self.resolve_expr(&get.object)?)
    }
    fn visit_enum_path(&mut self, _: &EnumPathExpr) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_grouping(&mut self, grouping: &GroupingExpr) -> Result<(), LangError> {
        Ok(self.resolve_expr(&grouping.expression)?)
    }
    fn visit_literal(&mut self, _: &LiteralExpr) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_logical(&mut self, logical: &LogicalExpr) -> Result<(), LangError> {
        self.resolve_expr(&logical.left)?;
        Ok(self.resolve_expr(&logical.right)?)
    }
    fn visit_set(&mut self, set: &SetExpr) -> Result<(), LangError> {
        self.resolve_expr(&set.value)?;
        Ok(self.resolve_expr(&set.object)?)
    }
    fn visit_unary(&mut self, unary: &UnaryExpr) -> Result<(), LangError> {
        Ok(self.resolve_expr(&unary.right)?)
    }
    fn visit_array(&mut self, array: &ArrayExpr) -> Result<(), LangError> {
        for item in array.elements.iter() {
            self.resolve_expr(&item)?;
        }
        Ok(())
    }
    fn visit_index(&mut self, index: &IndexExpr) -> Result<(), LangError> {
        Ok(self.resolve_expr(&index.index)?)
    }
    fn visit_set_array_element(&mut self, _: &SetArrayElementExpr) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_variable(&mut self, variable: &VariableExpr) -> Result<(), LangError> {
        if let Some(last) = self.scopes.last() {
            if let Some(value) = last.get(&variable.name) {
                if !(*value) {
                    return Err(LangErrorType::new_iie_error(format!(
                        "the value with identifier {} was not in scope",
                        variable.name
                    )));
                }
            }
        }
        self.resolve_local(&variable.name);
        Ok(())
    }
    fn visit_self_ident(&mut self, self_ident: &SelfIdentExpr) -> Result<(), LangError> {
        self.resolve_local(&self_ident.keyword);
        Ok(())
    }

    // stmt
    fn visit_break(&mut self) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_enum(&mut self, _: &EnumStmt) -> Result<(), LangError> {
        unimplemented!()
    }
    fn visit_impl(&mut self, impl_stmt: &ImplStmt) -> Result<(), LangError> {
        for fn_decl_statement in &impl_stmt.fn_declarations {
            self.resolve_statement(&fn_decl_statement)?;
        }
        Ok(())
    }
    fn visit_impl_trait(&mut self, _: &ImplTraitStmt) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_block(&mut self, block: &BlockStmt) -> Result<(), LangError> {
        self.begin_scope();
        self.resolve(&block.statements)?;
        self.end_scope();
        Ok(())
    }
    fn visit_struct(&mut self, struct_stmt: &StructStmt) -> Result<(), LangError> {
        self.declare(&struct_stmt.name)?;
        self.begin_scope();
        if let Some(ref mut last_scope) = self.scopes.last_mut() {
            last_scope.insert("self".to_string(), true);
        }
        self.end_scope();
        self.define(&struct_stmt.name);
        Ok(())
    }
    fn visit_expression(&mut self, expr: &ExpressionStmt) -> Result<(), LangError> {
        Ok(self.resolve_expr(&expr.expression)?)
    }
    fn visit_trait(&mut self, trait_stmt: &TraitStmt) -> Result<(), LangError> {
        for fn_declarations in &trait_stmt.trait_fn_declarations {
            self.resolve_statement(&fn_declarations)?;
        }
        Ok(())
    }
    fn visit_trait_function(&mut self, trait_fn_stmt: &TraitFunctionStmt) -> Result<(), LangError> {
        self.declare(&trait_fn_stmt.name)?;
        self.define(&trait_fn_stmt.name);
        Ok(())
    }
    fn visit_function(&mut self, function_stmt: &FunctionStmt) -> Result<(), LangError> {
        self.declare(&function_stmt.name)?;
        self.define(&function_stmt.name);
        Ok(self.resolve_function(&function_stmt, FunctionType::Function)?)
    }
    fn visit_if(&mut self, if_stmt: &IfStmt) -> Result<(), LangError> {
        self.resolve_expr(&if_stmt.condition)?;
        self.resolve_statement(&if_stmt.then_branch)?;
        if let Some(ref else_branch) = if_stmt.else_branch {
            self.resolve_statement(else_branch)?;
        }
        Ok(())
    }
    fn visit_print(&mut self, print_stmt: &PrintStmt) -> Result<(), LangError> {
        Ok(self.resolve_expr(&print_stmt.expression)?)
    }
    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> Result<(), LangError> {
        if self.current_function_type == FunctionType::None {
            return Err(Lang::error_s(
                &return_stmt.keyword,
                "Cannot return from top-level code",
            ));
        }
        if return_stmt.value
            != Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                Value::Unit,
                TypeAnnotation::Unit,
            ))))
        {
            self.resolve_expr(&return_stmt.value)?;
        }
        Ok(())
    }
    fn visit_var(&mut self, var_stmt: &VarStmt) -> Result<(), LangError> {
        self.declare(&var_stmt.name)?;
        if let Some(ref initializer) = var_stmt.initializer {
            self.resolve_expr(initializer)?;
        }
        self.define(&var_stmt.name);
        Ok(())
    }
    fn visit_while(&mut self, while_stmt: &WhileStmt) -> Result<(), LangError> {
        self.resolve_expr(&while_stmt.condition)?;
        self.resolve_statement(&while_stmt.body)?;
        Ok(())
    }
    fn visit_import(&mut self, _: &ImportStmt) -> Result<(), LangError> {
        Ok(())
    }
}
