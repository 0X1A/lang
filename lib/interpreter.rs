extern crate log;

use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::env::*;
use crate::error::*;
use crate::token::*;
use crate::type_checker::*;
use crate::value::*;
use crate::value_traits::callable::*;
use crate::visitor::*;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Interpreter {
    pub env_id: EnvironmentId,
    pub locals: HashMap<String, usize>,
    pub env_entries: Environment,
    pub stack: Vec<TypedValue>,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            locals: HashMap::new(),
            env_id: EnvironmentId { index: 0 },
            env_entries: Environment::default(),
            stack: Vec::new(),
        }
    }
}

impl Interpreter {
    fn pretty_print_locals(&self) -> String {
        self.locals
            .iter()
            .map(|ref kvp| format!("{:?} => {}", kvp.0, kvp.1))
            .collect::<Vec<String>>()
            .join(",\n")
    }

    pub fn new() -> Interpreter {
        let mut env_entries = Environment::default();
        let env_id = env_entries.new_entry();
        Interpreter {
            locals: HashMap::new(),
            env_id,
            env_entries,
            stack: Vec::new(),
        }
    }

    pub fn resolve(&mut self, token: &str, idx: usize) {
        self.locals.insert(token.into(), idx);
        debug!(
            "{}:{} Inserting expr '{:?}' at index '{}' into locals '{:?}' and env '{:?}'",
            file!(),
            line!(),
            token,
            idx,
            self.locals,
            self.env_entries
        );
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<(), LangError> {
        self.visit_expr(expr)?;
        Ok(())
    }

    fn pop(&mut self) -> Result<TypedValue, LangError> {
        self.stack.pop().ok_or_else(|| {
            LangErrorType::new_iie_error("the interpreter's stack pop failed!".to_string())
        })
    }

    fn is_truthy(&self, val: &Value) -> bool {
        if *val == Value::Unit {
            false
        } else {
            match *val {
                Value::Boolean(b) => b,
                _ => true,
            }
        }
    }

    fn visit_assign_expr(&mut self, assign: &AssignExpr) -> Result<(), LangError> {
        self.evaluate(&assign.expr)?;
        let value = self.pop()?;
        self.env_entries
            .assign(&self.env_id, &assign.name, value.clone())?;
        self.stack.push(value);
        Ok(())
    }

    fn visit_call_expr(&mut self, call: &CallExpr) -> Result<(), LangError> {
        self.evaluate(&call.callee)?;
        let callee = self.pop()?;
        let mut args = Vec::new();
        for arg in &call.arguments {
            self.evaluate(&arg)?;
            args.push(self.pop()?);
        }
        match callee.value {
            Value::Callable(callable) => {
                let value = callable.call(self, args)?;
                self.stack.push(value);
                Ok(())
            }
            Value::Struct(struct_value) => {
                let value = struct_value.call(self, args)?;
                self.stack.push(value);
                Ok(())
            }
            _ => Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::CallError {
                    reason: "Can only call functions and structs".to_string(),
                },
            )),
        }
    }

    fn visit_get_expr(&mut self, get_expr: &GetExpr) -> Result<(), LangError> {
        self.evaluate(&get_expr.object)?;
        let value = self.pop()?;
        match value.value {
            Value::Struct(_) => {
                let struct_value: &dyn StructInstanceTrait = (&value.value).try_into()?;
                let field = struct_value.get_field(&get_expr.name, self)?;
                self.stack.push(field);
            }
            Value::SelfIndex(s) => {
                let nvalue = self.env_entries.get(&s.env_id, &s.name)?;
                let struct_value: &dyn StructInstanceTrait = (&nvalue.value).try_into()?;
                let field = struct_value.get_field(&get_expr.name, self)?;
                self.stack.push(field);
            }
            _ => {}
        }
        Ok(())
    }

    fn execute_binary_op(
        &mut self,
        op: &TokenType,
        left: TypedValue,
        right: TypedValue,
    ) -> Result<TypedValue, LangError> {
        match op {
            TokenType::Plus => Ok(TypedValue::new(left.value + right.value, left.value_type)),
            TokenType::Minus => Ok(TypedValue::new(left.value - right.value, left.value_type)),
            TokenType::Star => Ok(TypedValue::new(left.value * right.value, left.value_type)),
            TokenType::Slash => Ok(TypedValue::new(left.value / right.value, left.value_type)),
            TokenType::Greater => Ok(TypedValue::new(
                Value::Boolean(left.value > right.value),
                TypeAnnotation::Bool,
            )),
            TokenType::GreaterEqual => Ok(TypedValue::new(
                Value::Boolean(left.value >= right.value),
                TypeAnnotation::Bool,
            )),
            TokenType::Less => Ok(TypedValue::new(
                Value::Boolean(left.value < right.value),
                TypeAnnotation::Bool,
            )),
            TokenType::LessEqual => Ok(TypedValue::new(
                Value::Boolean(left.value <= right.value),
                TypeAnnotation::Bool,
            )),
            TokenType::BangEqual => Ok(TypedValue::new(
                Value::Boolean(left.value != right.value),
                TypeAnnotation::Bool,
            )),
            TokenType::EqualEqual => Ok(TypedValue::new(
                Value::Boolean(left.value == right.value),
                TypeAnnotation::Bool,
            )),
            _ => Err(LangErrorType::new_iie_error(
                "attempted to execute a binary operation with an incorrect token".to_string(),
            )),
        }
    }

    fn visit_impl_trait_stmt(&mut self, impl_trait_stmt: &ImplTraitStmt) -> Result<(), LangError> {
        for fn_impl in impl_trait_stmt.fn_declarations.iter() {
            if let Stmt::Function(function_statement) = fn_impl {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_statement.clone(),
                    &self.env_id,
                )));
                self.check_impl_trait(
                    &function_statement.name,
                    &function,
                    &impl_trait_stmt.trait_name,
                )?;
                let update_struct_decl = |struct_value: &mut TypedValue| -> Result<(), LangError> {
                    let struct_value: &mut dyn StructInstanceTrait =
                        (&mut struct_value.value).try_into()?;
                    struct_value.define_method(
                        &function_statement.name,
                        TypedValue::new(function.clone(), TypeAnnotation::Fn),
                    )?;
                    Ok(())
                };
                self.env_entries.update_value(
                    &self.env_id,
                    &impl_trait_stmt.impl_name,
                    update_struct_decl,
                )?;
            }
        }
        Ok(())
    }

    fn visit_trait_stmt(&mut self, trait_stmt: &TraitStmt) -> Result<(), LangError> {
        self.env_entries.define(
            &self.env_id,
            &trait_stmt.name,
            TypedValue::new(Value::Unit, TypeAnnotation::Unit),
        );
        let mut trait_value = TraitValue {
            trait_stmt: trait_stmt.clone(),
            fn_declarations: HashMap::new(),
        };
        for fn_decl in trait_stmt.trait_fn_declarations.iter() {
            self.execute(&fn_decl)?;
            let trait_fn = self.pop()?;
            if let Stmt::TraitFunction(trait_fn_decl) = fn_decl {
                trait_value
                    .fn_declarations
                    .insert(trait_fn_decl.name.clone(), trait_fn);
            }
        }
        self.env_entries.assign(
            &self.env_id,
            &trait_stmt.name,
            TypedValue::new(
                Value::Trait(Box::new(trait_value.clone())),
                TypeAnnotation::Trait,
            ),
        )?;
        self.stack.push(TypedValue::new(
            Value::Trait(Box::new(trait_value)),
            TypeAnnotation::Trait,
        ));
        Ok(())
    }

    fn visit_impl_stmt(&mut self, impl_stmt: &ImplStmt) -> Result<(), LangError> {
        for fn_decl in &impl_stmt.fn_declarations {
            if let Stmt::Function(function_statement) = fn_decl {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_statement.clone(),
                    &self.env_id,
                )));
                let update_struct = |struct_value: &mut TypedValue| -> Result<(), LangError> {
                    let struct_value: &mut dyn StructInstanceTrait =
                        (&mut struct_value.value).try_into()?;
                    struct_value.define_method(
                        &function_statement.name,
                        TypedValue::new(function, TypeAnnotation::Fn),
                    )?;
                    Ok(())
                };
                self.env_entries
                    .update_value(&self.env_id, &impl_stmt.name, update_struct)?;
            }
        }
        Ok(())
    }

    fn visit_struct_stmt(&mut self, struct_stmt: &StructStmt) -> Result<(), LangError> {
        self.env_entries.define(
            &self.env_id,
            &struct_stmt.name,
            TypedValue::new(Value::Unit, TypeAnnotation::Unit),
        );
        let mut fields = HashMap::new();
        for field in struct_stmt.fields.iter() {
            fields.insert(
                field.identifier.clone(),
                TypedValue::new(
                    Value::default_value(&field.type_annotation),
                    field.type_annotation.clone(),
                ),
            );
        }
        let struct_value = Value::Struct(Box::new(StructValue::new(
            struct_stmt.clone(),
            fields,
            struct_stmt.name.clone(),
        )));
        self.env_entries.assign(
            &self.env_id,
            &struct_stmt.name,
            TypedValue::new(
                struct_value.clone(),
                TypeAnnotation::User(struct_stmt.name.clone()),
            ),
        )?;
        Ok(())
    }

    // Visit Expr stuff

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<(), LangError> {
        self.evaluate(&expr.left)?;
        let left = self.pop()?;
        self.evaluate(&expr.right)?;
        let right = self.pop()?;
        let value = self.execute_binary_op(&expr.operator, left, right)?;
        self.stack.push(value);
        Ok(())
    }

    fn visit_unary_expr(&mut self, unary_expr: &UnaryExpr) -> Result<(), LangError> {
        self.evaluate(&unary_expr.right)?;
        let right = self.pop()?;
        match unary_expr.operator {
            TokenType::Minus => match right.value {
                Value::Int32(i) => {
                    self.stack
                        .push(TypedValue::new(Value::Int32(-i), TypeAnnotation::I32));
                    Ok(())
                }
                Value::Int64(i) => {
                    self.stack
                        .push(TypedValue::new(Value::Int64(-i), TypeAnnotation::I64));
                    Ok(())
                }
                Value::Float64(f) => {
                    self.stack.push(TypedValue::new(
                        Value::Float64(Float64::from(-f.inner)),
                        TypeAnnotation::F64,
                    ));
                    Ok(())
                }
                _ => {
                    self.stack
                        .push(TypedValue::new(Value::Unit, TypeAnnotation::Unit));
                    Ok(())
                }
            },
            TokenType::Bang => {
                let value = !self.is_truthy(&right.value);
                self.stack
                    .push(TypedValue::new(Value::Boolean(value), TypeAnnotation::Bool));
                Ok(())
            }
            _ => {
                self.stack
                    .push(TypedValue::new(Value::Unit, TypeAnnotation::Unit));
                Ok(())
            }
        }
    }

    fn visit_logical_expr(&mut self, logical_expr: &LogicalExpr) -> Result<(), LangError> {
        self.evaluate(&logical_expr.left)?;
        let left = self.pop()?;
        if logical_expr.operator == TokenType::Or {
            if self.is_truthy(&left.value) {
                self.stack.push(left);
                return Ok(());
            }
        } else if !self.is_truthy(&left.value) {
            self.stack.push(left);
            return Ok(());
        }
        self.evaluate(&logical_expr.right)?;
        Ok(())
    }

    fn visit_set_expr(&mut self, set_expr: &SetExpr) -> Result<(), LangError> {
        self.evaluate(&set_expr.object)?;
        let mut object = self.pop()?;
        // check value
        self.evaluate(&set_expr.value)?;
        let value = self.pop()?;
        match object.value {
            Value::Struct(ref mut struct_value) => {
                if !struct_value.field_exists(&set_expr.name) {
                    return Err(LangErrorType::new_runtime_error(
                        RuntimeErrorType::UndefinedVariable {
                            reason: "Tried to set an undefined struct field".to_string(),
                        },
                    ));
                }
                let update_struct = |struct_value: &mut TypedValue| -> Result<(), LangError> {
                    let struct_value: &mut dyn StructInstanceTrait =
                        (&mut struct_value.value).try_into()?;
                    struct_value.set_field(&set_expr.name, &value)?;
                    Ok(())
                };
                if let Expr::Variable(var) = set_expr.object.clone() {
                    self.env_entries
                        .update_value(&self.env_id, &var.name, update_struct)?;
                }
            }
            Value::SelfIndex(s) => {
                let nvalue = self.env_entries.get(&s.env_id, &s.name)?;
                let struct_value: &dyn StructInstanceTrait = (&nvalue.value).try_into()?;
                if !struct_value.field_exists(&set_expr.name) {
                    return Err(LangErrorType::new_runtime_error(
                        RuntimeErrorType::UndefinedVariable {
                            reason: "Tried to set an undefined struct field".to_string(),
                        },
                    ));
                }
                let update_struct = |typed_value: &mut TypedValue| -> Result<(), LangError> {
                    let struct_value: &mut dyn StructInstanceTrait =
                        (&mut typed_value.value).try_into()?;
                    struct_value.set_field(&set_expr.name, &value)?;
                    Ok(())
                };
                if let Expr::SelfIdent(_) = set_expr.object.clone() {
                    self.env_entries.update_value(
                        &s.env_id,
                        &struct_value.get_instance_name(),
                        update_struct,
                    )?;
                }
            }
            _ => {
                return Err(LangErrorType::new_runtime_error(
                    RuntimeErrorType::UndefinedVariable {
                        reason: "Tried to do a set on an invalid value type".to_string(),
                    },
                ));
            }
        }
        Ok(())
    }

    fn visit_set_array_element_expr(
        &mut self,
        set_array_element_expr: &SetArrayElementExpr,
    ) -> Result<(), LangError> {
        self.evaluate(&set_array_element_expr.value)?;
        let value = self.pop()?;
        self.evaluate(&set_array_element_expr.index)?;
        let index = self.pop()?.as_array_index()?;
        self.env_entries.assign_index_entry(
            &self.env_id,
            &set_array_element_expr.name,
            &value,
            index,
        )?;
        Ok(())
    }

    fn visit_array_expr(&mut self, array_expr: &ArrayExpr) -> Result<(), LangError> {
        let mut elements = Vec::new();
        let mut type_annotation = TypeAnnotation::Unit;
        let mut array_element_type = TypeAnnotation::Unit;
        // Invariant used in order evaluate the initial array element only once
        let mut first_element = true;
        for item in array_expr.elements.iter() {
            self.evaluate(&item)?;
            let element = self.pop()?;
            if first_element {
                array_element_type = element.value_type.clone();
                first_element = false;
            } else {
                TypeChecker::check_type(&array_element_type, &element.value_type)?;
            }
            elements.push(element);
        }
        if let Some(ref type_annotation_set) = array_expr.type_annotation {
            type_annotation = type_annotation_set.to_type_annotation()?;
        }
        if type_annotation == TypeAnnotation::Unit {
            type_annotation = TypeAnnotation::Array(Box::new(array_element_type.clone()));
        }
        self.stack
            .push(TypedValue::new(Value::Array(elements), type_annotation));
        Ok(())
    }

    fn visit_index_expr(&mut self, index_expr: &IndexExpr) -> Result<(), LangError> {
        self.evaluate(&index_expr.index)?;
        let index = self.pop()?.as_array_index()?;
        let value = self.env_entries.get(&self.env_id, &index_expr.from)?;
        match value.value {
            Value::Array(arr) => {
                if index < arr.len() {
                    self.stack.push(arr[index].clone());
                    Ok(())
                } else {
                    Err(LangErrorType::new_runtime_error(
                                RuntimeErrorType::GenericError {
                                    reason: format!("Index out of bounds. Tried to index at {} for an array of length {}", index, arr.len()),
                                },
                            ))
                }
            }
            _ => Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::GenericError {
                    reason: "Tried to index a non-array value. This should never happen"
                        .to_string(),
                },
            )),
        }
    }

    #[inline(always)]
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), LangError> {
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        self.env_entries.entries = Vec::with_capacity(0);
        self.stack = Vec::with_capacity(0);
        Ok(())
    }

    #[inline(always)]
    fn execute(&mut self, stmt: &Stmt) -> Result<(), LangError> {
        self.visit_stmt(stmt)?;
        Ok(())
    }

    fn look_up_variable(&mut self, token: &str) -> Result<(), LangError> {
        debug!(
            "{}:{} Looking for token '{:?}' within env '{:?}' and locals\n'{}'",
            file!(),
            line!(),
            token,
            self.env_entries,
            self.pretty_print_locals()
        );
        if let Some(distance) = self.locals.get(token) {
            if let Ok(value) = self
                .env_entries
                .get(&EnvironmentId { index: *distance }, &token)
            {
                match value.value {
                    Value::SelfIndex(s) => {
                        let str_val = self.env_entries.get(&s.env_id, &s.name)?;
                        self.stack.push(str_val);
                    }
                    _ => {
                        self.stack.push(value);
                    }
                }
                Ok(())
            } else {
                let value = self.env_entries.get(
                    &EnvironmentId {
                        index: *distance + 1,
                    },
                    &token,
                )?;
                self.stack.push(value);
                Ok(())
            }
        } else {
            let value = self.env_entries.get(&self.env_id, &token)?;
            self.stack.push(value);
            Ok(())
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        env_id: EnvironmentId,
    ) -> Result<TypedValue, LangError> {
        let previous = self.env_id.clone();
        self.env_id = env_id;
        let mut value = TypedValue::new(Value::Unit, TypeAnnotation::Unit);
        for stmt in stmts {
            match stmt {
                Stmt::Return(_) => {
                    // Set value and break early on a return
                    self.execute(&stmt)?;
                    value = self.pop()?;
                    break;
                }
                _ => {
                    self.execute(&stmt)?;
                }
            }
        }
        self.env_entries.remove_entry(&self.env_id);
        self.env_id = previous;
        Ok(value)
    }

    fn check_impl_trait_return_type(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        if let Some(return_type) = callable.get_return_type() {
            if return_type != return_type {
                return Err(LangErrorType::new_runtime_error(
                    RuntimeErrorType::InvalidTypeAssignmentError {
                        reason: format!(
                            "trait impl {} doesn't match trait return type {}",
                            return_type, trait_function.function.return_type
                        ),
                    },
                ));
            }
        }
        Ok(())
    }

    fn check_impl_trait_arity(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        if callable.arity() != trait_function.function.params.len() {
            return Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "trait impl expected {} arguments, found {}",
                        trait_function.function.params.len(),
                        callable.arity()
                    ),
                },
            ));
        }
        Ok(())
    }

    fn check_impl_trait_param_types(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        for params in trait_function
            .function
            .params
            .iter()
            .zip(callable.get_params().iter())
        {
            if params.0.type_annotation != params.1.type_annotation {
                return Err(LangErrorType::new_runtime_error(
                    RuntimeErrorType::InvalidTypeAssignmentError {
                        reason: format!(
                            "trait impl expected parameter of type {}, found type {}",
                            params.0.type_annotation, params.1.type_annotation
                        ),
                    },
                ));
            }
        }
        Ok(())
    }

    fn check_impl_trait(
        &self,
        impl_trait: &String,
        fn_value: &Value,
        trait_token: &String,
    ) -> Result<bool, LangError> {
        let typed_trait_value = self.env_entries.get_ref(&self.env_id, trait_token)?;
        let trait_value_type: &TraitValue = (&typed_trait_value.value).try_into()?;
        if let Some(trait_fn_decl) = trait_value_type.fn_declarations.get(impl_trait) {
            if let Value::TraitFunction(ref trait_function) = trait_fn_decl.value {
                let callable_value: &dyn CallableTrait = fn_value.try_into()?;
                self.check_impl_trait_return_type(callable_value, trait_function)?;
                self.check_impl_trait_arity(callable_value, trait_function)?;
                self.check_impl_trait_param_types(callable_value, trait_function)?;
            }
        }
        Ok(false)
    }
}

impl Visitor for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), LangError> {
        Ok(noop_expr(self, expr)?)
    }
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), LangError> {
        Ok(noop_stmt(self, stmt)?)
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> Result<(), LangError> {
        Ok(self.visit_assign_expr(assign)?)
    }
    fn visit_binary(&mut self, binary: &BinaryExpr) -> Result<(), LangError> {
        Ok(self.visit_binary_expr(binary)?)
    }
    fn visit_call(&mut self, call: &CallExpr) -> Result<(), LangError> {
        Ok(self.visit_call_expr(call)?)
    }
    fn visit_get(&mut self, get: &GetExpr) -> Result<(), LangError> {
        Ok(self.visit_get_expr(get)?)
    }
    fn visit_enum_path(&mut self, _: &EnumPathExpr) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_grouping(&mut self, _: &GroupingExpr) -> Result<(), LangError> {
        Ok(())
    }
    fn visit_literal(&mut self, literal: &LiteralExpr) -> Result<(), LangError> {
        match literal.value.value_type {
            TypeAnnotation::User(ref user_type) => {
                let value = self.env_entries.get(&self.env_id, &user_type)?;
                self.stack.push(value);
            }
            _ => {
                self.stack.push(literal.value.clone());
            }
        }
        Ok(())
    }
    fn visit_logical(&mut self, logical: &LogicalExpr) -> Result<(), LangError> {
        Ok(self.visit_logical_expr(logical)?)
    }
    fn visit_set(&mut self, set: &SetExpr) -> Result<(), LangError> {
        Ok(self.visit_set_expr(set)?)
    }
    fn visit_unary(&mut self, unary: &UnaryExpr) -> Result<(), LangError> {
        Ok(self.visit_unary_expr(unary)?)
    }
    fn visit_array(&mut self, array: &ArrayExpr) -> Result<(), LangError> {
        Ok(self.visit_array_expr(array)?)
    }
    fn visit_index(&mut self, index: &IndexExpr) -> Result<(), LangError> {
        Ok(self.visit_index_expr(index)?)
    }
    fn visit_set_array_element(
        &mut self,
        set_array_element: &SetArrayElementExpr,
    ) -> Result<(), LangError> {
        Ok(self.visit_set_array_element_expr(set_array_element)?)
    }
    fn visit_variable(&mut self, variable: &VariableExpr) -> Result<(), LangError> {
        Ok(self.look_up_variable(&variable.name)?)
    }
    fn visit_self_ident(&mut self, self_ident: &SelfIdentExpr) -> Result<(), LangError> {
        self.look_up_variable(&self_ident.keyword)?;
        Ok(())
    }

    // stmt
    fn visit_break(&mut self) -> Result<(), LangError> {
        Err(LangError::from(LangErrorType::ControlFlow {
            subtype: ControlFlow::Break,
        }))
    }
    fn visit_enum(&mut self, _: &EnumStmt) -> Result<(), LangError> {
        unimplemented!()
    }
    fn visit_impl(&mut self, impl_stmt: &ImplStmt) -> Result<(), LangError> {
        Ok(self.visit_impl_stmt(impl_stmt)?)
    }
    fn visit_impl_trait(&mut self, impl_trait: &ImplTraitStmt) -> Result<(), LangError> {
        Ok(self.visit_impl_trait_stmt(impl_trait)?)
    }
    fn visit_block(&mut self, block: &BlockStmt) -> Result<(), LangError> {
        let env = self.env_entries.entry_from(&self.env_id);
        self.execute_block(&block.statements, env)?;
        Ok(())
    }
    fn visit_struct(&mut self, block: &StructStmt) -> Result<(), LangError> {
        Ok(self.visit_struct_stmt(block)?)
    }
    fn visit_expression(&mut self, block: &ExpressionStmt) -> Result<(), LangError> {
        Ok(self.visit_expr(&block.expression)?)
    }
    fn visit_trait(&mut self, block: &TraitStmt) -> Result<(), LangError> {
        Ok(self.visit_trait_stmt(block)?)
    }
    fn visit_trait_function(&mut self, trait_fn_stmt: &TraitFunctionStmt) -> Result<(), LangError> {
        let trait_fn = Value::TraitFunction(Box::new(TraitFunctionValue {
            function: trait_fn_stmt.clone(),
        }));
        self.stack
            .push(TypedValue::new(trait_fn, TypeAnnotation::Fn));
        Ok(())
    }
    fn visit_function(&mut self, function_stmt: &FunctionStmt) -> Result<(), LangError> {
        let function =
            Value::Callable(Box::new(Callable::new(function_stmt.clone(), &self.env_id)));
        self.env_entries.define(
            &self.env_id,
            &function_stmt.name,
            TypedValue::new(function.clone(), TypeAnnotation::Fn),
        );
        Ok(())
    }
    fn visit_if(&mut self, if_stmt: &IfStmt) -> Result<(), LangError> {
        self.evaluate(&if_stmt.condition)?;
        let eval = self.pop()?;
        if self.is_truthy(&eval.value) {
            self.execute(&if_stmt.then_branch)?;
        }
        if let Some(ref else_branch) = if_stmt.else_branch {
            self.execute(&else_branch)?;
        }
        Ok(())
    }
    fn visit_print(&mut self, print_stmt: &PrintStmt) -> Result<(), LangError> {
        self.evaluate(&print_stmt.expression)?;
        let value = self.pop()?;
        println!("{}", value.value);
        Ok(())
    }
    fn visit_return(&mut self, return_stmt: &ReturnStmt) -> Result<(), LangError> {
        let value = Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
            Value::Unit,
            TypeAnnotation::Unit,
        ))));
        let return_value = if return_stmt.value != value {
            self.evaluate(&return_stmt.value)?;
            self.pop()?
        } else {
            TypedValue::new(Value::Unit, TypeAnnotation::Unit)
        };
        self.stack.push(return_value);
        Ok(())
    }
    fn visit_var(&mut self, var_stmt: &VarStmt) -> Result<(), LangError> {
        let mut value = TypedValue::new(Value::Unit, TypeAnnotation::Unit);
        if let Some(ref initializer) = var_stmt.initializer {
            self.evaluate(&initializer)?;
            value = self.pop()?;
        }
        let var_type_annotation = var_stmt.type_annotation.to_type_annotation()?;
        if var_type_annotation != value.value_type {
            return Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "Tried to assign a variable of type {} with an initializer of type {}",
                        var_type_annotation.to_string(),
                        value.value_type.to_string()
                    ),
                },
            ));
        }
        if let Value::Struct(ref mut struct_value) = value.value {
            struct_value.set_instance_name(var_stmt.name.clone());
        }
        self.env_entries.define(&self.env_id, &var_stmt.name, value);
        self.stack
            .push(TypedValue::new(Value::Unit, TypeAnnotation::Unit));
        Ok(())
    }
    fn visit_while(&mut self, while_stmt: &WhileStmt) -> Result<(), LangError> {
        self.evaluate(&while_stmt.condition)?;
        let mut while_condition = self.pop()?;
        while self.is_truthy(&while_condition.value) {
            if let Err(error) = self.execute(&while_stmt.body) {
                match error.context.get_context() {
                    LangErrorType::ControlFlow { .. } => {
                        break;
                    }
                    other => {
                        return Err(LangError::from((*other).clone()));
                    }
                }
            }
            self.evaluate(&while_stmt.condition)?;
            while_condition = self.pop()?;
        }
        Ok(())
    }
}
