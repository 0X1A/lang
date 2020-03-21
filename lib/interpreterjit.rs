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
use std::collections::HashMap;
use std::convert::TryInto;

pub struct IRGenerator<'context> {
    context: &'context Context,
    module: Module<'context>,
    builder: Builder<'context>,
    exec_engine: ExecutionEngine<'context>,
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

    fn evaluate(
        &self,
        context: &Context,
        expr: &Expr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_expr(context, expr, arena, env)?)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        unimplemented!()
    }

    fn visit_assign_expr(
        &self,
        context: &Context,
        assign: &AssignExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: Clean this up. The nested iflets are an eyesore
    fn visit_call_expr(
        &self,
        context: &Context,
        call: &CallExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_get_expr(
        &self,
        context: &Context,
        get_expr: &GetExpr,
        arena: &mut Arena<()>,
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

    fn visit_impl_trait_stmt(
        &self,
        context: &Context,
        impl_trait_stmt: &ImplTraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_trait_stmt(
        &self,
        context: &Context,
        trait_stmt: &TraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_impl_stmt(
        &self,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_struct_stmt(
        &self,
        context: &Context,
        struct_stmt: &StructStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // Visit Expr stuff

    fn visit_binary_expr(
        &self,
        context: &Context,
        expr: &BinaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_unary_expr(
        &self,
        context: &Context,
        unary_expr: &UnaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_logical_expr(
        &self,
        context: &Context,
        logical_expr: &LogicalExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: this shit is _still_ a mess
    fn visit_set_expr(
        &self,
        context: &Context,
        set_expr: &SetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_set_array_element_expr(
        &self,
        context: &Context,
        set_array_element_expr: &SetArrayElementExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_array_expr(
        &self,
        context: &Context,
        array_expr: &ArrayExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_index_expr(
        &self,
        context: &Context,
        index_expr: &IndexExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    #[inline(always)]
    pub fn interpret(&self, context: &Context, stmts: Vec<Stmt>) -> Result<(), LangError> {
        unimplemented!()
    }

    #[inline(always)]
    fn execute(
        &self,
        context: &Context,
        stmt: &Stmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn look_up_variable(
        &self,
        token: &str,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    pub fn execute_block(
        &self,
        context: &Context,
        stmts: &[Stmt],
        env_id: &mut EnvironmentEntryIndex,
        arena: &mut Arena<()>,
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

    fn check_impl_trait(
        &self,
        impl_trait: &str,
        fn_value: &Value,
        env: &Environment,
        arena: &Arena<()>,
        trait_token: &str,
    ) -> Result<bool, LangError> {
        unimplemented!()
    }
}

impl Visitor<Option<ArenaEntryIndex>, ()> for InterpreterJIT {
    fn visit_expr(
        &self,
        context: &Context,
        expr: &Expr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &Context,
        stmt: &Stmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_stmt(self, context, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        context: &Context,
        assign: &AssignExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_assign_expr(context, assign, arena, env)?)
    }
    fn visit_binary(
        &self,
        context: &Context,
        binary: &BinaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_binary_expr(context, binary, arena, env)?)
    }
    fn visit_call(
        &self,
        context: &Context,
        call: &CallExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_call_expr(context, call, arena, env)?)
    }
    fn visit_get(
        &self,
        context: &Context,
        get: &GetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_get_expr(context, get, arena, env)?)
    }
    fn visit_enum_path(
        &self,
        _: &Context,
        _: &EnumPathExpr,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_grouping(
        &self,
        context: &Context,
        grouping_expr: &GroupingExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.evaluate(context, &grouping_expr.expression, arena, env)?)
    }
    fn visit_literal(
        &self,
        context: &Context,
        literal: &LiteralExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_logical(
        &self,
        context: &Context,
        logical: &LogicalExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_logical_expr(context, logical, arena, env)?)
    }
    fn visit_set(
        &self,
        context: &Context,
        set: &SetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_expr(context, set, arena, env)?)
    }
    fn visit_unary(
        &self,
        context: &Context,
        unary: &UnaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_unary_expr(context, unary, arena, env)?)
    }
    fn visit_array(
        &self,
        context: &Context,
        array: &ArrayExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_array_expr(context, array, arena, env)?)
    }
    fn visit_index(
        &self,
        context: &Context,
        index: &IndexExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_index_expr(context, index, arena, env)?)
    }
    fn visit_set_array_element(
        &self,
        context: &Context,
        set_array_element: &SetArrayElementExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_array_element_expr(context, set_array_element, arena, env)?)
    }
    fn visit_variable(
        &self,
        context: &Context,
        variable: &VariableExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&variable.name, arena, env)?)
    }
    fn visit_self_ident(
        &self,
        context: &Context,
        self_ident: &SelfIdentExpr,
        arena: &mut Arena<()>,
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
        context: &Context,
        assert_stmt: &AssertStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_enum(
        &self,
        _: &Context,
        _: &EnumStmt,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_impl(
        &self,
        context: &Context,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_stmt(impl_stmt, arena, env)?)
    }
    fn visit_impl_trait(
        &self,
        context: &Context,
        impl_trait: &ImplTraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_trait_stmt(context, impl_trait, arena, env)?)
    }
    fn visit_block(
        &self,
        context: &Context,
        block: &BlockStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_struct(
        &self,
        context: &Context,
        block: &StructStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_expression(
        &self,
        context: &Context,
        block: &ExpressionStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait(
        &self,
        context: &Context,
        block: &TraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait_function(
        &self,
        context: &Context,
        trait_fn_stmt: &TraitFunctionStmt,
        arena: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_function(
        &self,
        context: &Context,
        function_stmt: &FunctionStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_if(
        &self,
        context: &Context,
        if_stmt: &IfStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_print(
        &self,
        context: &Context,
        print_stmt: &PrintStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_return(
        &self,
        context: &Context,
        return_stmt: &ReturnStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_var(
        &self,
        context: &Context,
        var_stmt: &VarStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_while(
        &self,
        context: &Context,
        while_stmt: &WhileStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_import(
        &self,
        _: &Context,
        _: &ImportStmt,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
}
