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

pub struct IRGenerator<'context> {
    pub context: &'context Context,
    pub module: Module<'context>,
    pub builder: Builder<'context>,
    pub exec_engine: ExecutionEngine<'context>,
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

    fn evaluate<'context>(
        &self,
        context: &'context IRGenerator,
        expr: &Expr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(context, expr, arena, env)?)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        unimplemented!()
    }

    fn visit_assign_expr<'context>(
        &self,
        context: &'context IRGenerator,
        assign: &AssignExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: Clean this up. The nested iflets are an eyesore
    fn visit_call_expr<'context>(
        &self,
        context: &'context IRGenerator,
        call: &CallExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_get_expr<'context>(
        &self,
        context: &'context IRGenerator,
        get_expr: &GetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
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

    fn visit_impl_trait_stmt<'context>(
        &self,
        context: &'context IRGenerator,
        impl_trait_stmt: &ImplTraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_trait_stmt<'context>(
        &self,
        context: &'context IRGenerator,
        trait_stmt: &TraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_impl_stmt<'context>(
        &self,
        impl_stmt: &ImplStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_struct_stmt<'context>(
        &self,
        context: &'context IRGenerator,
        struct_stmt: &StructStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // Visit Expr stuff

    fn visit_binary_expr<'context>(
        &self,
        context: &'context IRGenerator,
        expr: &BinaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_unary_expr<'context>(
        &self,
        context: &'context IRGenerator,
        unary_expr: &UnaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_logical_expr<'context>(
        &self,
        context: &'context IRGenerator,
        logical_expr: &LogicalExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: this shit is _still_ a mess
    fn visit_set_expr<'context>(
        &self,
        context: &'context IRGenerator,
        set_expr: &SetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_set_array_element_expr<'context>(
        &self,
        context: &'context IRGenerator,
        set_array_element_expr: &SetArrayElementExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_array_expr<'context>(
        &self,
        context: &'context IRGenerator,
        array_expr: &ArrayExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_index_expr<'context>(
        &self,
        context: &'context IRGenerator,
        index_expr: &IndexExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
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
        let ir_gen = IRGenerator {
            context: &context,
            module,
            builder,
            exec_engine,
        };
        for stmt in stmts {
            self.execute(&ir_gen, &stmt, &mut arena, &mut env)?;
        }
        Ok(())
    }

    #[inline(always)]
    fn execute<'context: 'arena, 'arena>(
        &self,
        context: &'context IRGenerator,
        stmt: &Stmt,
        arena: &'arena mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn look_up_variable<'context>(
        &self,
        token: &str,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    pub fn execute_block<'context>(
        &self,
        context: &'context IRGenerator,
        stmts: &[Stmt],
        env_id: &mut EnvironmentEntryIndex,
        arena: &'context mut Arena<AnyValueType<'context>>,
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

    fn check_impl_trait<'context>(
        &self,
        impl_trait: &str,
        fn_value: &Value,
        env: &Environment,
        arena: &Arena<AnyValueType<'context>>,
        trait_token: &str,
    ) -> Result<bool, LangError> {
        unimplemented!()
    }
}

impl<'context: 'arena, 'arena> Visitor<'context, Option<ArenaEntryIndex>> for InterpreterJIT {
    fn visit_expr(
        &self,
        context: &'context IRGenerator,
        expr: &Expr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &'context IRGenerator,
        stmt: &Stmt,
        arena: &'arena mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_assign(
        &self,
        context: &'context IRGenerator,
        assign: &AssignExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_assign_expr(context, assign, arena, env)?)
    }
    fn visit_binary(
        &self,
        context: &'context IRGenerator,
        binary: &BinaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_binary_expr(context, binary, arena, env)?)
    }
    fn visit_call(
        &self,
        context: &'context IRGenerator,
        call: &CallExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_call_expr(context, call, arena, env)?)
    }
    fn visit_get(
        &self,
        context: &'context IRGenerator,
        get: &GetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_get_expr(context, get, arena, env)?)
    }
    fn visit_enum_path(
        &self,
        _: &'context IRGenerator,
        _: &EnumPathExpr,
        _: &mut Arena<AnyValueType<'context>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_grouping(
        &self,
        context: &'context IRGenerator,
        grouping_expr: &GroupingExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.evaluate(context, &grouping_expr.expression, arena, env)?)
    }
    fn visit_literal(
        &self,
        context: &'context IRGenerator,
        literal: &LiteralExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_logical(
        &self,
        context: &'context IRGenerator,
        logical: &LogicalExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_logical_expr(context, logical, arena, env)?)
    }
    fn visit_set(
        &self,
        context: &'context IRGenerator,
        set: &SetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_expr(context, set, arena, env)?)
    }
    fn visit_unary(
        &self,
        context: &'context IRGenerator,
        unary: &UnaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_unary_expr(context, unary, arena, env)?)
    }
    fn visit_array(
        &self,
        context: &'context IRGenerator,
        array: &ArrayExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_array_expr(context, array, arena, env)?)
    }
    fn visit_index(
        &self,
        context: &'context IRGenerator,
        index: &IndexExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_index_expr(context, index, arena, env)?)
    }
    fn visit_set_array_element(
        &self,
        context: &'context IRGenerator,
        set_array_element: &SetArrayElementExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_array_element_expr(context, set_array_element, arena, env)?)
    }
    fn visit_variable(
        &self,
        context: &'context IRGenerator,
        variable: &VariableExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&variable.name, arena, env)?)
    }
    fn visit_self_ident(
        &self,
        context: &'context IRGenerator,
        self_ident: &SelfIdentExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
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
        context: &'context IRGenerator,
        assert_stmt: &AssertStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_enum(
        &self,
        _: &'context IRGenerator,
        _: &EnumStmt,
        _: &mut Arena<AnyValueType<'context>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_impl(
        &self,
        context: &'context IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_stmt(impl_stmt, arena, env)?)
    }
    fn visit_impl_trait(
        &self,
        context: &'context IRGenerator,
        impl_trait: &ImplTraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_trait_stmt(context, impl_trait, arena, env)?)
    }
    fn visit_block(
        &self,
        context: &'context IRGenerator,
        block: &BlockStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_struct(
        &self,
        context: &'context IRGenerator,
        block: &StructStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        let struct_t = context.context.struct_type(&[], true);
        let index = arena.insert(AnyValueType::BasicType(BasicTypeEnum::StructType(struct_t)));
        Ok(Some(index))
    }
    fn visit_expression(
        &self,
        context: &'context IRGenerator,
        block: &ExpressionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait(
        &self,
        context: &'context IRGenerator,
        block: &TraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait_function(
        &self,
        context: &'context IRGenerator,
        trait_fn_stmt: &TraitFunctionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_function(
        &self,
        context: &'context IRGenerator,
        function_stmt: &FunctionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_if(
        &self,
        context: &'context IRGenerator,
        if_stmt: &IfStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_print(
        &self,
        context: &'context IRGenerator,
        print_stmt: &PrintStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_return(
        &self,
        context: &'context IRGenerator,
        return_stmt: &ReturnStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_var(
        &self,
        context: &'context IRGenerator,
        var_stmt: &VarStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_while(
        &self,
        context: &'context IRGenerator,
        while_stmt: &WhileStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_import(
        &self,
        _: &'context IRGenerator,
        _: &ImportStmt,
        _: &mut Arena<AnyValueType<'context>>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
}
