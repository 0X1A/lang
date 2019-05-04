use crate::error::*;
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::value::TypedValue;

pub trait StructTrait {
    fn get_name(&self) -> String;
    fn box_clone(&self) -> Box<dyn StructTrait>;
    fn get_field(&self, name: &str, interpreter: &mut Interpreter)
        -> Result<TypedValue, LangError>;
    fn field_exists(&self, name: &str) -> bool;
    fn define_method(&mut self, name: &Token, value: TypedValue) -> Result<(), LangError>;
    fn get_method(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
    ) -> Result<TypedValue, LangError>;
    fn set_field(&mut self, name: &Token, value: &TypedValue) -> Result<(), LangError>;
}
