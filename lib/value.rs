use ast::stmt::*;
use env::*;
use error::*;
use interpreter::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::TryInto,
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    ops::*,
};
use token::{Token, TokenType, TypeAnnotation};
use value_traits::callable::CallableTrait;
use value_traits::r#struct::StructTrait;

#[derive(Clone, Copy, PartialOrd, Debug)]
pub struct Float64 {
    pub inner: f64,
}

impl Float64 {
    #[inline(always)]
    pub fn from(f: f64) -> Float64 {
        Float64 { inner: f }
    }
}

impl PartialEq for Float64 {
    #[inline(always)]
    fn eq(&self, other: &Float64) -> bool {
        self.inner == other.inner
    }
}

impl Display for Float64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Clone, Copy, PartialOrd, Debug)]
pub struct Float32 {
    pub inner: f32,
}

impl Float32 {
    #[inline(always)]
    pub fn from(f: f32) -> Float32 {
        Float32 { inner: f }
    }
}

impl PartialEq for Float32 {
    #[inline(always)]
    fn eq(&self, other: &Float32) -> bool {
        self.inner == other.inner
    }
}

impl Display for Float32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

pub struct Enum {
    pub values: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct SelfIndex {
    pub name: String,
    pub env_id: EnvironmentId,
}

pub enum Value {
    Struct(Box<dyn StructInstanceTrait>),
    Callable(Box<dyn CallableTrait>),
    Enum(Box<Enum>),
    String(String),
    Int32(i32),
    Int64(i64),
    Float32(Float32),
    Float64(Float64),
    Trait(Box<TraitValue>),
    TraitFunction(Box<TraitFunctionValue>),
    Ident(String),
    Boolean(bool),
    Array(Vec<TypedValue>),
    SelfIndex(SelfIndex),
    Unit,
}

impl<'a> TryInto<&'a StructInstanceTrait> for &'a Value {
    type Error = LangError;
    fn try_into(self) -> Result<&'a StructInstanceTrait, Self::Error> {
        match self {
            Value::Struct(struct_value) => Ok(&**struct_value),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectValueType("struct".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a mut StructInstanceTrait> for &'a mut Value {
    type Error = LangError;
    fn try_into(self) -> Result<&'a mut StructInstanceTrait, Self::Error> {
        match self {
            Value::Struct(struct_value) => Ok(&mut **struct_value),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectValueType("struct".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a CallableTrait> for &'a Value {
    type Error = LangError;
    fn try_into(self) -> Result<&'a CallableTrait, Self::Error> {
        match self {
            Value::Callable(struct_value) => Ok(&**struct_value),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectValueType("callable".to_string()),
            ))),
        }
    }
}

impl<'a> TryInto<&'a TraitValue> for &'a Value {
    type Error = LangError;
    fn try_into(self) -> Result<&'a TraitValue, Self::Error> {
        match self {
            Value::Trait(struct_value) => Ok(&**struct_value),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectValueType("trait".to_string()),
            ))),
        }
    }
}

impl TryInto<TraitFunctionValue> for Value {
    type Error = LangError;
    fn try_into(self) -> Result<TraitFunctionValue, Self::Error> {
        match self {
            Value::TraitFunction(trait_function) => Ok(*trait_function),
            _ => Err(LangError::new_iie_error(error_message(
                &ErrMessage::ExpectValueType("trait function".to_string()),
            ))),
        }
    }
}

impl Value {
    pub fn default_value(type_annotation: &TypeAnnotation) -> Value {
        match type_annotation {
            TypeAnnotation::I32 => Value::Int32(0),
            TypeAnnotation::I64 => Value::Int64(0),
            TypeAnnotation::F32 => Value::Float32(Float32::from(0.0)),
            TypeAnnotation::F64 => Value::Float64(Float64::from(0.0)),
            TypeAnnotation::Bool => Value::Boolean(false),
            TypeAnnotation::String => Value::String(String::new()),
            TypeAnnotation::Array(_) => Value::Array(Vec::new()),
            // Struct, callable defaults values are deferred until they hit
            // the interpreter. Ident should never be processed because
            // ident is only ever used by the scanner
            _ => Value::Unit,
        }
    }

    pub fn type_to_str(&self) -> &str {
        match self {
            Value::Struct(_) => "struct",
            Value::SelfIndex(_) => "self",
            Value::Callable(_) => "callable",
            Value::String(_) => "string",
            Value::Enum(_) => "enum",
            Value::Int32(_) => "i32",
            Value::Int64(_) => "i64",
            Value::Float32(_) => "f32",
            Value::Float64(_) => "f64",
            Value::TraitFunction(_) => "trait fn",
            Value::Trait(_) => "trait",
            Value::Ident(_) => "Ident",
            Value::Boolean(_) => "bool",
            Value::Array(_) => "array",
            Value::Unit => "()",
        }
    }

    pub fn as_array_index(&self) -> Result<usize, LangError> {
        match self {
            Value::Int64(i) => Ok(*i as usize),
            Value::Float64(f) => Ok(f.inner as usize),
            Value::Boolean(b) => Ok(*b as usize),
            _ => Err(LangError::new_runtime_error(
                RuntimeErrorType::GenericError {
                    reason: error_message(&ErrMessage::IncorrectIndexType(self.to_string())),
                },
            )),
        }
    }
}

impl Hash for Value {
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
    }
}

// TODO: All operations should be done by implementing traits so we can have
// operator overloading
impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        match self {
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => Value::Int64(lhs + rhs),
                Value::Float64(rhs) => Value::Int64(lhs + rhs.inner as i64),
                _ => Value::Int64(lhs),
            },
            Value::Float64(lhs) => match other {
                Value::Int64(rhs) => Value::Float64(Float64::from(lhs.inner + rhs as f64)),
                Value::Float64(rhs) => Value::Float64(Float64::from(lhs.inner + rhs.inner)),
                _ => Value::Float64(lhs),
            },
            Value::String(lhs) => match other {
                Value::String(rhs) => Value::String(format!("{}{}", lhs, rhs)),
                _ => Value::String(lhs),
            },
            _ => self,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        match self {
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => Value::Int64(lhs - rhs),
                Value::Float64(rhs) => Value::Int64(lhs - rhs.inner as i64),
                _ => Value::Int64(lhs),
            },
            Value::Float64(lhs) => match other {
                Value::Int64(rhs) => Value::Float64(Float64::from(lhs.inner - rhs as f64)),
                Value::Float64(rhs) => Value::Float64(Float64::from(lhs.inner - rhs.inner)),
                _ => Value::Float64(lhs),
            },
            _ => self,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        match self {
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => Value::Int64(lhs * rhs),
                Value::Float64(rhs) => Value::Int64(lhs * rhs.inner as i64),
                _ => Value::Int64(lhs),
            },
            Value::Float64(lhs) => match other {
                Value::Int64(rhs) => Value::Float64(Float64::from(lhs.inner * rhs as f64)),
                Value::Float64(rhs) => Value::Float64(Float64::from(lhs.inner * rhs.inner)),
                _ => Value::Float64(lhs),
            },
            _ => self,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        match self {
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => Value::Int64(lhs / rhs),
                Value::Float64(rhs) => Value::Int64(lhs / rhs.inner as i64),
                _ => Value::Int64(lhs),
            },
            Value::Float64(lhs) => match other {
                Value::Int64(rhs) => Value::Float64(Float64::from(lhs.inner / rhs as f64)),
                Value::Float64(rhs) => Value::Float64(Float64::from(lhs.inner / rhs.inner)),
                _ => Value::Float64(lhs),
            },
            _ => self,
        }
    }
}

impl Eq for Value {}

// Value equivalence
impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Value::Struct(_) => false,
            Value::SelfIndex(_) => false,
            Value::Callable(_) => false,
            Value::Enum(_) => unimplemented!(),
            Value::String(lhs) => match other {
                Value::String(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Int32(lhs) => match other {
                Value::Int32(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Float32(lhs) => match other {
                Value::Float32(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Float64(lhs) => match other {
                Value::Float64(rhs) => lhs == rhs,
                _ => false,
            },
            Value::TraitFunction(_) => false,
            Value::Trait(_) => false,
            Value::Ident(lhs) => match other {
                Value::Ident(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Boolean(lhs) => match other {
                Value::Boolean(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Array(lhs) => match other {
                Value::Array(rhs) => lhs == rhs,
                _ => false,
            },
            Value::Unit => match other {
                Value::Unit => true,
                _ => false,
            },
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait StructInstanceTrait: CallableTrait + StructTrait + Debug {
    fn box_clone(&self) -> Box<StructInstanceTrait>;
    // For constructors
    fn callable_trait(&self) -> &CallableTrait;
    fn struct_trait(&self) -> &StructTrait;
    fn set_instance_name(&mut self, name: String);
    fn get_instance_name(&self) -> String;
}

impl StructInstanceTrait for StructValue {
    fn box_clone(&self) -> Box<StructInstanceTrait> {
        Box::new((*self).clone())
    }

    fn set_instance_name(&mut self, name: String) {
        self.instance_name = name;
    }

    fn get_instance_name(&self) -> String {
        self.instance_name.clone()
    }

    fn struct_trait(&self) -> &StructTrait {
        self
    }

    fn callable_trait(&self) -> &CallableTrait {
        self
    }
}

impl Clone for Box<StructInstanceTrait> {
    fn clone(&self) -> Box<StructInstanceTrait> {
        StructInstanceTrait::box_clone(&**self)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        match self {
            Value::Struct(_) => Ordering::Less,
            Value::SelfIndex(_) => Ordering::Less,
            Value::Callable(_) => Ordering::Less,
            Value::Enum(_) => Ordering::Less,
            Value::String(lhs) => match other {
                Value::String(rhs) => lhs.cmp(&rhs),
                _ => Ordering::Less,
            },
            Value::Int32(lhs) => match other {
                Value::Int32(rhs) => lhs.cmp(&rhs),
                _ => Ordering::Less,
            },
            Value::Int64(lhs) => match other {
                Value::Int64(rhs) => lhs.cmp(&rhs),
                _ => Ordering::Less,
            },
            Value::Float32(lhs) => match other {
                Value::Float32(rhs) => {
                    if lhs == rhs {
                        Ordering::Equal
                    } else if lhs < rhs {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                _ => Ordering::Less,
            },
            Value::Float64(lhs) => match other {
                Value::Float64(rhs) => {
                    if lhs == rhs {
                        Ordering::Equal
                    } else if lhs < rhs {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                _ => Ordering::Less,
            },
            Value::TraitFunction(_) => Ordering::Less,
            Value::Trait(_) => Ordering::Less,
            Value::Ident(lhs) => match other {
                Value::Ident(rhs) => lhs.cmp(&rhs),
                _ => Ordering::Less,
            },
            Value::Boolean(lhs) => match other {
                Value::Boolean(rhs) => lhs.cmp(&rhs),
                _ => Ordering::Less,
            },
            // TODO: Revisit this, ordering for arrays may just be dumb c:
            Value::Array(lhs) => match other {
                Value::Array(rhs) => {
                    if lhs.len() > rhs.len() {
                        Ordering::Greater
                    } else if rhs.len() > lhs.len() {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                _ => Ordering::Less,
            },
            Value::Unit => match other {
                Value::Unit => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Struct(s) => Value::Struct(s.clone()),
            Value::SelfIndex(s) => Value::SelfIndex(s.clone()),
            Value::Enum(_) => unimplemented!(),
            Value::Callable(c) => Value::Callable(c.clone()),
            Value::String(lhs) => Value::String(lhs.clone()),
            Value::Int32(lhs) => Value::Int32(*lhs),
            Value::Int64(lhs) => Value::Int64(*lhs),
            Value::Float32(lhs) => Value::Float32(*lhs),
            Value::Float64(lhs) => Value::Float64(*lhs),
            Value::TraitFunction(lhs) => Value::TraitFunction(lhs.clone()),
            Value::Trait(lhs) => Value::Trait(lhs.clone()),
            Value::Ident(lhs) => Value::Ident(lhs.clone()),
            Value::Boolean(lhs) => Value::Boolean(*lhs),
            Value::Array(lhs) => Value::Array(lhs.clone()),
            Value::Unit => Value::Unit,
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Struct(struct_value) => write!(
                f,
                "Value::Struct({})",
                struct_value.struct_trait().get_name()
            ),
            Value::SelfIndex(self_index) => write!(
                f,
                "Value::SelfIndex({}, {})",
                self_index.name, self_index.env_id.index
            ),
            Value::Enum(_) => unimplemented!(),
            Value::Callable(callable_value) => {
                write!(f, "Value::Callable({})", callable_value.get_name())
            }
            Value::String(string_value) => write!(f, "Value::String({})", string_value),
            Value::Int32(int_value) => write!(f, "Value::Int32({})", int_value),
            Value::Int64(int_value) => write!(f, "Value::Int64({})", int_value),
            Value::Float32(float_value) => write!(f, "Value::Float32({})", float_value),
            Value::Float64(float_value) => write!(f, "Value::Float64({})", float_value),
            // TODO: Fix this
            Value::TraitFunction(trait_fn_value) => {
                write!(f, "Value::TraitFunction({:?})", trait_fn_value)
            }
            Value::Trait(trait_value) => write!(f, "Value::Trait({:?})", trait_value),
            Value::Ident(ident_value) => write!(f, "Value::Ident({})", ident_value),
            Value::Boolean(bool_value) => write!(f, "Value::Boolean({})", bool_value),
            Value::Array(array_value) => write!(f, "Value::Array({:?})", array_value),
            Value::Unit => write!(f, "Value::Unit"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TraitValue {
    pub trait_stmt: TraitStmt,
    pub fn_declarations: HashMap<String, TypedValue>,
}

#[derive(Clone, Debug)]
pub struct TraitFunctionValue {
    pub function: TraitFunctionStmt,
}

#[derive(Clone, Debug, PartialEq, Hash, PartialOrd)]
pub struct TypedValue {
    pub value: Value,
    pub value_type: TypeAnnotation,
}

impl Eq for TypedValue {}

impl TypedValue {
    /// Checks the type annotation of `other`, returning `Err` if it does not match that of `self`.
    /// Clones the value
    pub fn assign_checked(&mut self, other: &TypedValue) -> Result<(), LangError> {
        self.check_type(other)?;
        *self = other.clone();
        Ok(())
    }

    pub fn check_type(&self, value: &TypedValue) -> Result<(), LangError> {
        let matches = match self.value_type.clone() {
            TypeAnnotation::User(lhs_struct) => match value.value_type {
                TypeAnnotation::User(ref rhs_struct) => lhs_struct == *rhs_struct,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::String => match value.value_type {
                TypeAnnotation::String => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::I32 => match value.value_type {
                TypeAnnotation::I32 => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::I64 => match value.value_type {
                TypeAnnotation::I64 => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::F32 => match value.value_type {
                TypeAnnotation::F32 => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::F64 => match value.value_type {
                TypeAnnotation::F64 => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::Bool => match value.value_type {
                TypeAnnotation::Bool => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::Array(lhs_type) => match value.value_type.clone() {
                TypeAnnotation::Array(rhs_type) => (*lhs_type).eq(&*rhs_type),
                TypeAnnotation::Unit => true,
                _ => false,
            },
            TypeAnnotation::Fn => match value.value_type {
                TypeAnnotation::Fn => true,
                TypeAnnotation::Unit => true,
                _ => false,
            },
            // TODO(?) this ends up being such that we don't care if we reassign unit values.
            _ => true,
        };
        if !matches {
            Err(LangError::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "tried to assign value of type {} to value of type {}",
                        self.value_type.to_string(),
                        value.value_type.to_string()
                    ),
                },
            ))
        } else {
            Ok(())
        }
    }

    pub fn as_array_index(&self) -> Result<usize, LangError> {
        match self.value {
            Value::Int64(i) => Ok(i as usize),
            Value::Float64(f) => Ok(f.inner as usize),
            Value::Boolean(b) => Ok(b as usize),
            _ => Err(LangError::new_runtime_error(
                RuntimeErrorType::GenericError {
                    reason: format!(
                        "Tried to index an array with incorrect type '{}'",
                        self.value.type_to_str()
                    ),
                },
            )),
        }
    }
}

impl TypedValue {
    pub fn new(value: Value, value_type: TypeAnnotation) -> TypedValue {
        TypedValue { value, value_type }
    }
}

#[derive(Clone, Debug)]
pub struct StructValue {
    struct_stmt: StructStmt,
    fields: HashMap<String, TypedValue>,
    methods: HashMap<String, TypedValue>,
    instance_name: String,
}

impl StructValue {
    pub fn new(
        struct_stmt: StructStmt,
        fields: HashMap<String, TypedValue>,
        instance_name: String,
    ) -> StructValue {
        StructValue {
            struct_stmt,
            methods: HashMap::new(),
            fields,
            instance_name,
        }
    }
}

impl CallableTrait for StructValue {
    fn get_name(&self) -> String {
        self.struct_stmt.name.lexeme.clone()
    }

    fn arity(&self) -> usize {
        0
    }

    fn get_return_type(&self) -> Option<TypeAnnotation> {
        Some(TypeAnnotation::User(self.struct_stmt.name.lexeme.clone()))
    }

    fn bind(&self, _: &StructInstanceTrait, _: &mut Interpreter) -> Result<(), LangError> {
        unimplemented!()
    }

    fn get_params(&self) -> Vec<VariableData> {
        vec![]
    }

    // TODO: This should take constructor args
    fn call(&self, _: &mut Interpreter, _: Vec<TypedValue>) -> Result<TypedValue, LangError> {
        Ok(TypedValue::new(
            Value::Struct(Box::new(self.clone())),
            TypeAnnotation::User(CallableTrait::get_name(self)),
        ))
    }

    fn box_clone(&self) -> Box<dyn CallableTrait> {
        Box::new((*self).clone())
    }
}

impl StructTrait for StructValue {
    fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    fn box_clone(&self) -> Box<dyn StructTrait> {
        Box::new((*self).clone())
    }

    #[inline(always)]
    fn field_exists(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    fn get_field(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
    ) -> Result<TypedValue, LangError> {
        self.fields.get(name).map_or(
            {
                if let Ok(method) = self.get_method(name, interpreter) {
                    return Ok(method);
                }
                Err(LangError::new_runtime_error(
                    RuntimeErrorType::UndefinedVariable {
                        reason: format!("get_field tried to get an undefined variable: '{}'", name),
                    },
                ))
            },
            |value| Ok(value.clone()),
        )
    }

    fn define_method(&mut self, name: &Token, value: TypedValue) -> Result<(), LangError> {
        debug!("Setting method with name {:?} to value {:?}", name, value);
        self.methods.insert(name.lexeme.clone(), value);
        Ok(())
    }

    fn get_method(
        &self,
        name: &str,
        interpreter: &mut Interpreter,
    ) -> Result<TypedValue, LangError> {
        debug!("get_method\nLooking for {} in {:?}", name, self.methods);
        self.methods.get(name).map_or(
            Err(LangError::new_runtime_error(
                RuntimeErrorType::UndefinedVariable {
                    reason: format!("tried to get an undefined method: '{}'", name),
                },
            )),
            |value| {
                let callable_value = value.clone();
                {
                    let callable: &CallableTrait = (&callable_value.value).try_into()?;
                    callable.bind(self, interpreter)?;
                }
                Ok(callable_value)
            },
        )
    }

    fn set_field(&mut self, name: &Token, value: &TypedValue) -> Result<(), LangError> {
        self.fields.get_mut(&name.lexeme).map_or(
            Err(LangError::new_runtime_error(
                RuntimeErrorType::UndefinedVariable {
                    reason: format!("tried to set an undefined variable: '{}'", name),
                },
            )),
            |field| field.assign_checked(value),
        )
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Struct(s) => return write!(f, "{}", s.struct_trait().get_name()),
            Value::SelfIndex(s) => return write!(f, "{}, {}", s.name, s.env_id.index),
            Value::Enum(_) => unimplemented!(),
            Value::Callable(c) => return write!(f, "{}", c.get_name()),
            Value::String(v) => {
                return write!(f, "{}", v);
            }
            Value::Int32(i) => return write!(f, "{}", i),
            Value::Int64(i) => return write!(f, "{}", i),
            Value::Float32(fl) => return write!(f, "{}", fl),
            Value::Float64(fl) => return write!(f, "{}", fl),
            // TODO: Fix this
            Value::TraitFunction(trait_fn_value) => return write!(f, "{:?}", trait_fn_value),
            Value::Trait(trait_value) => return write!(f, "{:?}", trait_value),
            Value::Ident(id) => return write!(f, "{}", id),
            Value::Boolean(b) => return write!(f, "{}", b),
            Value::Array(arr) => {
                let formatted_string: String = arr
                    .iter()
                    .map(|ref item| item.value.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                return write!(f, "{}", formatted_string);
            }
            Value::Unit => return write!(f, "unit"),
        };
    }
}

#[derive(Clone)]
pub struct Callable {
    function: FunctionStmt,
    closure: EnvironmentId,
}

impl Callable {
    pub fn new(function: FunctionStmt, closure: &EnvironmentId) -> Callable {
        Callable {
            function,
            closure: closure.clone(),
        }
    }
}

impl Hash for Float64 {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let addr = self as *const Float64 as u64;
        addr.hash(state);
    }
}

impl Hash for Box<dyn CallableTrait> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let addr = self as *const Box<dyn CallableTrait> as u64;
        addr.hash(state);
    }
}

impl CallableTrait for Callable {
    fn box_clone(&self) -> Box<dyn CallableTrait> {
        Box::new((*self).clone())
    }

    #[inline(always)]
    fn get_name(&self) -> String {
        format!("<fn {}>", self.function.name.lexeme)
    }

    #[inline(always)]
    fn arity(&self) -> usize {
        self.function.params.len()
    }

    fn bind(
        &self,
        struct_instance: &StructInstanceTrait,
        interpreter: &mut Interpreter,
    ) -> Result<(), LangError> {
        let value = TypedValue::new(
            Value::SelfIndex(SelfIndex {
                name: struct_instance.get_instance_name(),
                env_id: self.closure.clone(),
            }),
            TypeAnnotation::SelfIndex,
        );
        interpreter.env_entries.define(&self.closure, "self", value);
        Ok(())
    }

    fn get_params(&self) -> Vec<VariableData> {
        self.function.params.clone()
    }

    fn get_return_type(&self) -> Option<TypeAnnotation> {
        match self.function.return_type.token_type {
            TokenType::Type(ref type_name) => match type_name {
                _ => Some(type_name.clone()),
            },
            _ => None,
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<TypedValue>,
    ) -> Result<TypedValue, LangError> {
        let env_id = interpreter.env_entries.entry_from(&self.closure);

        if args.len() != self.arity() {
            return Err(LangError::new_runtime_error(
                RuntimeErrorType::FnArityError {
                    reason: format!(
                        "Function {} requires {} arg(s), passed {}",
                        self.get_name(),
                        self.arity(),
                        args.len()
                    ),
                },
            ));
        }
        for it in self.function.params.iter().zip(args.iter()) {
            if it.0.type_annotation != it.1.value_type {
                return Err(LangError::new_runtime_error(
                    RuntimeErrorType::InvalidFunctionArgumentType {
                        reason: format!(
                            "Tried pass an argument of type {:?} for function which takes type {:?}",
                            it.1.value_type.to_string(),
                            it.0.type_annotation.to_string()
                        ),
                    },
                ));
            }
            interpreter
                .env_entries
                .define(&env_id, &it.0.identifier.lexeme, it.1.clone());
        }
        let return_value = interpreter.execute_block(&self.function.body, env_id)?;
        if let Some(function_return_type) = self.get_return_type() {
            if function_return_type != return_value.value_type {
                return Err(LangError::new_runtime_error(
                    RuntimeErrorType::InvalidFunctionReturnType {
                        reason: format!(
                            "Tried to return value of {:?} for function which returns type {:?}",
                            return_value.value_type,
                            function_return_type.to_string()
                        ),
                    },
                ));
            }
        }
        Ok(return_value)
    }
}
