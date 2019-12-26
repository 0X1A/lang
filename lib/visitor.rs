use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::env::*;
use crate::error::LangError;
use crate::mem::*;
use crate::value::TypedValue;

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
pub trait Visitor<T>: Sized {
    fn visit_expr(
        &self,
        expr: &Expr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_expr(self, expr, arena, env)?)
    }
    fn visit_stmt(
        &self,
        stmt: &Stmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError> {
        Ok(visit_stmt(self, stmt, arena, env)?)
    }

    fn visit_assign(
        &self,
        assign: &AssignExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_binary(
        &self,
        binary: &BinaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_call(
        &self,
        call: &CallExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_get(
        &self,
        get: &GetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum_path(
        &self,
        enum_path: &EnumPathExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_grouping(
        &self,
        grouping: &GroupingExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_literal(
        &self,
        literal: &LiteralExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_logical(
        &self,
        logical: &LogicalExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set(
        &self,
        set: &SetExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_unary(
        &self,
        unary: &UnaryExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_array(
        &self,
        array: &ArrayExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_index(
        &self,
        index: &IndexExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_set_array_element(
        &self,
        set_array_element: &SetArrayElementExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_variable(
        &self,
        variable: &VariableExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_self_ident(
        &self,
        self_ident: &SelfIdentExpr,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;

    fn visit_break(&self) -> Result<T, LangError>;
    fn visit_assert(
        &self,
        condition: &AssertStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_enum(
        &self,
        enum_stmt: &EnumStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl(
        &self,
        impl_stmt: &ImplStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_impl_trait(
        &self,
        impl_trait: &ImplTraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_block(
        &self,
        block: &BlockStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_struct(
        &self,
        block: &StructStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_expression(
        &self,
        block: &ExpressionStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait(
        &self,
        block: &TraitStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_trait_function(
        &self,
        block: &TraitFunctionStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_function(
        &self,
        block: &FunctionStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_if(
        &self,
        block: &IfStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_print(
        &self,
        block: &PrintStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_return(
        &self,
        block: &ReturnStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_var(
        &self,
        block: &VarStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_while(
        &self,
        block: &WhileStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
    fn visit_import(
        &self,
        import_stmt: &ImportStmt,
        arena: &mut Arena<TypedValue>,
        env: &mut Environment,
    ) -> Result<T, LangError>;
}

pub fn visit_expr<T, V: Visitor<T>>(
    visitor: &V,
    expr: &Expr,
    arena: &mut Arena<TypedValue>,
    env: &mut Environment,
) -> Result<T, LangError> {
    match expr {
        Expr::Assign(ref assign_expr) => {
            Ok(visitor.visit_assign(&*assign_expr, &mut *arena, &mut *env)?)
        }
        Expr::Binary(ref binary_expr) => {
            Ok(visitor.visit_binary(&*binary_expr, &mut *arena, &mut *env)?)
        }
        Expr::Call(ref call_expr) => Ok(visitor.visit_call(&*call_expr, &mut *arena, &mut *env)?),
        Expr::Get(ref get_expr) => Ok(visitor.visit_get(&*get_expr, &mut *arena, &mut *env)?),
        Expr::EnumPath(ref enum_path_expr) => {
            Ok(visitor.visit_enum_path(&*enum_path_expr, &mut *arena, &mut *env)?)
        }
        Expr::Grouping(ref grouping_expr) => {
            Ok(visitor.visit_grouping(&*grouping_expr, &mut *arena, &mut *env)?)
        }
        Expr::Literal(ref literal_expr) => {
            Ok(visitor.visit_literal(&*literal_expr, &mut *arena, &mut *env)?)
        }
        Expr::Logical(ref logical_expr) => {
            Ok(visitor.visit_logical(&*logical_expr, &mut *arena, &mut *env)?)
        }
        Expr::Set(ref set_expr) => Ok(visitor.visit_set(&*set_expr, &mut *arena, &mut *env)?),
        Expr::Unary(ref set_expr) => Ok(visitor.visit_unary(&*set_expr, &mut *arena, &mut *env)?),
        Expr::Array(ref set_expr) => Ok(visitor.visit_array(&*set_expr, &mut *arena, &mut *env)?),
        Expr::Index(ref set_expr) => Ok(visitor.visit_index(&*set_expr, &mut *arena, &mut *env)?),
        Expr::SetArrayElement(ref set_expr) => {
            Ok(visitor.visit_set_array_element(&*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::Variable(ref set_expr) => {
            Ok(visitor.visit_variable(&*set_expr, &mut *arena, &mut *env)?)
        }
        Expr::SelfIdent(ref set_expr) => {
            Ok(visitor.visit_self_ident(&*set_expr, &mut *arena, &mut *env)?)
        }
    }
}

pub fn visit_stmt<T, V: Visitor<T>>(
    visitor: &V,
    stmt: &Stmt,
    arena: &mut Arena<TypedValue>,
    env: &mut Environment,
) -> Result<T, LangError> {
    match stmt {
        Stmt::Break => Ok(visitor.visit_break()?),
        Stmt::Assert(ref assert_stmt) => Ok(visitor.visit_assert(&*assert_stmt, arena, env)?),
        Stmt::Enum(ref enum_stmt) => Ok(visitor.visit_enum(&*enum_stmt, &mut *arena, &mut *env)?),
        Stmt::Impl(ref enum_stmt) => Ok(visitor.visit_impl(&*enum_stmt, &mut *arena, &mut *env)?),
        Stmt::ImplTrait(ref enum_stmt) => {
            Ok(visitor.visit_impl_trait(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Block(ref block_stmt) => {
            Ok(visitor.visit_block(&*block_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Struct(ref enum_stmt) => {
            Ok(visitor.visit_struct(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Expression(ref enum_stmt) => {
            Ok(visitor.visit_expression(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Trait(ref enum_stmt) => {
            Ok(visitor.visit_trait(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::TraitFunction(ref enum_stmt) => {
            Ok(visitor.visit_trait_function(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Function(ref enum_stmt) => {
            Ok(visitor.visit_function(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::If(ref enum_stmt) => Ok(visitor.visit_if(&*enum_stmt, &mut *arena, &mut *env)?),
        Stmt::Print(ref enum_stmt) => {
            Ok(visitor.visit_print(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Return(ref enum_stmt) => {
            Ok(visitor.visit_return(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Var(ref enum_stmt) => Ok(visitor.visit_var(&*enum_stmt, &mut *arena, &mut *env)?),
        Stmt::While(ref enum_stmt) => {
            Ok(visitor.visit_while(&*enum_stmt, &mut *arena, &mut *env)?)
        }
        Stmt::Import(ref import_stmt) => {
            Ok(visitor.visit_import(&*import_stmt, &mut *arena, &mut *env)?)
        }
    }
}
