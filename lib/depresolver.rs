use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::error::*;
use crate::lang::*;
use crate::syntax::parser::Parser;
use crate::syntax::scanner::*;
use crate::syntax::token::*;
use crate::visitor::*;

pub struct DependencyResolver {}

impl DependencyResolver {
    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<Vec<Stmt>, LangError> {
        let mut statements: Vec<Stmt> = Vec::new();
        for stmt in stmts {
            if let Stmt::Import(_) = stmt {
                let mut imported = self.visit_stmt_mut(stmt)?;
                statements.append(&mut imported);
            }
        }
        Ok(statements)
    }
}

impl Default for DependencyResolver {
    fn default() -> DependencyResolver {
        DependencyResolver {}
    }
}

impl VisitorMut<Vec<Stmt>> for DependencyResolver {
    fn visit_expr_mut(&mut self, expr: &Expr) -> Result<Vec<Stmt>, LangError> {
        Ok(visit_expr_mut(self, expr)?)
    }
    fn visit_stmt_mut(&mut self, stmt: &Stmt) -> Result<Vec<Stmt>, LangError> {
        Ok(visit_stmt_mut(self, stmt)?)
    }

    fn visit_assign(&mut self, _: &AssignExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_binary(&mut self, _: &BinaryExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_call(&mut self, _: &CallExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_get(&mut self, _: &GetExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_enum_path(&mut self, _: &EnumPathExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_grouping(&mut self, _: &GroupingExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_literal(&mut self, _: &LiteralExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_logical(&mut self, _: &LogicalExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_set(&mut self, _: &SetExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_unary(&mut self, _: &UnaryExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_array(&mut self, _: &ArrayExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_index(&mut self, _: &IndexExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_set_array_element(&mut self, _: &SetArrayElementExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_variable(&mut self, _: &VariableExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_self_ident(&mut self, _: &SelfIdentExpr) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }

    // stmt
    fn visit_break(&mut self) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_assert(&mut self, _: &AssertStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_enum(&mut self, _: &EnumStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_impl(&mut self, _: &ImplStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_impl_trait(&mut self, _: &ImplTraitStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_block(&mut self, _: &BlockStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_struct(&mut self, _: &StructStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_expression(&mut self, _: &ExpressionStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_trait(&mut self, _: &TraitStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_trait_function(&mut self, _: &TraitFunctionStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_function(&mut self, _: &FunctionStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_if(&mut self, _: &IfStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_print(&mut self, _: &PrintStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_return(&mut self, _: &ReturnStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_var(&mut self, _: &VarStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_while(&mut self, _: &WhileStmt) -> Result<Vec<Stmt>, LangError> {
        unreachable!()
    }
    fn visit_import(&mut self, import_stmt: &ImportStmt) -> Result<Vec<Stmt>, LangError> {
        // TODO!!!
        // This is naive as fuck but works as a PoC that all our stuff can work
        // We just need to slap a dep_graph on this bad boy and we're good to go
        let contents = Lang::read_file(&import_stmt.module_path)?;
        let mut scanner = Scanner::new(&contents);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser = Parser::new(&contents, tokens);
        let mut statements = parser.parse()?;
        let mut dep_resolver = DependencyResolver::default();
        let mut import_statements = dep_resolver.resolve(&statements)?;
        import_statements.append(&mut statements);
        debug!("visit import: {:?}", import_statements);
        Ok(import_statements)
    }
}
