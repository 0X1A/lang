use crate::ast::expr::*;
use crate::error::*;
use crate::token::{TokenType, TypeAnnotation};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub enum Stmt {
    Break,
    Enum(Box<EnumStmt>),
    Impl(Box<ImplStmt>),
    ImplTrait(Box<ImplTraitStmt>),
    Block(Box<BlockStmt>),
    Struct(Box<StructStmt>),
    Expression(Box<ExpressionStmt>),
    Trait(Box<TraitStmt>),
    TraitFunction(Box<TraitFunctionStmt>),
    Function(Box<FunctionStmt>),
    If(Box<IfStmt>),
    Print(Box<PrintStmt>),
    Return(Box<ReturnStmt>),
    Var(Box<VarStmt>),
    While(Box<WhileStmt>),
}

#[derive(Clone, Debug)]
pub struct VariableData {
    pub identifier: String,
    pub type_annotation: TypeAnnotation,
}

impl VariableData {
    pub fn new(identifier: String, type_annotation: TypeAnnotation) -> VariableData {
        VariableData {
            identifier,
            type_annotation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumItem {
    pub identifier: String,
    pub initializer: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct EnumStmt {
    pub name: String,
    pub item_list: Vec<EnumItem>,
}

#[derive(Clone, Debug)]
pub struct ImplStmt {
    pub name: String,
    pub fn_declarations: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct ImplTraitStmt {
    pub trait_name: String,
    pub impl_name: String,
    pub fn_declarations: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct StructStmt {
    pub fields: Vec<VariableData>,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ExpressionStmt {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct TraitStmt {
    pub name: String,
    pub trait_fn_declarations: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct TraitFunctionStmt {
    pub name: String,
    pub return_type: TokenType,
    pub params: Vec<VariableData>,
}

#[derive(Clone, Debug)]
pub struct FunctionStmt {
    pub name: String,
    pub return_type: TokenType,
    pub params: Vec<VariableData>,
    pub body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct IfStmt {
    pub condition: Expr,
    pub else_branch: Option<Stmt>,
    pub then_branch: Stmt,
}

#[derive(Clone, Debug)]
pub struct PrintStmt {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct ReturnStmt {
    pub keyword: String,
    pub value: Expr,
}

#[derive(Clone, Debug)]
pub struct VarStmt {
    pub initializer: Option<Expr>,
    pub type_annotation: TokenType,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct WhileStmt {
    pub body: Stmt,
    pub condition: Expr,
}

impl TryInto<ImplStmt> for Stmt {
    type Error = LangError;
    fn try_into(self) -> Result<ImplStmt, Self::Error> {
        match self {
            Stmt::Impl(impl_stmt) => Ok(*impl_stmt.clone()),
            _ => Err(LangErrorType::new_iie_error("".to_string())),
        }
    }
}

impl<'a> TryInto<&'a ImplStmt> for &'a Stmt {
    type Error = LangError;
    fn try_into(self) -> Result<&'a ImplStmt, Self::Error> {
        match self {
            Stmt::Impl(impl_stmt) => Ok(impl_stmt),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectStmt("impl".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a ImplTraitStmt> for &'a Stmt {
    type Error = LangError;
    fn try_into(self) -> Result<&'a ImplTraitStmt, Self::Error> {
        match self {
            Stmt::ImplTrait(impl_trait_stmt) => Ok(impl_trait_stmt),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectStmt("impl trait".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a TraitStmt> for &'a Stmt {
    type Error = LangError;
    fn try_into(self) -> Result<&'a TraitStmt, Self::Error> {
        match self {
            Stmt::Trait(trait_stmt) => Ok(trait_stmt),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectStmt("trait".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a StructStmt> for &'a Stmt {
    type Error = LangError;
    fn try_into(self) -> Result<&'a StructStmt, Self::Error> {
        match self {
            Stmt::Struct(struct_stmt) => Ok(struct_stmt),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectStmt("struct".to_string()),
            ))),
        }
    }
}
