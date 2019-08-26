use crate::error::*;
use crate::syntax::token::TokenTwo;
use crate::value::TypedValue;
use std::convert::Into;
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    Assign(Box<AssignExpr<'a>>),
    Binary(Box<BinaryExpr<'a>>),
    Call(Box<CallExpr<'a>>),
    Get(Box<GetExpr<'a>>),
    EnumPath(Box<EnumPathExpr<'a>>),
    Grouping(Box<GroupingExpr<'a>>),
    Literal(Box<LiteralExpr>),
    Logical(Box<LogicalExpr<'a>>),
    Set(Box<SetExpr<'a>>),
    Unary(Box<UnaryExpr<'a>>),
    Array(Box<ArrayExpr<'a>>),
    Index(Box<IndexExpr<'a>>),
    SetArrayElement(Box<SetArrayElementExpr<'a>>),
    Variable(Box<VariableExpr<'a>>),
    SelfIdent(Box<SelfIdentExpr<'a>>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfIdentExpr<'a> {
    pub keyword: TokenTwo<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetArrayElementExpr<'a> {
    pub index: Expr<'a>,
    pub name: TokenTwo<'a>,
    pub value: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumPathExpr<'a> {
    pub name: TokenTwo<'a>,
    pub path_items: Vec<TokenTwo<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AssignExpr<'a> {
    pub expr: Expr<'a>,
    pub name: TokenTwo<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IndexExpr<'a> {
    pub index: Expr<'a>,
    pub from: TokenTwo<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayExpr<'a> {
    pub type_annotation: Option<TokenTwo<'a>>,
    pub elements: Vec<Expr<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryExpr<'a> {
    pub left: Expr<'a>,
    pub operator: TokenTwo<'a>,
    pub right: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallExpr<'a> {
    pub arguments: Vec<Expr<'a>>,
    pub callee: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetExpr<'a> {
    pub name: TokenTwo<'a>,
    pub object: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupingExpr<'a> {
    pub expression: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LiteralExpr {
    pub value: TypedValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogicalExpr<'a> {
    pub left: Expr<'a>,
    pub operator: TokenTwo<'a>,
    pub right: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetExpr<'a> {
    /// Name of value being set
    pub name: TokenTwo<'a>,
    /// The object whos field is being set
    pub object: Expr<'a>,
    /// The value the field should be set to
    pub value: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnaryExpr<'a> {
    pub operator: TokenTwo<'a>,
    pub right: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariableExpr<'a> {
    pub name: TokenTwo<'a>,
}

impl LiteralExpr {
    pub fn new(value: TypedValue) -> LiteralExpr {
        LiteralExpr { value }
    }
}

impl<'a> TryInto<&'a AssignExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a AssignExpr<'a>, Self::Error> {
        match self {
            Expr::Assign(assign_expr) => Ok(assign_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("assignment".to_string()),
            ))),
        }
    }
}

impl<'a> Into<Expr<'a>> for &'a AssignExpr<'a> {
    fn into(self) -> Expr<'a> {
        Expr::Assign(Box::new(self.clone()))
    }
}

impl<'a> TryInto<&'a CallExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a CallExpr<'a>, Self::Error> {
        match self {
            Expr::Call(call_expr) => Ok(call_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("call".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a GetExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a GetExpr<'a>, Self::Error> {
        match self {
            Expr::Get(get_expr) => Ok(get_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("get".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a UnaryExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a UnaryExpr<'a>, Self::Error> {
        match self {
            Expr::Unary(unary_expr) => Ok(unary_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("unary".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a LogicalExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a LogicalExpr<'a>, Self::Error> {
        match self {
            Expr::Logical(logical_expr) => Ok(logical_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("logical".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a SetExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a SetExpr<'a>, Self::Error> {
        match self {
            Expr::Set(set_expr) => Ok(set_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("set".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a SetArrayElementExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a SetArrayElementExpr<'a>, Self::Error> {
        match self {
            Expr::SetArrayElement(set_array_element_expr) => Ok(set_array_element_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("set array element".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a ArrayExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a ArrayExpr<'a>, Self::Error> {
        match self {
            Expr::Array(array_expr) => Ok(array_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("array".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a IndexExpr<'a>> for &'a Expr<'a> {
    type Error = LangError;

    fn try_into(self) -> Result<&'a IndexExpr<'a>, Self::Error> {
        match self {
            Expr::Index(index_expr) => Ok(index_expr),
            _ => Err(LangErrorType::new_iie_error(error_message(
                &ErrMessage::ExpectExpr("index".to_string()),
            ))),
        }
    }
}
