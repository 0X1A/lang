extern crate log;

use accept::*;
use ast::expr::*;
use ast::stmt::*;
use env::*;
use error::*;
use std::collections::HashMap;
use std::convert::TryInto;
use token::*;
use value::*;
use value_traits::callable::*;
use value_traits::r#struct::*;
use visitor::*;

#[derive(Debug)]
pub struct Interpreter {
    pub env_id: EnvironmentId,
    pub locals: HashMap<Expr, usize>,
    pub env_entries: Environment,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            locals: HashMap::new(),
            env_id: EnvironmentId { index: 0 },
            env_entries: Environment::default(),
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
        }
    }

    /// Inserts `expr` into the local scope
    pub fn resolve(&mut self, expr: &Expr, idx: usize) {
        self.locals.insert(expr.clone(), idx);
        debug!(
            "Interpreter::resolve\nInserting expr '{:?}' at index '{}' into locals '{:?}' and env '{:?}'",
            expr, idx, self.locals, self.env_entries
        );
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        Ok(expr.accept(self)?)
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

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let assign: &AssignExpr = expr.try_into()?;
        let value = self.evaluate(&assign.expr)?;
        self.env_entries
            .assign(&self.env_id, &assign.name, &value)?;
        Ok(value)
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let call: &CallExpr = expr.try_into()?;
        let callee = self.evaluate(&call.callee)?;
        let mut args = Vec::new();
        for arg in &call.arguments {
            args.push(self.evaluate(&arg)?);
        }
        match callee.value {
            Value::Callable(callable) => Ok(callable.call(self, args)?),
            Value::Struct(struct_value) => Ok(struct_value.call(self, args)?),
            _ => Err(LangError::new_runtime_error(RuntimeErrorType::CallError {
                reason: "Can only call functions and structs".to_string(),
            })),
        }
    }

    fn visit_get_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let get_expr: &GetExpr = expr.try_into()?;
        let value = self.evaluate(&get_expr.object)?;
        let struct_value: &StructInstanceTrait = (&value.value).try_into()?;
        Ok(struct_value.get_field(get_expr.name.lexeme.clone())?)
    }

    fn execute_binary_op(
        &mut self,
        op: &Token,
        left: TypedValue,
        right: TypedValue,
    ) -> Result<TypedValue, LangError> {
        match op.token_type {
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
            _ => Err(LangError::new_iie_error(
                "attempted to execute a binary operation with an incorrect token".to_string(),
            )),
        }
    }

    fn visit_impl_trait_stmt(&mut self, stmt: &Stmt) -> Result<TypedValue, LangError> {
        let impl_trait_stmt: &ImplTraitStmt = stmt.try_into()?;
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
                let mut struct_dec = self
                    .env_entries
                    .get(&self.env_id, &impl_trait_stmt.impl_name)?;
                if let Value::Struct(ref mut struct_value) = struct_dec.value {
                    struct_value.define_method(
                        &function_statement.name,
                        &TypedValue::new(function, TypeAnnotation::Fn),
                    )?;
                    self.env_entries.direct_assign(
                        &self.env_id,
                        impl_trait_stmt.impl_name.lexeme.clone(),
                        TypedValue::new(
                            Value::Struct(struct_value.clone()),
                            TypeAnnotation::User(struct_value.struct_trait().get_name()),
                        ),
                    )?;
                }
            }
        }
        Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
    }

    fn visit_trait_stmt(&mut self, stmt: &Stmt) -> Result<TypedValue, LangError> {
        let trait_stmt: &TraitStmt = stmt.try_into()?;
        self.env_entries.define(
            &self.env_id,
            &trait_stmt.name.lexeme,
            &TypedValue::new(Value::Unit, TypeAnnotation::Unit),
        );
        let mut trait_value = TraitValue {
            trait_stmt: trait_stmt.clone(),
            fn_declarations: HashMap::new(),
        };
        for fn_decl in trait_stmt.trait_fn_declarations.iter() {
            let trait_fn = self.execute(&fn_decl)?;
            if let Stmt::TraitFunction(trait_fn_decl) = fn_decl {
                trait_value
                    .fn_declarations
                    .insert(trait_fn_decl.name.lexeme.clone(), trait_fn);
            }
        }
        self.env_entries.direct_assign(
            &self.env_id,
            trait_stmt.name.lexeme.clone(),
            TypedValue::new(
                Value::Trait(Box::new(trait_value.clone())),
                TypeAnnotation::Trait,
            ),
        )?;
        Ok(TypedValue::new(
            Value::Trait(Box::new(trait_value)),
            TypeAnnotation::Trait,
        ))
    }

    fn visit_impl_stmt(&mut self, stmt: &Stmt) -> Result<TypedValue, LangError> {
        let impl_stmt: &ImplStmt = stmt.try_into()?;
        for fn_decl in &impl_stmt.fn_declarations {
            if let Stmt::Function(function_statement) = fn_decl {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_statement.clone(),
                    &self.env_id,
                )));
                let mut struct_dec = self.env_entries.get(&self.env_id, &impl_stmt.name)?;
                if let Value::Struct(ref mut struct_value) = struct_dec.value {
                    struct_value.define_method(
                        &function_statement.name,
                        &TypedValue::new(function, TypeAnnotation::Fn),
                    )?;
                    self.env_entries.direct_assign(
                        &self.env_id,
                        impl_stmt.clone().name.lexeme,
                        TypedValue::new(
                            Value::Struct(struct_value.clone()),
                            TypeAnnotation::User(struct_value.struct_trait().get_name()),
                        ),
                    )?;
                }
            }
        }
        Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
    }

    fn visit_struct_stmt(&mut self, stmt: &Stmt) -> Result<TypedValue, LangError> {
        let struct_stmt: &StructStmt = stmt.try_into()?;
        self.env_entries.define(
            &self.env_id,
            &struct_stmt.name.lexeme,
            &TypedValue::new(Value::Unit, TypeAnnotation::Unit),
        );
        let mut fields = HashMap::new();
        for field in struct_stmt.fields.iter() {
            fields.insert(
                field.identifier.lexeme.clone(),
                TypedValue::new(
                    Value::default_value(&field.type_annotation),
                    field.type_annotation.clone(),
                ),
            );
        }
        let struct_value = Value::Struct(Box::new(StructValue::new(struct_stmt.clone(), fields)));
        self.env_entries.assign(
            &self.env_id,
            &struct_stmt.name,
            &TypedValue::new(
                struct_value.clone(),
                TypeAnnotation::User(struct_stmt.name.lexeme.clone()),
            ),
        )?;
        Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
    }

    // Visit Expr stuff

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        match expr {
            Expr::Binary(b) => {
                let left = self.evaluate(&b.left)?;
                let right = self.evaluate(&b.right)?;
                Ok(self.execute_binary_op(&b.operator, left, right)?)
            }
            _ => Err(LangError::new_iie_error(
                "expected a binary expression".to_string(),
            )),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let unary_expr: &UnaryExpr = expr.try_into()?;
        let right = self.evaluate(&unary_expr.right)?;
        match unary_expr.operator.token_type {
            TokenType::Minus => match right.value {
                Value::Int32(i) => Ok(TypedValue::new(Value::Int32(-i), TypeAnnotation::I32)),
                Value::Int64(i) => Ok(TypedValue::new(Value::Int64(-i), TypeAnnotation::I64)),
                Value::Float64(f) => Ok(TypedValue::new(
                    Value::Float64(Float64::from(-f.inner)),
                    TypeAnnotation::F64,
                )),
                _ => Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit)),
            },
            TokenType::Bang => Ok(TypedValue::new(
                Value::Boolean(!self.is_truthy(&right.value)),
                TypeAnnotation::Bool,
            )),
            _ => Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit)),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let logical_expr: &LogicalExpr = expr.try_into()?;
        let left = self.evaluate(&logical_expr.left)?;
        if logical_expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left.value) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left.value) {
            return Ok(left);
        }
        Ok(self.evaluate(&logical_expr.right)?)
    }

    fn visit_set_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let set_expr: &SetExpr = expr.try_into()?;
        let mut object = self.evaluate(&set_expr.object)?;
        // check value
        let value = self.evaluate(&set_expr.value)?;
        match object.value {
            Value::Struct(ref mut struct_value) => {
                if !struct_value.field_exists(&set_expr.name.lexeme) {
                    return Err(LangError::new_runtime_error(
                        RuntimeErrorType::UndefinedVariable {
                            reason: "Tried to set an undefined struct field".to_string(),
                        },
                    ));
                }
                let mutate_struct = |struct_value: &mut TypedValue| -> Result<(), LangError> {
                    match struct_value.value {
                        Value::Struct(ref mut struct_value) => {
                            struct_value.set_field(&set_expr.name, &value)?;
                        }
                        _ => {
                            return Err(LangError::new_runtime_error(
                                RuntimeErrorType::UndefinedVariable {
                                    reason: "Tried to set an undefined struct field".to_string(),
                                },
                            ));
                        }
                    };
                    Ok(())
                };
                if let Expr::Variable(var) = set_expr.object.clone() {
                    self.env_entries
                        .mutable_action(&self.env_id, &var.name, mutate_struct)?;
                }
            }
            _ => {
                return Err(LangError::new_runtime_error(
                    RuntimeErrorType::UndefinedVariable {
                        reason: "Tried to do a set on an invalid value type".to_string(),
                    },
                ));
            }
        }
        Ok(value)
    }

    fn visit_set_array_element_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let set_array_element_expr: &SetArrayElementExpr = expr.try_into()?;
        let value = self.evaluate(&set_array_element_expr.value)?;
        let index = self
            .evaluate(&set_array_element_expr.index)?
            .as_array_index()?;
        self.env_entries.assign_index_entry(
            &self.env_id,
            &set_array_element_expr.name,
            &value,
            index,
        )?;
        Ok(value)
    }

    // TODO: Array elements must match the type annotation for the array!
    fn visit_array_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let array_expr: &ArrayExpr = expr.try_into()?;
        let mut elements = Vec::new();
        let mut type_annotation = TypeAnnotation::Unit;
        for item in array_expr.elements.iter() {
            elements.push(self.evaluate(&item)?);
        }
        if let Some(type_annotation_set) = elements.last() {
            type_annotation = type_annotation_set.clone().value_type;
        }
        Ok(TypedValue::new(
            Value::Array(elements),
            TypeAnnotation::Array(Box::new(type_annotation)),
        ))
    }

    fn visit_index_expr(&mut self, expr: &Expr) -> Result<TypedValue, LangError> {
        let index_expr: &IndexExpr = expr.try_into()?;
        let index = self.evaluate(&index_expr.index)?.as_array_index()?;
        let value = self.env_entries.get(&self.env_id, &index_expr.from)?;
        match value.value {
            Value::Array(arr) => {
                if index < arr.len() {
                    Ok(arr[index].clone())
                } else {
                    Err(LangError::new_runtime_error(
                                RuntimeErrorType::GenericError {
                                    reason: format!("Index out of bounds. Tried to index at {} for an array of length {}", index, arr.len()),
                                },
                            ))
                }
            }
            _ => Err(LangError::new_runtime_error(
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
        Ok(())
    }

    #[inline(always)]
    fn execute(&mut self, stmt: &Stmt) -> Result<TypedValue, LangError> {
        Ok(stmt.accept(self)?)
    }

    fn look_up_variable(&mut self, name: &Token, expr: &Expr) -> Result<TypedValue, LangError> {
        debug!(
            "Interpreter::look_up_variable:\nLooking for token '{:?}' within env '{:?}' and locals\n'{}'",
            name, self.env_entries, self.pretty_print_locals()
        );
        if let Some(distance) = self.locals.get(expr) {
            if let Ok(value) = self
                .env_entries
                .get(&EnvironmentId { index: *distance }, &name)
            {
                return Ok(value);
            } else {
                return Ok(self.env_entries.get(
                    &EnvironmentId {
                        index: *distance + 1,
                    },
                    &name,
                )?);
            }
        } else {
            return Ok(self.env_entries.get(&self.env_id, &name)?);
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: Vec<Stmt>,
        env_id: EnvironmentId,
    ) -> Result<TypedValue, LangError> {
        let previous = self.env_id.clone();
        self.env_id = env_id;
        let mut value = TypedValue::new(Value::Unit, TypeAnnotation::Unit);
        for stmt in stmts {
            match stmt {
                Stmt::Return(_) => {
                    // Set value and break early on a return
                    value = self.execute(&stmt)?;
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
        callable: &CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        if let Some(return_type) = callable.get_return_type() {
            if return_type
                != TypeAnnotation::from_token_type(&trait_function.function.return_type.token_type)?
            {
                return Err(LangError::new_runtime_error(
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
        callable: &CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        if callable.arity() != trait_function.function.params.len() {
            return Err(LangError::new_runtime_error(
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
        callable: &CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        for params in trait_function
            .function
            .params
            .iter()
            .zip(callable.get_params().iter())
        {
            if params.0.type_annotation != params.1.type_annotation {
                return Err(LangError::new_runtime_error(
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
        impl_trait: &Token,
        fn_value: &Value,
        trait_token: &Token,
    ) -> Result<bool, LangError> {
        let typed_trait_value = self.env_entries.get(&self.env_id, trait_token)?;
        let trait_value_type: &TraitValue = (&typed_trait_value.value).try_into()?;
        if let Some(trait_fn_decl) = trait_value_type.fn_declarations.get(&impl_trait.lexeme) {
            if let Value::TraitFunction(ref trait_function) = trait_fn_decl.value {
                let callable_value: &CallableTrait = fn_value.try_into()?;
                self.check_impl_trait_return_type(callable_value, trait_function)?;
                self.check_impl_trait_arity(callable_value, trait_function)?;
                self.check_impl_trait_param_types(callable_value, trait_function)?;
            }
        }
        Ok(false)
    }
}

impl Visitor<Stmt> for Interpreter {
    type Value = Result<TypedValue, LangError>;

    fn visit(&mut self, stmt: &Stmt) -> Self::Value {
        match stmt {
            Stmt::Enum(_) => unimplemented!(),
            Stmt::Break => Err(LangError::ControlFlow {
                subtype: ControlFlow::Break,
            }),
            Stmt::ImplTrait(_) => Ok(self.visit_impl_trait_stmt(stmt)?),
            Stmt::Trait(_) => Ok(self.visit_trait_stmt(stmt)?),
            Stmt::TraitFunction(ref trait_fn_stmt) => {
                let trait_fn = Value::TraitFunction(Box::new(TraitFunctionValue {
                    function: (**trait_fn_stmt).clone(),
                }));
                Ok(TypedValue::new(trait_fn, TypeAnnotation::Fn))
            }
            Stmt::Impl(_) => Ok(self.visit_impl_stmt(stmt)?),
            Stmt::Block(block_stmt) => {
                let env = self.env_entries.entry_from(&self.env_id);
                self.execute_block(block_stmt.statements.clone(), env)?;
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::Struct(_) => Ok(self.visit_struct_stmt(stmt)?),
            Stmt::Expression(expression_stmt) => {
                self.evaluate(&expression_stmt.expression)?;
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::Return(return_stmt) => {
                let mut value = Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
                    Value::Unit,
                    TypeAnnotation::Unit,
                ))));
                let return_value = if return_stmt.value != value {
                    self.evaluate(&return_stmt.value)?
                } else {
                    TypedValue::new(Value::Unit, TypeAnnotation::Unit)
                };
                Ok(return_value)
            }
            Stmt::Function(function_stmt) => {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_stmt.clone(),
                    &self.env_id,
                )));
                self.env_entries.define(
                    &self.env_id,
                    &function_stmt.name.lexeme,
                    &TypedValue::new(function.clone(), TypeAnnotation::Fn),
                );
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::If(if_stmt) => {
                let eval = self.evaluate(&if_stmt.condition)?;
                if self.is_truthy(&eval.value) {
                    self.execute(&if_stmt.then_branch)?;
                }
                if let Some(ref else_branch) = if_stmt.else_branch {
                    self.execute(&else_branch)?;
                }
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::Print(print_stmt) => {
                let value = self.evaluate(&print_stmt.expression)?;
                println!("{}", value.value);
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::While(while_stmt) => {
                let mut while_condition = self.evaluate(&while_stmt.condition)?;
                while self.is_truthy(&while_condition.value) {
                    if let Err(error) = self.execute(&while_stmt.body) {
                        match error {
                            LangError::ControlFlow { .. } => {
                                break;
                            }
                            other => {
                                return Err(other);
                            }
                        }
                    }
                    while_condition = self.evaluate(&while_stmt.condition)?;
                }
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Stmt::Var(var_stmt) => {
                let mut value = TypedValue::new(Value::Unit, TypeAnnotation::Unit);
                if let Some(ref initializer) = var_stmt.initializer {
                    value = self.evaluate(&initializer)?;
                }
                let var_type_annotation =
                    var_stmt.type_annotation.token_type.to_type_annotation()?;
                if var_type_annotation != value.value_type {
                    return Err(LangError::new_runtime_error(RuntimeErrorType::InvalidTypeAssignmentError {
                        reason: format!("Tried to assign a variable of type {} with an initializer of type {}", var_type_annotation.to_string(), value.value_type.to_string())
                    }));
                }
                self.env_entries
                    .define(&self.env_id, &var_stmt.name.lexeme, &value);
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
        }
    }
}

impl Visitor<Expr> for Interpreter {
    type Value = Result<TypedValue, LangError>;

    fn visit(&mut self, expr: &Expr) -> Self::Value {
        match expr {
            Expr::Assign(_) => Ok(self.visit_assign_expr(expr)?),
            Expr::Call(_) => Ok(self.visit_call_expr(expr)?),
            Expr::Get(_) => Ok(self.visit_get_expr(expr)?),
            Expr::Binary(_) => Ok(self.visit_binary_expr(expr)?),
            Expr::Unary(_) => Ok(self.visit_unary_expr(expr)?),
            Expr::Logical(_) => Ok(self.visit_logical_expr(expr)?),
            Expr::Set(_) => Ok(self.visit_set_expr(expr)?),
            Expr::SetArrayElement(_) => Ok(self.visit_set_array_element_expr(expr)?),
            Expr::Array(_) => Ok(self.visit_array_expr(expr)?),
            Expr::Index(_) => Ok(self.visit_index_expr(expr)?),
            Expr::Literal(literal_expr) => match literal_expr.value.value_type {
                TypeAnnotation::User(ref user_type) => Ok(self
                    .env_entries
                    .direct_get(&self.env_id, user_type.clone())?),
                _ => Ok(literal_expr.value.clone()),
            },
            Expr::Variable(var_expr) => Ok(self.look_up_variable(&var_expr.name, &expr)?),
            Expr::EnumPath(enum_path_expr) => {
                Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit))
            }
            Expr::Grouping(_) => Ok(TypedValue::new(Value::Unit, TypeAnnotation::Unit)),
        }
    }
}
