use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::error::LangErrorTwo;

// We _have_ to return a concrete definition of Result here since we can't have
// bounds on an associated type in order to use the error propogation operator
pub trait Visitor: Sized {
    fn visit_expr(&mut self, expr: &Expr) -> Result<(), LangErrorTwo> {
        Ok(noop_expr(self, expr)?)
    }
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), LangErrorTwo> {
        Ok(noop_stmt(self, stmt)?)
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> Result<(), LangErrorTwo>;
    fn visit_binary(&mut self, binary: &BinaryExpr) -> Result<(), LangErrorTwo>;
    fn visit_call(&mut self, call: &CallExpr) -> Result<(), LangErrorTwo>;
    fn visit_get(&mut self, get: &GetExpr) -> Result<(), LangErrorTwo>;
    fn visit_enum_path(&mut self, enum_path: &EnumPathExpr) -> Result<(), LangErrorTwo>;
    fn visit_grouping(&mut self, grouping: &GroupingExpr) -> Result<(), LangErrorTwo>;
    fn visit_literal(&mut self, literal: &LiteralExpr) -> Result<(), LangErrorTwo>;
    fn visit_logical(&mut self, logical: &LogicalExpr) -> Result<(), LangErrorTwo>;
    fn visit_set(&mut self, set: &SetExpr) -> Result<(), LangErrorTwo>;
    fn visit_unary(&mut self, unary: &UnaryExpr) -> Result<(), LangErrorTwo>;
    fn visit_array(&mut self, array: &ArrayExpr) -> Result<(), LangErrorTwo>;
    fn visit_index(&mut self, index: &IndexExpr) -> Result<(), LangErrorTwo>;
    fn visit_set_array_element(
        &mut self,
        set_array_element: &SetArrayElementExpr,
    ) -> Result<(), LangErrorTwo>;
    fn visit_variable(&mut self, variable: &VariableExpr) -> Result<(), LangErrorTwo>;
    fn visit_self_ident(&mut self, self_ident: &SelfIdentExpr) -> Result<(), LangErrorTwo>;

    fn visit_break(&mut self) -> Result<(), LangErrorTwo>;
    fn visit_enum(&mut self, enum_stmt: &EnumStmt) -> Result<(), LangErrorTwo>;
    fn visit_impl(&mut self, impl_stmt: &ImplStmt) -> Result<(), LangErrorTwo>;
    fn visit_impl_trait(&mut self, impl_trait: &ImplTraitStmt) -> Result<(), LangErrorTwo>;
    fn visit_block(&mut self, block: &BlockStmt) -> Result<(), LangErrorTwo>;
    fn visit_struct(&mut self, block: &StructStmt) -> Result<(), LangErrorTwo>;
    fn visit_expression(&mut self, block: &ExpressionStmt) -> Result<(), LangErrorTwo>;
    fn visit_trait(&mut self, block: &TraitStmt) -> Result<(), LangErrorTwo>;
    fn visit_trait_function(&mut self, block: &TraitFunctionStmt) -> Result<(), LangErrorTwo>;
    fn visit_function(&mut self, block: &FunctionStmt) -> Result<(), LangErrorTwo>;
    fn visit_if(&mut self, block: &IfStmt) -> Result<(), LangErrorTwo>;
    fn visit_print(&mut self, block: &PrintStmt) -> Result<(), LangErrorTwo>;
    fn visit_return(&mut self, block: &ReturnStmt) -> Result<(), LangErrorTwo>;
    fn visit_var(&mut self, block: &VarStmt) -> Result<(), LangErrorTwo>;
    fn visit_while(&mut self, block: &WhileStmt) -> Result<(), LangErrorTwo>;
}

pub fn noop_expr<V: Visitor>(visitor: &mut V, expr: &Expr) -> Result<(), LangErrorTwo> {
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

pub fn noop_stmt<V: Visitor>(visitor: &mut V, stmt: &Stmt) -> Result<(), LangErrorTwo> {
    match stmt {
        Stmt::Break => Ok(visitor.visit_break()?),
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
    }
}
