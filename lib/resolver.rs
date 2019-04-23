extern crate log;

use accept::*;
use ast::expr::*;
use ast::stmt::*;
use error::*;
use interpreter::*;
use lang::*;
use token::*;
use value::*;
use visitor::*;

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
        stmt.accept(self)?;
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LangError> {
        expr.accept(self)?;
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
        debug!("Resolver::begin_scope:\n{:?}", self.scopes);
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
        debug!("Resolver::end_scope:\n{:?}", self.scopes);
    }

    // TODO: Figure out how to handle trait implementations' declarations
    /// Declare token in latest scope
    fn declare(&mut self, name: &Token) -> Result<(), LangError> {
        assert!(!self.scopes.is_empty());
        // Our scopes are a stack, check the last element for the token being declared
        self.scopes.last_mut().map_or(
            Err(LangError::new_iie_error(
                "Tried to declare with no scopes 🤔".to_string(),
            )),
            |last| {
                if last.contains_key(&name.lexeme) {
                    // See TODO above!
                    /*                     Err(LangError::new_iie_error(format!(
                        "Variable with the name '{}' already declared in this scope",
                        name.lexeme
                    ))) */
                    Ok(())
                } else {
                    last.insert(name.lexeme.clone(), false);
                    Ok(())
                }
            },
        )
    }

    /// Defines `name` in current scope
    fn define(&mut self, name: &Token) {
        debug!("Resolver::define:\nDefining {:?} as in scope", name);
        assert!(!self.scopes.is_empty());
        if let Some(ref mut last) = self.scopes.last_mut() {
            last.insert(name.lexeme.clone(), true);
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

    fn resolve_local(&mut self, var: &Expr, name: &Token) {
        debug!(
            "Resolver::resolve_local:\nAttempting to resolve expr '{:?}' within scopes '{:?}'",
            var, self.scopes
        );
        for scope_index in (0..self.scopes.len()).rev() {
            if self.scopes[scope_index].contains_key(&name.lexeme) {
                self.interpreter.resolve(var, self.scopes.len() - 1);
                return;
            }
        }
    }
}

impl<'a> Visitor<Expr> for Resolver<'a> {
    type Value = Result<Value, LangError>;

    fn visit(&mut self, expr: &Expr) -> Self::Value {
        match expr {
            Expr::Binary(binary_expr) => {
                self.resolve_expr(&binary_expr.left)?;
                self.resolve_expr(&binary_expr.right)?;
                Ok(Value::Unit)
            }
            Expr::Call(call_expr) => {
                self.resolve_expr(&call_expr.callee)?;
                for arg in &call_expr.arguments {
                    self.resolve_expr(&arg)?;
                }
                Ok(Value::Unit)
            }
            Expr::Get(get_expr) => {
                self.resolve_expr(&get_expr.object)?;
                Ok(Value::Unit)
            }
            Expr::Unary(unary_expr) => {
                self.resolve_expr(&unary_expr.right)?;
                Ok(Value::Unit)
            }
            Expr::Logical(logical_expr) => {
                self.resolve_expr(&logical_expr.left)?;
                self.resolve_expr(&logical_expr.right)?;
                Ok(Value::Unit)
            }
            Expr::Set(set_expr) => {
                self.resolve_expr(&set_expr.value)?;
                self.resolve_expr(&set_expr.object)?;
                Ok(Value::Unit)
            }
            Expr::Grouping(grouping_expr) => {
                self.resolve_expr(&grouping_expr.expression)?;
                Ok(Value::Unit)
            }
            Expr::Literal(_) => Ok(Value::Unit),
            Expr::Assign(assign_expr) => {
                self.resolve_expr(&assign_expr.expr)?;
                self.resolve_local(expr, &assign_expr.name);
                Ok(Value::Unit)
            }
            Expr::Variable(var_expr) => {
                if let Some(last) = self.scopes.last() {
                    if let Some(value) = last.get(&var_expr.name.lexeme) {
                        if !(*value) {
                            return Err(LangError::new_iie_error(format!(
                                "the value with identifier {} was not in scope",
                                var_expr.name.lexeme
                            )));
                        }
                    }
                }
                self.resolve_local(expr, &var_expr.name);
                Ok(Value::Unit)
            }
            Expr::Index(index_expr) => {
                self.resolve_expr(&index_expr.index)?;
                Ok(Value::Unit)
            }
            Expr::Array(array_expr) => {
                for item in array_expr.elements.iter() {
                    self.resolve_expr(&item)?;
                }
                Ok(Value::Unit)
            }
            Expr::EnumPath(_) => Ok(Value::Unit),
            Expr::SetArrayElement(_) => Ok(Value::Unit),
            Expr::SelfIdent(self_ident_expr) => {
                self.resolve_local(expr, &self_ident_expr.keyword);
                Ok(Value::Unit)
            }
        }
    }
}

impl<'a> Visitor<Stmt> for Resolver<'a> {
    type Value = Result<(), LangError>;

    fn visit(&mut self, expr: &Stmt) -> Self::Value {
        match expr {
            Stmt::Enum(_) => unimplemented!(),
            Stmt::Break => Ok(()),
            Stmt::ImplTrait(_) => Ok(()),
            Stmt::Trait(ref trait_stmt) => {
                for fn_decl_statement in &trait_stmt.trait_fn_declarations {
                    self.resolve_statement(&fn_decl_statement)?;
                }
                Ok(())
            }
            Stmt::TraitFunction(ref trait_fn_stmt) => {
                self.declare(&trait_fn_stmt.name)?;
                self.define(&trait_fn_stmt.name);
                Ok(())
            }
            Stmt::Impl(ref impl_stmt) => {
                for fn_decl_statement in &impl_stmt.fn_declarations {
                    self.resolve_statement(&fn_decl_statement)?;
                }
                Ok(())
            }
            Stmt::Expression(expr_stmt) => {
                debug!("Resolver::visit Stmt::Expression\n");
                self.resolve_expr(&expr_stmt.expression)?;
                Ok(())
            }
            Stmt::If(if_stmt) => {
                debug!("Resolver::visit Stmt::If\n");
                self.resolve_expr(&if_stmt.condition)?;
                self.resolve_statement(&if_stmt.then_branch)?;
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.resolve_statement(else_branch)?;
                }
                Ok(())
            }
            Stmt::Block(block_stmt) => {
                debug!("Resolver::visit Stmt::Block\n");
                self.begin_scope();
                self.resolve(&block_stmt.statements)?;
                self.end_scope();
                Ok(())
            }
            Stmt::Struct(struct_stmt) => {
                debug!("Resolver::visit Stmt::Struct\n");
                self.declare(&struct_stmt.name)?;
                self.begin_scope();
                // TODO: Error if no scope
                if let Some(ref mut last_scope) = self.scopes.last_mut() {
                    last_scope.insert("self".to_string(), true);
                }
                self.end_scope();
                self.define(&struct_stmt.name);
                Ok(())
            }
            Stmt::Print(print_stmt) => {
                self.resolve_expr(&print_stmt.expression)?;
                Ok(())
            }
            Stmt::Return(return_stmt) => {
                if self.current_function_type == FunctionType::None {
                    return Err(Lang::error(
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
            Stmt::While(while_stmt) => {
                self.resolve_expr(&while_stmt.condition)?;
                self.resolve_statement(&while_stmt.body)?;
                Ok(())
            }
            Stmt::Function(function_stmt) => {
                self.declare(&function_stmt.name)?;
                self.define(&function_stmt.name);
                self.resolve_function(&function_stmt, FunctionType::Function)?;
                Ok(())
            }
            Stmt::Var(var_stmt) => {
                self.declare(&var_stmt.name)?;
                if let Some(ref initializer) = var_stmt.initializer {
                    self.resolve_expr(initializer)?;
                }
                self.define(&var_stmt.name);
                Ok(())
            }
        }
    }
}
