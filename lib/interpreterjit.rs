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
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::types::*;
use inkwell::values::*;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct IRGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub exec_engine: ExecutionEngine<'ctx>,
}

#[derive(Debug)]
pub struct InterpreterJIT {}

impl Default for InterpreterJIT {
    fn default() -> InterpreterJIT {
        InterpreterJIT {}
    }
}

impl InterpreterJIT {
    pub fn new() -> InterpreterJIT {
        InterpreterJIT {}
    }

    fn evaluate<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        expr: &Expr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(context, expr, arena, env)?)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        unimplemented!()
    }

    fn visit_assign_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        assign: &AssignExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: Clean this up. The nested iflets are an eyesore
    fn visit_call_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        call: &CallExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_get_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        get_expr: &GetExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn execute_binary_op(
        &self,
        op: &TokenType,
        left: &TypedValue,
        right: &TypedValue,
    ) -> Result<TypedValue, LangError> {
        unimplemented!()
    }

    fn visit_impl_trait_stmt<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        impl_trait_stmt: &ImplTraitStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_trait_stmt<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        trait_stmt: &TraitStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_impl_stmt<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        for fn_decl in impl_stmt.fn_declarations.iter() {
            if let Stmt::Function(fn_stmt) = fn_decl {
                let mut fn_param = Vec::new();
                for param in fn_stmt.params.iter() {
                    match param.type_annotation {
                        TypeAnnotation::I32 => {
                            fn_param.push(BasicTypeEnum::IntType(context.context.i32_type()))
                        }
                        TypeAnnotation::I64 => {
                            fn_param.push(BasicTypeEnum::IntType(context.context.i64_type()))
                        }
                        TypeAnnotation::F32 => {
                            fn_param.push(BasicTypeEnum::FloatType(context.context.f32_type()))
                        }
                        TypeAnnotation::F64 => {
                            fn_param.push(BasicTypeEnum::FloatType(context.context.f64_type()))
                        }
                        TypeAnnotation::Bool => {
                            fn_param.push(BasicTypeEnum::IntType(context.context.bool_type()))
                        }
                        _ => fn_param.push(BasicTypeEnum::IntType(context.context.i32_type())),
                    }
                }
                let return_t = match fn_stmt.return_type.to_type_annotation()? {
                    TypeAnnotation::I32 => context.context.i32_type().fn_type(&fn_param, false),
                    _ => context.context.i32_type().fn_type(&fn_param, false),
                };
                let fn_value = context.module.add_function(&fn_stmt.name, return_t, None);
                arena.insert(AnyValueType::AnyValue(AnyValueEnum::FunctionValue(
                    fn_value,
                )));
            }
        }
        Ok(None)
    }

    fn visit_struct_stmt<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        struct_stmt: &StructStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // Visit Expr stuff

    fn visit_binary_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        expr: &BinaryExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_unary_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        unary_expr: &UnaryExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_logical_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        logical_expr: &LogicalExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: this shit is _still_ a mess
    fn visit_set_expr<'arna, 'ctx: 'arna>(
        &self,
        context: &'ctx IRGenerator,
        set_expr: &SetExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_set_array_element_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        set_array_element_expr: &SetArrayElementExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_array_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        array_expr: &ArrayExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_index_expr<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        index_expr: &IndexExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    #[inline(always)]
    pub fn interpret(&self, stmts: Vec<Stmt>) -> Result<(), LangError> {
        let mut env = Environment::new();
        let mut arena: Arena<AnyValueType> = Arena::with_capacity(256);
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();
        let exec_engine = module.create_execution_engine().unwrap();
        let context = IRGenerator {
            context: &context,
            module,
            builder,
            exec_engine,
        };
        for stmt in stmts {
            self.execute(&context, &stmt, &mut arena, &mut env)?;
        }
        println!("{:?}", arena);
        println!("{:?}", env);
        Ok(())
    }

    #[inline(always)]
    fn execute<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        stmt: &Stmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_stmt(&context, stmt, arena, env)?)
    }

    fn look_up_variable<'ctx: 'arna, 'arna>(
        &self,
        token: &str,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    pub fn execute_block<'ctx: 'arna, 'arna>(
        &self,
        context: &'ctx IRGenerator,
        stmts: &[Stmt],
        env_id: &mut EnvironmentEntryIndex,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn check_impl_trait_return_type(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        unimplemented!()
    }

    fn check_impl_trait_arity(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        unimplemented!()
    }

    fn check_impl_trait_param_types(
        &self,
        callable: &dyn CallableTrait,
        trait_function: &TraitFunctionValue,
    ) -> Result<(), LangError> {
        unimplemented!()
    }

    fn check_impl_trait<'ctx: 'arna, 'arna>(
        &self,
        impl_trait: &str,
        fn_value: &Value,
        env: &Environment,
        arena: &'arna Arena<AnyValueType<'ctx>>,
        trait_token: &str,
    ) -> Result<bool, LangError> {
        unimplemented!()
    }
}

impl<'ctx: 'arna, 'arna> Visitor<'arna, 'ctx, Option<ArenaEntryIndex>> for InterpreterJIT {
    fn visit_expr(
        &self,
        context: &'ctx IRGenerator,
        expr: &Expr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &'ctx IRGenerator,
        stmt: &Stmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_stmt(self, context, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        context: &'ctx IRGenerator,
        assign: &AssignExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_assign_expr(context, assign, arena, env)?)
    }
    fn visit_binary(
        &self,
        context: &'ctx IRGenerator,
        binary: &BinaryExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_binary_expr(context, binary, arena, env)?)
    }
    fn visit_call(
        &self,
        context: &'ctx IRGenerator,
        call: &CallExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_call_expr(context, call, arena, env)?)
    }
    fn visit_get(
        &self,
        context: &'ctx IRGenerator,
        get: &GetExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_get_expr(context, get, arena, env)?)
    }
    fn visit_enum_path(
        &self,
        _: &'ctx IRGenerator,
        _: &EnumPathExpr,
        _: &mut Arena<AnyValueType<'ctx>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_grouping(
        &self,
        context: &'ctx IRGenerator,
        grouping_expr: &GroupingExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.evaluate(context, &grouping_expr.expression, arena, env)?)
    }
    fn visit_literal(
        &self,
        context: &'ctx IRGenerator,
        literal: &LiteralExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let value = match literal.value.value_type {
            TypeAnnotation::I32 => context.context.i32_type().const_int(0, false),
            TypeAnnotation::I64 => context.context.i64_type().const_int(0, false),
            _ => context.context.i32_type().const_int(0, false),
        };
        unimplemented!()
    }
    fn visit_logical(
        &self,
        context: &'ctx IRGenerator,
        logical: &LogicalExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_logical_expr(context, logical, arena, env)?)
    }
    fn visit_set(
        &self,
        context: &'ctx IRGenerator,
        set: &SetExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_expr(context, set, arena, env)?)
    }
    fn visit_unary(
        &self,
        context: &'ctx IRGenerator,
        unary: &UnaryExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_unary_expr(context, unary, arena, env)?)
    }
    fn visit_array(
        &self,
        context: &'ctx IRGenerator,
        array: &ArrayExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_array_expr(context, array, arena, env)?)
    }
    fn visit_index(
        &self,
        context: &'ctx IRGenerator,
        index: &IndexExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_index_expr(context, index, arena, env)?)
    }
    fn visit_set_array_element(
        &self,
        context: &'ctx IRGenerator,
        set_array_element: &SetArrayElementExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_array_element_expr(context, set_array_element, arena, env)?)
    }
    fn visit_variable(
        &self,
        context: &'ctx IRGenerator,
        variable: &VariableExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&variable.name, arena, env)?)
    }
    fn visit_self_ident(
        &self,
        context: &'ctx IRGenerator,
        self_ident: &SelfIdentExpr,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
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
        context: &'ctx IRGenerator,
        assert_stmt: &AssertStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_enum(
        &self,
        _: &'ctx IRGenerator,
        _: &EnumStmt,
        _: &mut Arena<AnyValueType<'ctx>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_impl(
        &self,
        context: &'ctx IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_stmt(context, impl_stmt, arena, env)?)
    }
    fn visit_impl_trait(
        &self,
        context: &'ctx IRGenerator,
        impl_trait: &ImplTraitStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_trait_stmt(context, impl_trait, arena, env)?)
    }
    fn visit_block(
        &self,
        context: &'ctx IRGenerator,
        block: &BlockStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_struct(
        &self,
        context: &'ctx IRGenerator,
        struct_stmt: &StructStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let mut fields_v = Vec::new();
        for field in struct_stmt.fields.iter() {
            match field.type_annotation {
                TypeAnnotation::I32 => {
                    fields_v.push(BasicTypeEnum::IntType(context.context.i32_type()))
                }
                TypeAnnotation::I64 => {
                    fields_v.push(BasicTypeEnum::IntType(context.context.i64_type()))
                }
                TypeAnnotation::Bool => {
                    fields_v.push(BasicTypeEnum::IntType(context.context.bool_type()))
                }
                _ => fields_v.push(BasicTypeEnum::IntType(context.context.i32_type())),
            }
        }
        let struct_t = context.context.struct_type(&fields_v, true);
        let index = arena.insert(AnyValueType::BasicType(BasicTypeEnum::StructType(struct_t)));
        Ok(Some(index))
    }
    fn visit_expression(
        &self,
        context: &'ctx IRGenerator,
        block: &ExpressionStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(&context, &block.expression, arena, env)?)
    }
    fn visit_trait(
        &self,
        context: &'ctx IRGenerator,
        block: &TraitStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait_function(
        &self,
        context: &'ctx IRGenerator,
        trait_fn_stmt: &TraitFunctionStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_function(
        &self,
        context: &'ctx IRGenerator,
        function_stmt: &FunctionStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_if(
        &self,
        context: &'ctx IRGenerator,
        if_stmt: &IfStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_print(
        &self,
        context: &'ctx IRGenerator,
        print_stmt: &PrintStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_return(
        &self,
        context: &'ctx IRGenerator,
        return_stmt: &ReturnStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_var(
        &self,
        context: &'ctx IRGenerator,
        var_stmt: &VarStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        // A variable can be instantiated without being initialized
        if let Some(ref initializer) = var_stmt.initializer {
            if let Some(value_index) = self.evaluate(context, &initializer, arena, env)? {
                let value_entry = &mut arena[value_index];
                let value = match value_entry {
                    ArenaEntry::Occupied(any_val_type) => any_val_type,
                    ArenaEntry::Emtpy => panic!(),
                };
                // Var vaue has already been put into the arena, so we just have to do an insert into the env
                let env_idx = env.current_index;
                env[env_idx]
                    .values
                    .insert(var_stmt.name.clone(), value_index);
                return Ok(Some(value_index));
            }
        } else {
            let val = AnyValueType::BasicValue(BasicValueEnum::IntValue(
                context.context.i32_type().const_int(0, false),
            ));
            let value_index = arena.insert(val);
            return Ok(Some(value_index));
        }
        Ok(None)
    }
    fn visit_while(
        &self,
        context: &'ctx IRGenerator,
        while_stmt: &WhileStmt,
        arena: &'arna mut Arena<AnyValueType<'ctx>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_import(
        &self,
        _: &'ctx IRGenerator,
        _: &ImportStmt,
        _: &mut Arena<AnyValueType<'ctx>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
}
