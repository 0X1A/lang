use crate::env::*;
use crate::error::*;
use crate::interpreter::Interpreter;
use crate::mem::*;
use crate::value::TypedValue;

pub trait StructTrait {
    fn get_name(&self) -> String;
    fn box_clone(&self) -> Box<dyn StructTrait>;
    fn get_field(
        &self,
        name: &str,
        env: &mut Environment,
        arena: &mut Arena<TypedValue>,
        interpreter: &Interpreter,
    ) -> Result<TypedValue, LangError>;
    fn field_exists(&self, name: &str) -> bool;
    fn define_method(&mut self, name: &str, value: TypedValue) -> Result<(), LangError>;
    fn get_method(
        &self,
        name: &str,
        env: &mut Environment,
        arena: &mut Arena<TypedValue>,
    ) -> Result<TypedValue, LangError>;
    fn set_field(&mut self, name: &str, value: &TypedValue) -> Result<(), LangError>;
}
