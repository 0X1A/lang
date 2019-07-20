use crate::error::*;
use crate::token::Token;
use crate::value::TypedValue;
use std::convert::Into;
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr {
    Assign(Box<AssignExpr>),
    Binary(Box<BinaryExpr>),
    Call(Box<CallExpr>),
    Get(Box<GetExpr>),
    EnumPath(Box<EnumPathExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Logical(Box<LogicalExpr>),
    Set(Box<SetExpr>),
    Unary(Box<UnaryExpr>),
    Array(Box<ArrayExpr>),
    Index(Box<IndexExpr>),
    SetArrayElement(Box<SetArrayElementExpr>),
    Variable(Box<VariableExpr>),
    SelfIdent(Box<SelfIdentExpr>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfIdentExpr {
    pub keyword: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetArrayElementExpr {
    pub index: Expr,
    pub name: Token,
    pub value: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumPathExpr {
    pub name: Token,
    pub path_items: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AssignExpr {
    pub expr: Expr,
    pub name: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub index: Expr,
    pub from: Token,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub type_annotation: Option<Token>,
    pub elements: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub arguments: Vec<Expr>,
    pub callee: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetExpr {
    pub name: Token,
    pub object: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupingExpr {
    pub expression: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LiteralExpr {
    pub value: TypedValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogicalExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetExpr {
    /// Name of value being set
    pub name: Token,
    /// The object whos field is being set
    pub object: Expr,
    /// The value the field should be set to
    pub value: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableExpr {
    pub name: Token,
}

impl LiteralExpr {
    pub fn new(value: TypedValue) -> LiteralExpr {
        LiteralExpr { value }
    }
}

impl<'a> TryInto<&'a AssignExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a AssignExpr, Self::Error> {
        match self {
            Expr::Assign(assign_expr) => Ok(assign_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("assignment".to_string()),
            ))),
        }
    }
}

impl<'a> Into<Expr> for &'a AssignExpr {
    fn into(self) -> Expr {
        Expr::Assign(Box::new(self.clone()))
    }
}

impl<'a> TryInto<&'a CallExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a CallExpr, Self::Error> {
        match self {
            Expr::Call(call_expr) => Ok(call_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("call".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a GetExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a GetExpr, Self::Error> {
        match self {
            Expr::Get(get_expr) => Ok(get_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("get".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a UnaryExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a UnaryExpr, Self::Error> {
        match self {
            Expr::Unary(unary_expr) => Ok(unary_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("unary".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a LogicalExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a LogicalExpr, Self::Error> {
        match self {
            Expr::Logical(logical_expr) => Ok(logical_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("logical".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a SetExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a SetExpr, Self::Error> {
        match self {
            Expr::Set(set_expr) => Ok(set_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("set".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a SetArrayElementExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a SetArrayElementExpr, Self::Error> {
        match self {
            Expr::SetArrayElement(set_array_element_expr) => Ok(set_array_element_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("set array element".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a ArrayExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a ArrayExpr, Self::Error> {
        match self {
            Expr::Array(array_expr) => Ok(array_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("array".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a IndexExpr> for &'a Expr {
    type Error = LangErrorTwo;

    fn try_into(self) -> Result<&'a IndexExpr, Self::Error> {
        match self {
            Expr::Index(index_expr) => Ok(index_expr),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("index".to_string()),
            ))),
        }
    }
}
