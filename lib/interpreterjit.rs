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

    fn evaluate(
        &self,
        context: &IRGenerator,
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
        context: &IRGenerator,
        assign: &AssignExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: Clean this up. The nested iflets are an eyesore
    fn visit_call_expr(
        &self,
        context: &IRGenerator,
        call: &CallExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_get_expr(
        &self,
        context: &IRGenerator,
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
        context: &IRGenerator,
        impl_trait_stmt: &ImplTraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_trait_stmt(
        &self,
        context: &IRGenerator,
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
        context: &IRGenerator,
        struct_stmt: &StructStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // Visit Expr stuff

    fn visit_binary_expr(
        &self,
        context: &IRGenerator,
        expr: &BinaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_unary_expr(
        &self,
        context: &IRGenerator,
        unary_expr: &UnaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_logical_expr(
        &self,
        context: &IRGenerator,
        logical_expr: &LogicalExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    // TODO: this shit is _still_ a mess
    fn visit_set_expr(
        &self,
        context: &IRGenerator,
        set_expr: &SetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_set_array_element_expr(
        &self,
        context: &IRGenerator,
        set_array_element_expr: &SetArrayElementExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_array_expr(
        &self,
        context: &IRGenerator,
        array_expr: &ArrayExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    fn visit_index_expr(
        &self,
        context: &IRGenerator,
        index_expr: &IndexExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }

    #[inline(always)]
    pub fn interpret(&self, context: &IRGenerator, stmts: Vec<Stmt>) -> Result<(), LangError> {
        unimplemented!()
    }

    #[inline(always)]
    fn execute(
        &self,
        context: &IRGenerator,
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
        context: &IRGenerator,
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
        context: &IRGenerator,
        expr: &Expr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &IRGenerator,
        stmt: &Stmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(visit_stmt(self, context, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        context: &IRGenerator,
        assign: &AssignExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_assign_expr(context, assign, arena, env)?)
    }
    fn visit_binary(
        &self,
        context: &IRGenerator,
        binary: &BinaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_binary_expr(context, binary, arena, env)?)
    }
    fn visit_call(
        &self,
        context: &IRGenerator,
        call: &CallExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_call_expr(context, call, arena, env)?)
    }
    fn visit_get(
        &self,
        context: &IRGenerator,
        get: &GetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_get_expr(context, get, arena, env)?)
    }
    fn visit_enum_path(
        &self,
        _: &IRGenerator,
        _: &EnumPathExpr,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
    fn visit_grouping(
        &self,
        context: &IRGenerator,
        grouping_expr: &GroupingExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.evaluate(context, &grouping_expr.expression, arena, env)?)
    }
    fn visit_literal(
        &self,
        context: &IRGenerator,
        literal: &LiteralExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_logical(
        &self,
        context: &IRGenerator,
        logical: &LogicalExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_logical_expr(context, logical, arena, env)?)
    }
    fn visit_set(
        &self,
        context: &IRGenerator,
        set: &SetExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_expr(context, set, arena, env)?)
    }
    fn visit_unary(
        &self,
        context: &IRGenerator,
        unary: &UnaryExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_unary_expr(context, unary, arena, env)?)
    }
    fn visit_array(
        &self,
        context: &IRGenerator,
        array: &ArrayExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_array_expr(context, array, arena, env)?)
    }
    fn visit_index(
        &self,
        context: &IRGenerator,
        index: &IndexExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_index_expr(context, index, arena, env)?)
    }
    fn visit_set_array_element(
        &self,
        context: &IRGenerator,
        set_array_element: &SetArrayElementExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_set_array_element_expr(context, set_array_element, arena, env)?)
    }
    fn visit_variable(
        &self,
        context: &IRGenerator,
        variable: &VariableExpr,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.look_up_variable(&variable.name, arena, env)?)
    }
    fn visit_self_ident(
        &self,
        context: &IRGenerator,
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
        context: &IRGenerator,
        assert_stmt: &AssertStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_enum(
        &self,
        _: &IRGenerator,
        _: &EnumStmt,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_impl(
        &self,
        context: &IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_stmt(impl_stmt, arena, env)?)
    }
    fn visit_impl_trait(
        &self,
        context: &IRGenerator,
        impl_trait: &ImplTraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(self.visit_impl_trait_stmt(context, impl_trait, arena, env)?)
    }
    fn visit_block(
        &self,
        context: &IRGenerator,
        block: &BlockStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_struct(
        &self,
        context: &IRGenerator,
        block: &StructStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_expression(
        &self,
        context: &IRGenerator,
        block: &ExpressionStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait(
        &self,
        context: &IRGenerator,
        block: &TraitStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_trait_function(
        &self,
        context: &IRGenerator,
        trait_fn_stmt: &TraitFunctionStmt,
        arena: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_function(
        &self,
        context: &IRGenerator,
        function_stmt: &FunctionStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_if(
        &self,
        context: &IRGenerator,
        if_stmt: &IfStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_print(
        &self,
        context: &IRGenerator,
        print_stmt: &PrintStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_return(
        &self,
        context: &IRGenerator,
        return_stmt: &ReturnStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_var(
        &self,
        context: &IRGenerator,
        var_stmt: &VarStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_while(
        &self,
        context: &IRGenerator,
        while_stmt: &WhileStmt,
        arena: &mut Arena<()>,
        env: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        unimplemented!()
    }
    fn visit_import(
        &self,
        _: &IRGenerator,
        _: &ImportStmt,
        _: &mut Arena<()>,
        _: &mut Environment,
    ) -> Result<Option<ArenaEntryIndex>, LangError> {
        Ok(None)
    }
}
