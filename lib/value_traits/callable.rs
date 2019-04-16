use ast::stmt::*;
use error::*;
use interpreter::*;
use std::fmt::{self, Debug};
use token::TypeAnnotation;
use value::TypedValue;

pub trait CallableTrait {
    fn get_name(&self) -> String;
    fn arity(&self) -> usize;
    fn get_return_type(&self) -> Option<TypeAnnotation>;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<TypedValue>,
    ) -> Result<TypedValue, LangError>;
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
impl PartialEq for CallableTrait {
    #[inline(always)]
    fn eq(&self, _: &CallableTrait) -> bool {
        false
    }
}

impl Debug for CallableTrait {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
