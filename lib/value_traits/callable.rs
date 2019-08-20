use crate::ast::stmt::*;
use crate::error::*;
use crate::interpreter::*;
use crate::token::TypeAnnotation;
use crate::value::StructInstanceTrait;
use crate::value::TypedValue;
use std::fmt::{self, Debug};

pub trait CallableTrait {
    fn get_name(&self) -> String;
    fn arity(&self) -> usize;
    fn get_return_type(&self) -> Option<TypeAnnotation>;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<TypedValue>,
    ) -> Result<TypedValue, LangError>;
    fn bind(
        &self,
        struct_instance: &dyn StructInstanceTrait,
        interpreter: &mut Interpreter,
    ) -> Result<(), LangError>;
    fn get_params(&self) -> Vec<VariableData>;
    fn box_clone(&self) -> Box<dyn CallableTrait>;
}

impl Clone for Box<dyn CallableTrait> {
    fn clone(&self) -> Box<dyn CallableTrait> {
        self.box_clone()
    }
}

// TODO (?)
// Don't think any functions should ever be equal
impl PartialEq for dyn CallableTrait {
    #[inline(always)]
    fn eq(&self, _: &dyn CallableTrait) -> bool {
        false
    }
}

impl Debug for dyn CallableTrait {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
