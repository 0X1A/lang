extern crate log;

use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::env::*;
use crate::error::*;
use crate::mem::*;
use crate::token::*;
use crate::type_checker::*;
use crate::value::*;
use crate::value_traits::callable::*;
use crate::visitor::*;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct AstVisitor {}

#[derive(Debug)]
pub struct Interpreter {
    pub env_id: EnvironmentId,
    pub locals: HashMap<String, usize>,
    pub arena: Arena<TypedValue>,
    pub env_entries: Environment,
    pub ast_visitor: AstVisitor,
    pub stack: Vec<TypedValue>,
}

impl Default for Interpreter {
    fn default() -> Interpreter {
        Interpreter {
            locals: HashMap::new(),
            arena: Arena::with_capacity(256),
            env_id: EnvironmentId { index: 0 },
            env_entries: Environment::default(),
            ast_visitor: AstVisitor {},
            stack: Vec::new(),
        }
    }
}

impl Interpreter {
    fn pretty_print_locals(&self) -> String {
        let mut format = String::from("ident: idx\n");
        format.push_str(
            &self
                .locals
                .iter()
                .map(|ref kvp| format!("{:?} => {}", kvp.0, kvp.1))
                .collect::<Vec<String>>()
                .join(",\n"),
        );
        format
    }

    pub fn new() -> Interpreter {
        let mut env_entries = Environment::default();
        let env_id = env_entries.new_entry();
        Interpreter {
            locals: HashMap::new(),
            arena: Arena::with_capacity(256),
            env_id,
            env_entries,
            ast_visitor: AstVisitor {},
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

    fn evaluate(
        &self,
        expr: &Expr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(expr, arena, env)?)
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

    fn visit_assign_expr(
        &self,
        assign: &AssignExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(arena_entry_index) = self.evaluate(&assign.expr, arena, env)? {
            let arena_entry = &arena[arena_entry_index];
            let value: &TypedValue = arena_entry.try_into()?;
            let root_entry_id = env.root_entry_id.clone();
            env.assign_two(&root_entry_id, &assign.name, value.clone(), arena)?;
        }
        Ok(None)
    }

    fn visit_call_expr(
        &self,
        call: &CallExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let mut args = Vec::new();
        for arg in &call.arguments {
            if let Some(arg_entry_index) = self.evaluate(&arg, arena, env)? {
                args.push(arg_entry_index);
            }
        }
        if let Some(arena_entry_index) = self.evaluate(&call.callee, arena, env)? {
            let arena_entry = &arena[arena_entry_index];
            let callee: TypedValue = arena_entry.try_into()?;
            match &callee.value {
                Value::Callable(callable) => {
                    let value = callable.call_two(arena, env, self, args)?;
                    return Ok(Some(arena.insert(value)));
                }
                Value::Struct(struct_value) => {
                    let value = struct_value.call_two(arena, env, self, args)?;
                    return Ok(Some(arena.insert(value)));
                }
                _ => {
                    return Err(LangErrorType::new_runtime_error(
                        RuntimeErrorType::CallError {
                            reason: "Can only call functions and structs".to_string(),
                        },
                    ))
                }
            };
        }
        Ok(None)
    }

    fn visit_get_expr(
        &self,
        get_expr: &GetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(arena_entry_index) = self.evaluate(&get_expr.object, arena, env)? {
            let arena_entry = &arena[arena_entry_index];
            let value: TypedValue = arena_entry.try_into()?;
            let mut index = None;
            match &value.value {
                Value::Struct(_) => {
                    let struct_value: &dyn StructInstanceTrait = (&value.value).try_into()?;
                    let field = struct_value.get_field_two(&get_expr.name, env, arena, self)?;
                    index = Some(arena.insert(field));
                }
                Value::SelfIndex(s) => {
                    let nvalue_entry = &arena[env.get_two(&s.env_id, &s.name)?];
                    let nvalue: TypedValue = nvalue_entry.try_into()?;
                    let struct_value: &dyn StructInstanceTrait = (&nvalue.value).try_into()?;
                    let field = struct_value.get_field_two(&get_expr.name, env, arena, self)?;
                    index = Some(arena.insert(field));
                }
                _ => {}
            }
            return Ok(index);
        }
        Ok(None)
    }

    fn execute_binary_op(
        &self,
        op: &TokenType,
        left: &TypedValue,
        right: &TypedValue,
    ) -> Result<TypedValue, LangError> {
        match op {
            TokenType::Plus => Ok(TypedValue::new(
                &left.value + &right.value,
                left.value_type.clone(),
            )),
            TokenType::Minus => Ok(TypedValue::new(
                &left.value - &right.value,
                left.value_type.clone(),
            )),
            TokenType::Star => Ok(TypedValue::new(
                &left.value * &right.value,
                left.value_type.clone(),
            )),
            TokenType::Slash => Ok(TypedValue::new(
                &left.value / &right.value,
                left.value_type.clone(),
            )),
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

    fn visit_impl_trait_stmt(
        &self,
        impl_trait_stmt: &ImplTraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        for fn_impl in impl_trait_stmt.fn_declarations.iter() {
            if let Stmt::Function(function_statement) = fn_impl {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_statement.clone(),
                    &env.root_entry_id,
                )));
                self.check_impl_trait(
                    &function_statement.name,
                    &function,
                    env,
                    &arena,
                    &impl_trait_stmt.trait_name,
                )?;
                let update_struct_decl_closure =
                    |struct_value: &mut TypedValue| -> Result<(), LangError> {
                        let struct_value: &mut dyn StructInstanceTrait =
                            (&mut struct_value.value).try_into()?;
                        struct_value.define_method(
                            &function_statement.name,
                            TypedValue::new(function.clone(), TypeAnnotation::Fn),
                        )?;
                        Ok(())
                    };
                let root_id = env.root_entry_id.clone();
                env.update_value_two(
                    &root_id,
                    &impl_trait_stmt.impl_name,
                    arena,
                    update_struct_decl_closure,
                )?;
            }
        }
        Ok(None)
    }

    fn visit_trait_stmt(
        &self,
        trait_stmt: &TraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let root_id = env.root_entry_id.clone();
        env.define_two(
            &root_id,
            arena,
            &trait_stmt.name,
            TypedValue::new(Value::Unit, TypeAnnotation::Unit),
        );
        let mut trait_value = TraitValue {
            trait_stmt: trait_stmt.clone(),
            fn_declarations: HashMap::new(),
        };
        for fn_decl in trait_stmt.trait_fn_declarations.iter() {
            if let Some(fn_decl_index) = self.execute(&fn_decl, arena, env)? {
                let fn_decl_entry = &arena[fn_decl_index];
                let trait_fn: &TypedValue = fn_decl_entry.try_into()?;
                if let Stmt::TraitFunction(trait_fn_decl) = fn_decl {
                    trait_value
                        .fn_declarations
                        .insert(trait_fn_decl.name.clone(), trait_fn.clone());
                }
            }
        }
        let root_id = env.root_entry_id.clone();
        env.assign_two(
            &root_id,
            &trait_stmt.name,
            TypedValue::new(
                Value::Trait(Box::new(trait_value.clone())),
                TypeAnnotation::Trait,
            ),
            arena,
        )?;
        Ok(Some(arena.insert(TypedValue::new(
            Value::Trait(Box::new(trait_value)),
            TypeAnnotation::Trait,
        ))))
    }

    fn visit_impl_stmt(
        &self,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        for fn_decl in &impl_stmt.fn_declarations {
            if let Stmt::Function(function_statement) = fn_decl {
                let function = Value::Callable(Box::new(Callable::new(
                    *function_statement.clone(),
                    &env.root_entry_id,
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
                let root_id = env.root_entry_id.clone();
                env.update_value_two(&root_id, &impl_stmt.name, arena, update_struct)?;
            }
        }
        Ok(None)
    }

    fn visit_struct_stmt(
        &self,
        struct_stmt: &StructStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let root_id = env.root_entry_id.clone();
        env.define_two(
            &root_id,
            arena,
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
        let root_id = env.root_entry_id.clone();
        env.assign_two(
            &root_id,
            &struct_stmt.name,
            TypedValue::new(struct_value, TypeAnnotation::User(struct_stmt.name.clone())),
            arena,
        )?;
        Ok(None)
    }

    // Visit Expr stuff

    fn visit_binary_expr(
        &self,
        expr: &BinaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(left_arena_entry_index) = self.evaluate(&expr.left, arena, env)? {
            if let Some(right_arena_entry_index) = self.evaluate(&expr.right, arena, env)? {
                let left_arena_entry = &arena[left_arena_entry_index];
                let left: &TypedValue = left_arena_entry.try_into()?;
                let right_arena_entry = &arena[right_arena_entry_index];
                let right: &TypedValue = right_arena_entry.try_into()?;
                let value = self.execute_binary_op(&expr.operator, left, right)?;
                let index = arena.insert(value);
                return Ok(Some(index));
            }
        }
        Ok(None)
    }

    fn visit_unary_expr(
        &self,
        unary_expr: &UnaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(right_arena_index) = self.evaluate(&unary_expr.right, arena, env)? {
            let right_arena_entry = &arena[right_arena_index];
            let right: &TypedValue = right_arena_entry.try_into()?;
            match unary_expr.operator {
                TokenType::Minus => match right.value {
                    Value::Int32(i) => {
                        return Ok(Some(
                            arena.insert(TypedValue::new(Value::Int32(-i), TypeAnnotation::I32)),
                        ))
                    }
                    Value::Int64(i) => {
                        return Ok(Some(
                            arena.insert(TypedValue::new(Value::Int64(-i), TypeAnnotation::I64)),
                        ))
                    }
                    Value::Float64(f) => {
                        return Ok(Some(arena.insert(TypedValue::new(
                            Value::Float64(Float64::from(-f.inner)),
                            TypeAnnotation::F64,
                        ))))
                    }
                    _ => {
                        return Ok(Some(
                            arena.insert(TypedValue::new(Value::Unit, TypeAnnotation::Unit)),
                        ))
                    }
                },
                TokenType::Bang => {
                    let value = !self.is_truthy(&right.value);
                    return Ok(Some(arena.insert(TypedValue::new(
                        Value::Boolean(value),
                        TypeAnnotation::Bool,
                    ))));
                }
                _ => {
                    return Ok(Some(
                        arena.insert(TypedValue::new(Value::Unit, TypeAnnotation::Unit)),
                    ))
                }
            }
        }
        Ok(None)
    }

    fn visit_logical_expr(
        &self,
        logical_expr: &LogicalExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(arena_entry_index) = self.evaluate(&logical_expr.left, arena, env)? {
            let arena_entry = &arena[arena_entry_index];
            let left: TypedValue = arena_entry.try_into()?;
            if logical_expr.operator == TokenType::Or && self.is_truthy(&left.value) {
                let index = arena.insert(left);
                return Ok(Some(index));
            }
            if !self.is_truthy(&left.value) {
                let index = arena.insert(left);
                return Ok(Some(index));
            }
            return Ok(self.evaluate(&logical_expr.right, arena, env)?);
        }
        Ok(None)
    }

    // TODO: this shit is _still_ a mess
    fn visit_set_expr(
        &self,
        set_expr: &SetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let value: TypedValue;
        if let Some(value_entry_index) = self.evaluate(&set_expr.value, arena, env)? {
            let value_arena_entry = &arena[value_entry_index];
            value = value_arena_entry.try_into()?;
        } else {
            return Err(LangErrorType::new_iie_error(
                "Set expr failed to retrieve the value to set".into(),
            ));
        }
        if let Some(object_entry_index) = self.evaluate(&set_expr.object, arena, env)? {
            {
                let object_arena_entry = &mut arena[object_entry_index];
                let object: &mut TypedValue = object_arena_entry.try_into()?;
                if let Value::SelfIndex(ref s) = object.value {
                    let prev_object_index = env.get_two(&s.env_id, &s.name)?;
                    let prev_object_entry = &mut arena[prev_object_index];
                    let prev_object: &mut TypedValue = prev_object_entry.try_into()?;
                    if let Value::Struct(ref mut struct_value) = prev_object.value {
                        if !struct_value.field_exists(&set_expr.name) {
                            return Err(LangErrorType::new_runtime_error(
                                RuntimeErrorType::UndefinedVariable {
                                    reason: "Tried to set an undefined struct field".to_string(),
                                },
                            ));
                        }
                        struct_value.set_field(&set_expr.name, &value)?;
                        return Ok(None);
                    }
                }
            }
            let object_arena_entry = &mut arena[object_entry_index];
            let object: &mut TypedValue = object_arena_entry.try_into()?;
            match object.value {
                Value::Struct(ref mut struct_value) => {
                    if !struct_value.field_exists(&set_expr.name) {
                        return Err(LangErrorType::new_runtime_error(
                            RuntimeErrorType::UndefinedVariable {
                                reason: "Tried to set an undefined struct field".to_string(),
                            },
                        ));
                    }
                    if let Expr::Variable(ref var) = set_expr.object {
                        let root_id = env.root_entry_id.clone();
                        env.update_value_two(
                            &root_id,
                            &var.name,
                            arena,
                            |struct_value: &mut TypedValue| -> Result<(), LangError> {
                                let struct_value: &mut dyn StructInstanceTrait =
                                    (&mut struct_value.value).try_into()?;
                                struct_value.set_field(&set_expr.name, &value)?;
                                Ok(())
                            },
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
        }
        Ok(None)
    }

    fn visit_set_array_element_expr(
        &self,
        set_array_element_expr: &SetArrayElementExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let index: usize;
        if let Some(index_entry_index) = self.evaluate(&set_array_element_expr.index, arena, env)? {
            let index_arena_entry = &arena[index_entry_index];
            let index_value: &TypedValue = index_arena_entry.try_into()?;
            index = index_value.as_array_index()?;
        } else {
            return Err(LangErrorType::new_iie_error(
                "could not set array element".into(),
            ));
        }
        if let Some(value_entry_index) = self.evaluate(&set_array_element_expr.value, arena, env)? {
            let value_arena_entry = &arena[value_entry_index];
            let value: TypedValue = value_arena_entry.try_into()?;
            let root_id = env.root_entry_id.clone();
            env.assign_index_entry_two(
                &root_id,
                &set_array_element_expr.name,
                &value,
                arena,
                index,
            )?;
        }
        Ok(None)
    }

    fn visit_array_expr(
        &self,
        array_expr: &ArrayExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let mut elements = Vec::new();
        let mut type_annotation = TypeAnnotation::Unit;
        let mut array_element_type = TypeAnnotation::Unit;
        // Invariant used in order evaluate the initial array element only once
        let mut first_element = true;
        for item in array_expr.elements.iter() {
            if let Some(item_index) = self.evaluate(&item, arena, env)? {
                let item_arena_entry = &arena[item_index];
                let element: &TypedValue = item_arena_entry.try_into()?;
                if first_element {
                    array_element_type = element.value_type.clone();
                    first_element = false;
                } else {
                    TypeChecker::check_type(&array_element_type, &element.value_type)?;
                }
                elements.push(element.clone());
            }
        }
        if let Some(ref type_annotation_set) = array_expr.type_annotation {
            type_annotation = type_annotation_set.to_type_annotation()?;
        }
        if type_annotation == TypeAnnotation::Unit {
            type_annotation = TypeAnnotation::Array(Box::new(array_element_type));
        }
        Ok(Some(arena.insert(TypedValue::new(
            Value::Array(elements),
            type_annotation,
        ))))
    }

    fn visit_index_expr(
        &self,
        index_expr: &IndexExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(index_entry_index) = self.evaluate(&index_expr.index, arena, env)? {
            let index_arena_entry = &arena[index_entry_index];
            let index_value: &TypedValue = index_arena_entry.try_into()?;
            let index = index_value.as_array_index()?;
            let root_id = env.root_entry_id.clone();
            let value_entry_index = env.get_two(&root_id, &index_expr.from)?;
            let value_arena_entry = &mut arena[value_entry_index];
            let value: &mut TypedValue = value_arena_entry.try_into()?;
            let value_at_index = match value.value {
                Value::Array(ref mut arr) => {
                    if index < arr.len() {
                        arr[index].clone()
                    } else {
                        return Err(LangErrorType::new_runtime_error(
                                RuntimeErrorType::GenericError {
                                    reason: format!("Index out of bounds. Tried to index at {} for an array of length {}", index, arr.len()),
                                },
                            ));
                    }
                }
                _ => {
                    return Err(LangErrorType::new_runtime_error(
                        RuntimeErrorType::GenericError {
                            reason: "Tried to index a non-array value. This should never happen"
                                .to_string(),
                        },
                    ))
                }
            };
            return Ok(Some(arena.insert(value_at_index)));
        }
        Ok(None)
    }

    #[inline(always)]
    pub fn interpret_two(&self, stmts: Vec<Stmt>) -> Result<(), LangError> {
        let mut env = Environment::new();
        let mut arena: Arena<TypedValue> = Arena::new();
        for stmt in stmts {
            self.execute(&stmt, &mut arena, &mut env)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn execute(
        &self,
        stmt: &Stmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_stmt(stmt, arena, env)?)
    }

    fn look_up_variable(
        &self,
        token: &str,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        debug!(
            "{}:{} Looking for token '{:?}' within env '{:?}', arena '{:?}' and locals\n'{}'",
            file!(),
            line!(),
            token,
            env,
            arena,
            self.pretty_print_locals()
        );
        if let Some(distance) = self.locals.get(token) {
            if let Ok(value_index) = env.get_two(&EnvironmentId { index: *distance }, &token) {
                let arena_entry = &arena[value_index];
                let value: TypedValue = arena_entry.try_into()?;
                let new_index = match value.value {
                    Value::SelfIndex(ref s) => {
                        let str_val_index = env.get_two(&s.env_id, &s.name)?;
                        let str_val_entry = &arena[str_val_index];
                        let str_val: TypedValue = str_val_entry.try_into()?;
                        arena.insert(str_val)
                    }
                    _ => arena.insert(value),
                };
                Ok(Some(new_index))
            } else {
                let value_index = env.get_two(
                    &EnvironmentId {
                        index: *distance + 1,
                    },
                    &token,
                )?;
                let value_entry = &arena[value_index];
                let value: TypedValue = value_entry.try_into()?;
                Ok(Some(arena.insert(value)))
            }
        } else {
            let root_id = env.root_entry_id.clone();
            let value_index = env.get_two(&root_id, &token)?;
            let value_entry = &arena[value_index];
            let value: TypedValue = value_entry.try_into()?;
            Ok(Some(arena.insert(value)))
        }
    }

    pub fn execute_block(
        &self,
        stmts: &[Stmt],
        env_id: &mut EnvironmentId,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let previous = env_id.clone();
        for stmt in stmts {
            match stmt {
                Stmt::Return(_) => {
                    // Set value and break early on a return
                    if let Some(index) = self.execute(&stmt, arena, env)? {
                        env.remove_entry(&env_id);
                        *env_id = previous;
                        return Ok(Some(index));
                    }
                }
                _ => {
                    if let Some(index) = self.execute(&stmt, arena, env)? {
                        env.remove_entry(&env_id);
                        *env_id = previous;
                        return Ok(Some(index));
                    }
                }
            }
        }
        env.remove_entry(&env_id);
        *env_id = previous;
        Ok(None)
    }

    fn check_impl_trait_return_type(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        if let Some(return_type) = callable.get_return_type() {
            if trait_function.function.return_type != return_type {
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
        impl_trait: &str,
        fn_value: &Value,
        env: &Environment,
        arena: &Arena<TypedValue>,
        trait_token: &str,
    ) -> Result<bool, LangError> {
        let root_id = env.root_entry_id.clone();
        let typed_trait_value_idx = env.get_two(&root_id, trait_token)?;
        let entry = &arena[typed_trait_value_idx];
        let typed_trait_value: &TypedValue = entry.try_into()?;
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

impl Visitor<Option<ArenaEntryIndex>> for Interpreter {
    fn visit_expr(
        &self,
        expr: &Expr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_expr(self, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        stmt: &Stmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_stmt(self, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        assign: &AssignExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_assign_expr(assign, arena, env)?)
    }
    fn visit_binary(
        &self,
        binary: &BinaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_binary_expr(binary, arena, env)?)
    }
    fn visit_call(
        &self,
        call: &CallExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_call_expr(call, arena, env)?)
    }
    fn visit_get(
        &self,
        get: &GetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_get_expr(get, arena, env)?)
    }
    fn visit_enum_path(
        &self,
        _: &EnumPathExpr,
        _: &mut Arena<TypedValue>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_grouping(
        &self,
        _: &GroupingExpr,
        _: &mut Arena<TypedValue>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_literal(
        &self,
        literal: &LiteralExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        match literal.value.value_type {
            TypeAnnotation::User(ref user_type) => {
                let root_id = env.root_entry_id.clone();
                let value_index = env.get_two(&root_id, &user_type)?;
                let value_entry = &arena[value_index];
                let value: TypedValue = value_entry.try_into()?;
                Ok(Some(arena.insert(value)))
            }
            _ => Ok(Some(arena.insert(literal.value.clone()))),
        }
    }
    fn visit_logical(
        &self,
        logical: &LogicalExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_logical_expr(logical, arena, env)?)
    }
    fn visit_set(
        &self,
        set: &SetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_expr(set, arena, env)?)
    }
    fn visit_unary(
        &self,
        unary: &UnaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_unary_expr(unary, arena, env)?)
    }
    fn visit_array(
        &self,
        array: &ArrayExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_array_expr(array, arena, env)?)
    }
    fn visit_index(
        &self,
        index: &IndexExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_index_expr(index, arena, env)?)
    }
    fn visit_set_array_element(
        &self,
        set_array_element: &SetArrayElementExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_array_element_expr(set_array_element, arena, env)?)
    }
    fn visit_variable(
        &self,
        variable: &VariableExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&variable.name, arena, env)?)
    }
    fn visit_self_ident(
        &self,
        self_ident: &SelfIdentExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&self_ident.keyword, arena, env)?)
    }

    fn visit_break(&self) -> Result<Option<ArenaEntryIndex>, LangError> {
        Err(LangError::from(LangErrorType::ControlFlow {
            subtype: ControlFlow::Break,
        }))
    }

    fn visit_assert(
        &self,
        assert_stmt: &AssertStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(assert_stmt_index) = self.evaluate(&assert_stmt.condition, arena, env)? {
            let arena_entry = &arena[assert_stmt_index];
            let eval = match arena_entry {
                ArenaEntry::Occupied(v) => v,
                ArenaEntry::Emtpy => panic!(),
            };
            if self.is_truthy(&eval.value) {
                return Ok(Some(assert_stmt_index));
            } else {
                println!("assert failed");
                return Err(LangError::from(LangErrorType::ControlFlow {
                    subtype: ControlFlow::Assert,
                }));
            }
        }
        Ok(None)
    }
    fn visit_enum(
        &self,
        _: &EnumStmt,
        _: &mut Arena<TypedValue>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_impl(
        &self,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_stmt(impl_stmt, arena, env)?)
    }
    fn visit_impl_trait(
        &self,
        impl_trait: &ImplTraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_trait_stmt(impl_trait, arena, env)?)
    }
    fn visit_block(
        &self,
        block: &BlockStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let mut env_id = env.entry_from(&env.root_entry_id.clone());
        self.execute_block(&block.statements, &mut env_id, arena, env)?;
        Ok(None)
    }
    fn visit_struct(
        &self,
        block: &StructStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_struct_stmt(&block, arena, env)?)
    }
    fn visit_expression(
        &self,
        block: &ExpressionStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(&block.expression, arena, env)?)
    }
    fn visit_trait(
        &self,
        block: &TraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_trait_stmt(block, arena, env)?)
    }
    fn visit_trait_function(
        &self,
        trait_fn_stmt: &TraitFunctionStmt,
        arena: &mut Arena<TypedValue>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let trait_fn = Value::TraitFunction(Box::new(TraitFunctionValue {
            function: trait_fn_stmt.clone(),
        }));
        Ok(Some(
            arena.insert(TypedValue::new(trait_fn, TypeAnnotation::Fn)),
        ))
    }
    fn visit_function(
        &self,
        function_stmt: &FunctionStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let root_id = env.root_entry_id.clone();
        let function = Value::Callable(Box::new(Callable::new(function_stmt.clone(), &root_id)));
        env.define_two(
            &root_id,
            arena,
            &function_stmt.name,
            TypedValue::new(function, TypeAnnotation::Fn),
        );
        Ok(None)
    }
    fn visit_if(
        &self,
        if_stmt: &IfStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        debug!("if_stmt condition {:#?}", if_stmt.condition);
        if let Some(if_stmt_index) = self.evaluate(&if_stmt.condition, arena, env)? {
            let arena_entry = &arena[if_stmt_index];
            let eval = match arena_entry {
                ArenaEntry::Occupied(v) => v,
                ArenaEntry::Emtpy => panic!(),
            };
            if self.is_truthy(&eval.value) {
                return Ok(self.execute(&if_stmt.then_branch, arena, env)?);
            }
            if let Some(ref else_branch) = if_stmt.else_branch {
                return Ok(self.execute(&else_branch, arena, env)?);
            }
        }
        Ok(None)
    }
    fn visit_print(
        &self,
        print_stmt: &PrintStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(expr_index) = self.evaluate(&print_stmt.expression, arena, env)? {
            let arena_entry = &arena[expr_index];
            let value: &TypedValue = arena_entry.try_into()?;
            println!("{}", value.value);
        }
        Ok(None)
    }
    fn visit_return(
        &self,
        return_stmt: &ReturnStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let value = Expr::Literal(Box::new(LiteralExpr::new(TypedValue::new(
            Value::Unit,
            TypeAnnotation::Unit,
        ))));
        let return_value = if return_stmt.value != value {
            if let Some(return_value_index) = self.evaluate(&return_stmt.value, arena, env)? {
                let return_value_entry = &arena[return_value_index];
                return_value_entry.try_into()?
            } else {
                TypedValue::new(Value::Unit, TypeAnnotation::Unit)
            }
        } else {
            TypedValue::new(Value::Unit, TypeAnnotation::Unit)
        };
        Ok(Some(arena.insert(return_value)))
    }
    fn visit_var(
        &self,
        var_stmt: &VarStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let mut value = TypedValue::new(Value::Unit, TypeAnnotation::Unit);
        if let Some(ref initializer) = var_stmt.initializer {
            if let Some(value_index) = self.evaluate(&initializer, arena, env)? {
                let value_entry = &arena[value_index];
                value = match value_entry {
                    ArenaEntry::Occupied(v) => v.clone(),
                    ArenaEntry::Emtpy => panic!(),
                };
            }
        }
        let var_type_annotation = var_stmt.type_annotation.clone();
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
        let root_id = env.root_entry_id.clone();
        env.define_two(&root_id, arena, &var_stmt.name, value);
        Ok(Some(arena.insert(TypedValue::new(
            Value::Unit,
            TypeAnnotation::Unit,
        ))))
    }
    fn visit_while(
        &self,
        while_stmt: &WhileStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        if let Some(condition_index) = self.evaluate(&while_stmt.condition, arena, env)? {
            let condition_entry = &arena[condition_index];
            let mut while_condition: &TypedValue = condition_entry.try_into()?;
            while self.is_truthy(&while_condition.value) {
                if let Err(error) = self.execute(&while_stmt.body, arena, env) {
                    match error.context.get_context() {
                        LangErrorType::ControlFlow { .. } => {
                            break;
                        }
                        other => {
                            return Err(LangError::from((*other).clone()));
                        }
                    }
                }
                let while_condition_index =
                    self.evaluate(&while_stmt.condition, arena, env)?.unwrap();
                let while_condition_entry = &arena[while_condition_index];
                while_condition = match while_condition_entry {
                    ArenaEntry::Occupied(v) => v,
                    ArenaEntry::Emtpy => panic!(),
                };
            }
        }
        Ok(None)
    }
    fn visit_import(
        &self,
        _: &ImportStmt,
        _: &mut Arena<TypedValue>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
}
