use ast::expr::*;
use ast::stmt::*;

pub trait Visitor<T> {
    type Value;
    fn visit(&mut self, expr: &T) -> Self::Value;
}

pub trait VisitorTwo: Sized {
    fn visit_expr(&mut self, expr: &Expr) {
        noop_expr(self, expr);
    }
    fn visit_stmt(&mut self, stmt: &Stmt) {
        noop_stmt(self, stmt);
    }

    fn visit_assign(&mut self, assign: &AssignExpr);
    fn visit_binary(&mut self, binary: &BinaryExpr);
    fn visit_call(&mut self, call: &CallExpr);
    fn visit_get(&mut self, get: &GetExpr);
    fn visit_enum_path(&mut self, enum_path: &EnumPathExpr);
    fn visit_grouping(&mut self, grouping: &GroupingExpr);
    fn visit_literal(&mut self, literal: &LiteralExpr);
    fn visit_logical(&mut self, logical: &LogicalExpr);
    fn visit_set(&mut self, set: &SetExpr);
    fn visit_unary(&mut self, unary: &UnaryExpr);
    fn visit_array(&mut self, array: &ArrayExpr);
    fn visit_index(&mut self, index: &IndexExpr);
    fn visit_set_array_element(&mut self, set_array_element: &SetArrayElementExpr);
    fn visit_variable(&mut self, variable: &VariableExpr);
    fn visit_self_ident(&mut self, self_ident: &SelfIdentExpr);

    fn visit_break(&mut self);
    fn visit_enum(&mut self, enum_stmt: &EnumStmt);
    fn visit_impl(&mut self, impl_stmt: &ImplStmt);
    fn visit_impl_trait(&mut self, impl_trait: &ImplTraitStmt);
    fn visit_block(&mut self, block: &BlockStmt);
    fn visit_struct(&mut self, block: &StructStmt);
    fn visit_expression(&mut self, block: &ExpressionStmt);
    fn visit_trait(&mut self, block: &TraitStmt);
    fn visit_trait_function(&mut self, block: &TraitFunctionStmt);
    fn visit_function(&mut self, block: &FunctionStmt);
    fn visit_if(&mut self, block: &IfStmt);
    fn visit_print(&mut self, block: &PrintStmt);
    fn visit_return(&mut self, block: &ReturnStmt);
    fn visit_var(&mut self, block: &VarStmt);
    fn visit_while(&mut self, block: &WhileStmt);
}

pub fn noop_expr<V: VisitorTwo>(visitor: &mut V, expr: &Expr) {
    match expr {
        Expr::Assign(ref assign_expr) => visitor.visit_assign(&*assign_expr),
        Expr::Binary(ref binary_expr) => visitor.visit_binary(&*binary_expr),
        Expr::Call(ref call_expr) => visitor.visit_call(&*call_expr),
        Expr::Get(ref get_expr) => visitor.visit_get(&*get_expr),
        Expr::EnumPath(ref enum_path_expr) => visitor.visit_enum_path(&*enum_path_expr),
        Expr::Grouping(ref grouping_expr) => visitor.visit_grouping(&*grouping_expr),
        Expr::Literal(ref literal_expr) => visitor.visit_literal(&*literal_expr),
        Expr::Logical(ref logical_expr) => visitor.visit_logical(&*logical_expr),
        Expr::Set(ref set_expr) => visitor.visit_set(&*set_expr),
        Expr::Unary(ref set_expr) => visitor.visit_unary(&*set_expr),
        Expr::Array(ref set_expr) => visitor.visit_array(&*set_expr),
        Expr::Index(ref set_expr) => visitor.visit_index(&*set_expr),
        Expr::SetArrayElement(ref set_expr) => visitor.visit_set_array_element(&*set_expr),
        Expr::Variable(ref set_expr) => visitor.visit_variable(&*set_expr),
        Expr::SelfIdent(ref set_expr) => visitor.visit_self_ident(&*set_expr),
    }
}

pub fn noop_stmt<V: VisitorTwo>(visitor: &mut V, stmt: &Stmt) {
    match stmt {
        Stmt::Break => visitor.visit_break(),
        Stmt::Enum(ref enum_stmt) => visitor.visit_enum(&*enum_stmt),
        Stmt::Impl(ref enum_stmt) => visitor.visit_impl(&*enum_stmt),
        Stmt::ImplTrait(ref enum_stmt) => visitor.visit_impl_trait(&*enum_stmt),
        Stmt::Block(ref block_stmt) => visitor.visit_block(&*block_stmt),
        Stmt::Struct(ref enum_stmt) => visitor.visit_struct(&*enum_stmt),
        Stmt::Expression(ref enum_stmt) => visitor.visit_expression(&*enum_stmt),
        Stmt::Trait(ref enum_stmt) => visitor.visit_trait(&*enum_stmt),
        Stmt::TraitFunction(ref enum_stmt) => visitor.visit_trait_function(&*enum_stmt),
        Stmt::Function(ref enum_stmt) => visitor.visit_function(&*enum_stmt),
        Stmt::If(ref enum_stmt) => visitor.visit_if(&*enum_stmt),
        Stmt::Print(ref enum_stmt) => visitor.visit_print(&*enum_stmt),
        Stmt::Return(ref enum_stmt) => visitor.visit_return(&*enum_stmt),
        Stmt::Var(ref enum_stmt) => visitor.visit_var(&*enum_stmt),
        Stmt::While(ref enum_stmt) => visitor.visit_while(&*enum_stmt),
    }
}
