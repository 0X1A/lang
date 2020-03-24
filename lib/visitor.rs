use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::env::*;
use crate::error::LangError;
use crate::interpreterjit::*;
use crate::mem::*;
use crate::value::TypedValue;
use inkwell::context::Context;

// We _have_ to return a concrete definition of Result here since we can't have
// bounds on an associated type in order to use the error propogation operator
pub trait VisitorMut<T>: Sized {
    fn visit_expr_mut(&mut self, expr: &Expr) -> Result<T, LangError> {
        Ok(visit_expr_mut(self, expr)?)
    }
    fn visit_stmt_mut(&mut self, stmt: &Stmt) -> Result<T, LangError> {
        Ok(visit_stmt_mut(self, stmt)?)
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> Result<T, LangError>;
    fn visit_binary(&mut self, binary: &BinaryExpr) -> Result<T, LangError>;
    fn visit_call(&mut self, call: &CallExpr) -> Result<T, LangError>;
    fn visit_get(&mut self, get: &GetExpr) -> Result<T, LangError>;
    fn visit_enum_path(&mut self, enum_path: &EnumPathExpr) -> Result<T, LangError>;
    fn visit_grouping(&mut self, grouping: &GroupingExpr) -> Result<T, LangError>;
    fn visit_literal(&mut self, literal: &LiteralExpr) -> Result<T, LangError>;
    fn visit_logical(&mut self, logical: &LogicalExpr) -> Result<T, LangError>;
    fn visit_set(&mut self, set: &SetExpr) -> Result<T, LangError>;
    fn visit_unary(&mut self, unary: &UnaryExpr) -> Result<T, LangError>;
    fn visit_array(&mut self, array: &ArrayExpr) -> Result<T, LangError>;
    fn visit_index(&mut self, index: &IndexExpr) -> Result<T, LangError>;
    fn visit_set_array_element(
        &mut self,
        set_array_element: &SetArrayElementExpr,
    ) -> Result<T, LangError>;
    fn visit_variable(&mut self, variable: &VariableExpr) -> Result<T, LangError>;
    fn visit_self_ident(&mut self, self_ident: &SelfIdentExpr) -> Result<T, LangError>;

    fn visit_break(&mut self) -> Result<T, LangError>;
    fn visit_assert(&mut self, assert_stmt: &AssertStmt) -> Result<T, LangError>;
    fn visit_enum(&mut self, enum_stmt: &EnumStmt) -> Result<T, LangError>;
    fn visit_impl(&mut self, impl_stmt: &ImplStmt) -> Result<T, LangError>;
    fn visit_impl_trait(&mut self, impl_trait: &ImplTraitStmt) -> Result<T, LangError>;
    fn visit_block(&mut self, block: &BlockStmt) -> Result<T, LangError>;
    fn visit_struct(&mut self, block: &StructStmt) -> Result<T, LangError>;
    fn visit_expression(&mut self, block: &ExpressionStmt) -> Result<T, LangError>;
    fn visit_trait(&mut self, block: &TraitStmt) -> Result<T, LangError>;
    fn visit_trait_function(&mut self, block: &TraitFunctionStmt) -> Result<T, LangError>;
    fn visit_function(&mut self, block: &FunctionStmt) -> Result<T, LangError>;
    fn visit_if(&mut self, block: &IfStmt) -> Result<T, LangError>;
    fn visit_print(&mut self, block: &PrintStmt) -> Result<T, LangError>;
    fn visit_return(&mut self, block: &ReturnStmt) -> Result<T, LangError>;
    fn visit_var(&mut self, block: &VarStmt) -> Result<T, LangError>;
    fn visit_while(&mut self, block: &WhileStmt) -> Result<T, LangError>;
    fn visit_import(&mut self, import_stmt: &ImportStmt) -> Result<T, LangError>;
}

pub fn visit_expr_mut<T, V: VisitorMut<T>>(visitor: &mut V, expr: &Expr) -> Result<T, LangError> {
    match expr {
        Expr::Assign(ref assign_expr) => Ok(visitor.visit_assign(&*assign_expr)?),
        Expr::Binary(ref binary_expr) => Ok(visitor.visit_binary(&*binary_expr)?),
        Expr::Call(ref call_expr) => Ok(visitor.visit_call(&*call_expr)?),
        Expr::Get(ref get_expr) => Ok(visitor.visit_get(&*get_expr)?),
        Expr::EnumPath(ref enum_path_expr) => Ok(visitor.visit_enum_path(&*enum_path_expr)?),
        Expr::Grouping(ref grouping_expr) => Ok(visitor.visit_grouping(&*grouping_expr)?),
        Expr::Literal(ref literal_expr) => Ok(visitor.visit_literal(&*literal_expr)?),
        Expr::Logical(ref logical_expr) => Ok(visitor.visit_logical(&*logical_expr)?),
        Expr::Set(ref set_expr) => Ok(visitor.visit_set(&*set_expr)?),
        Expr::Unary(ref set_expr) => Ok(visitor.visit_unary(&*set_expr)?),
        Expr::Array(ref set_expr) => Ok(visitor.visit_array(&*set_expr)?),
        Expr::Index(ref set_expr) => Ok(visitor.visit_index(&*set_expr)?),
        Expr::SetArrayElement(ref set_expr) => Ok(visitor.visit_set_array_element(&*set_expr)?),
        Expr::Variable(ref set_expr) => Ok(visitor.visit_variable(&*set_expr)?),
        Expr::SelfIdent(ref set_expr) => Ok(visitor.visit_self_ident(&*set_expr)?),
    }
}

pub fn visit_stmt_mut<T, V: VisitorMut<T>>(visitor: &mut V, stmt: &Stmt) -> Result<T, LangError> {
    match stmt {
        Stmt::Break => Ok(visitor.visit_break()?),
        Stmt::Assert(ref assert_stmt) => Ok(visitor.visit_assert(&*assert_stmt)?),
        Stmt::Enum(ref enum_stmt) => Ok(visitor.visit_enum(&*enum_stmt)?),
        Stmt::Impl(ref enum_stmt) => Ok(visitor.visit_impl(&*enum_stmt)?),
        Stmt::ImplTrait(ref enum_stmt) => Ok(visitor.visit_impl_trait(&*enum_stmt)?),
        Stmt::Block(ref block_stmt) => Ok(visitor.visit_block(&*block_stmt)?),
        Stmt::Struct(ref enum_stmt) => Ok(visitor.visit_struct(&*enum_stmt)?),
        Stmt::Expression(ref enum_stmt) => Ok(visitor.visit_expression(&*enum_stmt)?),
        Stmt::Trait(ref enum_stmt) => Ok(visitor.visit_trait(&*enum_stmt)?),
        Stmt::TraitFunction(ref enum_stmt) => Ok(visitor.visit_trait_function(&*enum_stmt)?),
        Stmt::Function(ref enum_stmt) => Ok(visitor.visit_function(&*enum_stmt)?),
        Stmt::If(ref enum_stmt) => Ok(visitor.visit_if(&*enum_stmt)?),
        Stmt::Print(ref enum_stmt) => Ok(visitor.visit_print(&*enum_stmt)?),
        Stmt::Return(ref enum_stmt) => Ok(visitor.visit_return(&*enum_stmt)?),
        Stmt::Var(ref enum_stmt) => Ok(visitor.visit_var(&*enum_stmt)?),
        Stmt::While(ref enum_stmt) => Ok(visitor.visit_while(&*enum_stmt)?),
        Stmt::Import(ref import_stmt) => Ok(visitor.visit_import(&*import_stmt)?),
    }
}

// We _have_ to return a concrete definition of Result here since we can't have
// bounds on an associated type in order to use the error propogation operator
pub trait Visitor<T, ArenaType>: Sized {
    fn visit_expr(
        &self,
        context: &IRGenerator,
        expr: &Expr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_expr(self, context, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        context: &IRGenerator,
        stmt: &Stmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_stmt(self, context, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        context: &IRGenerator,
        assign: &AssignExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_binary(
        &self,
        context: &IRGenerator,
        binary: &BinaryExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_call(
        &self,
        context: &IRGenerator,
        call: &CallExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_get(
        &self,
        context: &IRGenerator,
        get: &GetExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum_path(
        &self,
        context: &IRGenerator,
        enum_path: &EnumPathExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_grouping(
        &self,
        context: &IRGenerator,
        grouping: &GroupingExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_literal(
        &self,
        context: &IRGenerator,
        literal: &LiteralExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_logical(
        &self,
        context: &IRGenerator,
        logical: &LogicalExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set(
        &self,
        context: &IRGenerator,
        set: &SetExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_unary(
        &self,
        context: &IRGenerator,
        unary: &UnaryExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_array(
        &self,
        context: &IRGenerator,
        array: &ArrayExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_index(
        &self,
        context: &IRGenerator,
        index: &IndexExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set_array_element(
        &self,
        context: &IRGenerator,
        set_array_element: &SetArrayElementExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_variable(
        &self,
        context: &IRGenerator,
        variable: &VariableExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_self_ident(
        &self,
        context: &IRGenerator,
        self_ident: &SelfIdentExpr,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;

    fn visit_break(&self) -> Result<T, LangError>;
    fn visit_assert(
        &self,
        context: &IRGenerator,
        condition: &AssertStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum(
        &self,
        context: &IRGenerator,
        enum_stmt: &EnumStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl(
        &self,
        context: &IRGenerator,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl_trait(
        &self,
        context: &IRGenerator,
        impl_trait: &ImplTraitStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_block(
        &self,
        context: &IRGenerator,
        block: &BlockStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_struct(
        &self,
        context: &IRGenerator,
        block: &StructStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_expression(
        &self,
        context: &IRGenerator,
        block: &ExpressionStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait(
        &self,
        context: &IRGenerator,
        block: &TraitStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait_function(
        &self,
        context: &IRGenerator,
        block: &TraitFunctionStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_function(
        &self,
        context: &IRGenerator,
        block: &FunctionStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_if(
        &self,
        context: &IRGenerator,
        block: &IfStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_print(
        &self,
        context: &IRGenerator,
        block: &PrintStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_return(
        &self,
        context: &IRGenerator,
        block: &ReturnStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_var(
        &self,
        context: &IRGenerator,
        block: &VarStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_while(
        &self,
        context: &IRGenerator,
        block: &WhileStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_import(
        &self,
        context: &IRGenerator,
        import_stmt: &ImportStmt,
        arena: &mut Arena<ArenaType>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
}

pub fn visit_expr<T, ArenaType, V: Visitor<T, ArenaType>>(
    visitor: &V,
    context: &IRGenerator,
    expr: &Expr,
    arena: &mut Arena<ArenaType>,
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

pub fn visit_stmt<T, ArenaType, V: Visitor<T, ArenaType>>(
    visitor: &V,
    context: &IRGenerator,
    stmt: &Stmt,
    arena: &mut Arena<ArenaType>,
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
