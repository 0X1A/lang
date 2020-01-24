use crate::error::*;
use crate::mem::*;
use crate::value::TypedValue;

pub trait StructTrait {
    fn get_name(&self) -> String;
    fn box_clone(&self) -> Box<dyn StructTrait>;
    fn get_field(&self, name: &str) -> Result<ArenaEntryIndex, LangError>;
    fn field_exists(&self, name: &str) -> bool;
    fn define_method(&mut self, name: &str, value_index: ArenaEntryIndex) -> Result<(), LangError>;
    fn get_method(&self, name: &str) -> Result<ArenaEntryIndex, LangError>;
    fn set_field(&mut self, name: &str, value: &TypedValue) -> Result<(), LangError>;
}
