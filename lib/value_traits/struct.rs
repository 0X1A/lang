use crate::error::*;
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::value::TypedValue;

pub trait StructTrait {
    fn get_name(&self) -> String;
    fn box_clone(&self) -> Box<dyn StructTrait>;
    fn get_field(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
    ) -> Result<TypedValue, LangErrorTwo>;
    fn field_exists(&self, name: &str) -> bool;
    fn define_method(&mut self, name: &Token, value: TypedValue) -> Result<(), LangErrorTwo>;
    fn get_method(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
    ) -> Result<TypedValue, LangErrorTwo>;
    fn set_field(&mut self, name: &Token, value: &TypedValue) -> Result<(), LangErrorTwo>;
}
