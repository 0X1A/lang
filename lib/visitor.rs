use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::env::*;
use crate::error::LangError;
use crate::interpreterjit::*;
use crate::mem::*;
use crate::value::AnyValueType;
use crate::value::TypedValue;
use inkwell::context::Context;

// We _have_ to return a concrete definition of Result here since we can't have
// bounds on an associated type in order to use the error propogation operator
pub trait Visitor<'context, T>: Sized {
    fn visit_expr(
        &self,
        context: &'context IRGenerator,
        expr: &Expr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &'context IRGenerator,
        stmt: &Stmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_stmt(self, context, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        context: &'context IRGenerator,
        assign: &AssignExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_binary(
        &self,
        context: &'context IRGenerator,
        binary: &BinaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_call(
        &self,
        context: &'context IRGenerator,
        call: &CallExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_get(
        &self,
        context: &'context IRGenerator,
        get: &GetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum_path(
        &self,
        context: &'context IRGenerator,
        enum_path: &EnumPathExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_grouping(
        &self,
        context: &'context IRGenerator,
        grouping: &GroupingExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_literal(
        &self,
        context: &'context IRGenerator,
        literal: &LiteralExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_logical(
        &self,
        context: &'context IRGenerator,
        logical: &LogicalExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set(
        &self,
        context: &'context IRGenerator,
        set: &SetExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_unary(
        &self,
        context: &'context IRGenerator,
        unary: &UnaryExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_array(
        &self,
        context: &'context IRGenerator,
        array: &ArrayExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_index(
        &self,
        context: &'context IRGenerator,
        index: &IndexExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set_array_element(
        &self,
        context: &'context IRGenerator,
        set_array_element: &SetArrayElementExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_variable(
        &self,
        context: &'context IRGenerator,
        variable: &VariableExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_self_ident(
        &self,
        context: &'context IRGenerator,
        self_ident: &SelfIdentExpr,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;

    fn visit_break(&self) -> Result<T, LangError>;
    fn visit_assert(
        &self,
        context: &'context IRGenerator,
        condition: &AssertStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum(
        &self,
        context: &'context IRGenerator,
        enum_stmt: &EnumStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl(
        &self,
        context: &'context IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl_trait(
        &self,
        context: &'context IRGenerator,
        impl_trait: &ImplTraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_block(
        &self,
        context: &'context IRGenerator,
        block: &BlockStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_struct(
        &self,
        context: &'context IRGenerator,
        block: &StructStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_expression(
        &self,
        context: &'context IRGenerator,
        block: &ExpressionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait(
        &self,
        context: &'context IRGenerator,
        block: &TraitStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait_function(
        &self,
        context: &'context IRGenerator,
        block: &TraitFunctionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_function(
        &self,
        context: &'context IRGenerator,
        block: &FunctionStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_if(
        &self,
        context: &'context IRGenerator,
        block: &IfStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_print(
        &self,
        context: &'context IRGenerator,
        block: &PrintStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_return(
        &self,
        context: &'context IRGenerator,
        block: &ReturnStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_var(
        &self,
        context: &'context IRGenerator,
        block: &VarStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_while(
        &self,
        context: &'context IRGenerator,
        block: &WhileStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_import(
        &self,
        context: &'context IRGenerator,
        import_stmt: &ImportStmt,
        arena: &'context mut Arena<AnyValueType<'context>>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
}

pub fn visit_expr<'context, T, V: Visitor<'context, T>>(
    visitor: &V,
    context: &'context IRGenerator,
    expr: &Expr,
    arena: &'context mut Arena<AnyValueType<'context>>,
    env: &mut Environment,
) -> Result<T, LangError> {
    match expr {
        Expr::Assign(ref assign_expr) => {
            Ok(visitor.visit_assign(context, &*assign_expr, &mut *arena, &mut *env)?)
        }
        Expr::Binary(ref binary_expr) => {
            Ok(visitor.visit_binary(context, &*binary_expr, &mut *arena, &mut *env)?)
        }
        Expr::Call(ref call_expr) => {
            Ok(visitor.visit_call(context, &*call_expr, &mut *arena, &mut *env)?)
        }
        Expr::Get(ref get_expr) => {
            Ok(visitor.visit_get(context, &*get_expr, &mut *arena, &mut *env)?)
        }
        Expr::EnumPath(ref enum_path_expr) => {
            Ok(visitor.visit_enum_path(context, &*enum_path_expr, &mut *arena, &mut *env)?)
        }
        Expr::Grouping(ref grouping_expr) => {
            Ok(visitor.visit_grouping(context, &*grouping_expr, &mut *arena, &mut *env)?)
        }
        Expr::Literal(ref literal_expr) => {
            Ok(visitor.visit_literal(context, &*literal_expr, &mut *arena, &mut *env)?)
        }
        Expr::Logical(ref logical_expr) => {
            Ok(visitor.visit_logical(context, &*logical_expr, &mut *arena, &mut *env)?)
        }
        Expr::Set(ref set_expr) => {
            Ok(visitor.visit_set(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::Unary(ref set_expr) => {
            Ok(visitor.visit_unary(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::Array(ref set_expr) => {
            Ok(visitor.visit_array(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::Index(ref set_expr) => {
            Ok(visitor.visit_index(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::SetArrayElement(ref set_expr) => {
            Ok(visitor.visit_set_array_element(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::Variable(ref set_expr) => {
            Ok(visitor.visit_variable(context, &*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::SelfIdent(ref set_expr) => {
            Ok(visitor.visit_self_ident(context, &*set_expr, &mut *arena, &mut *env)?)
        }
    }
}

pub fn visit_stmt<'context, T, V: Visitor<'context, T>>(
    visitor: &V,
    context: &'context IRGenerator,
    stmt: &Stmt,
    arena: &'context mut Arena<AnyValueType<'context>>,
    env: &mut Environment,
) -> Result<T, LangError> {
    match stmt {
        Stmt::Break => Ok(visitor.visit_break()?),
        Stmt::Assert(ref assert_stmt) => {
            Ok(visitor.visit_assert(context, &*assert_stmt, arena, env)?)
        }
        Stmt::Enum(ref enum_stmt) => {
            Ok(visitor.visit_enum(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Impl(ref enum_stmt) => {
            Ok(visitor.visit_impl(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::ImplTrait(ref enum_stmt) => {
            Ok(visitor.visit_impl_trait(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Block(ref block_stmt) => {
            Ok(visitor.visit_block(context, &*block_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Struct(ref enum_stmt) => {
            Ok(visitor.visit_struct(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Expression(ref enum_stmt) => {
            Ok(visitor.visit_expression(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Trait(ref enum_stmt) => {
            Ok(visitor.visit_trait(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::TraitFunction(ref enum_stmt) => {
            Ok(visitor.visit_trait_function(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Function(ref enum_stmt) => {
            Ok(visitor.visit_function(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::If(ref enum_stmt) => {
            Ok(visitor.visit_if(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Print(ref enum_stmt) => {
            Ok(visitor.visit_print(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Return(ref enum_stmt) => {
            Ok(visitor.visit_return(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Var(ref enum_stmt) => {
            Ok(visitor.visit_var(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::While(ref enum_stmt) => {
            Ok(visitor.visit_while(context, &*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Import(ref import_stmt) => {
            Ok(visitor.visit_import(context, &*import_stmt, &mut *arena, &mut *env)?)
        }
    }
}
