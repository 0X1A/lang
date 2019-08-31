extern crate log;

use crate::error::*;
use crate::lang::*;
use crate::token::Token;
use crate::type_checker::TypeChecker;
use crate::value::*;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug)]
pub struct EnvironmentId {
    pub index: usize,
}

// TODO: Generationl indices, memory management. Memory management will likely come along with
// a VM
#[derive(Clone, Debug)]
pub struct EnvironmentEntry {
    pub values: HashMap<String, TypedValue>,
    pub enclosing: Option<EnvironmentId>,
}

pub struct Environment {
    pub entries: Vec<EnvironmentEntry>,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.entries.iter() {
            write!(f, "[")?;
            write!(
                f,
                "{}",
                entry
                    .values
                    .iter()
                    .map(|ref kvp| format!("{} => {}", kvp.0, kvp.1.value))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl Default for Environment {
    fn default() -> Environment {
        Environment {
            entries: Vec::new(),
        }
    }
}

impl Index<&EnvironmentId> for Environment {
    type Output = EnvironmentEntry;

    fn index(&self, env_id: &EnvironmentId) -> &Self::Output {
        &self.entries[env_id.index]
    }
}

impl IndexMut<&EnvironmentId> for Environment {
    fn index_mut<'a>(&'a mut self, env_id: &EnvironmentId) -> &'a mut EnvironmentEntry {
        &mut self.entries[env_id.index]
    }
}

impl Environment {
    pub fn new_entry(&mut self) -> EnvironmentId {
        let env_id = self.entries.len();
        self.entries.push(EnvironmentEntry {
            values: HashMap::new(),
            enclosing: None,
        });
        EnvironmentId { index: env_id }
    }

    pub fn get_at(&mut self, index: usize, name: &str) -> Result<TypedValue, LangError> {
        self.entries.get(index).map_or(
            Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::GenericError {
                    reason: format!(
                        "tried to index an environment with lenght {} at index {}",
                        self.entries.len(),
                        index
                    ),
                },
            )),
            |env_entry| {
                env_entry.values.get(name).map_or(
                    Err(LangErrorType::new_runtime_error(
                        RuntimeErrorType::UndefinedVariable {
                            reason: format!(
                                "tried to get an undefined variable: '{}' at index {}",
                                name, index
                            ),
                        },
                    )),
                    |value| Ok(value.clone()),
                )
            },
        )
    }

    pub fn get(&self, env_id: &EnvironmentId, name: &str) -> Result<TypedValue, LangError> {
        if self[env_id].values.contains_key(name) {
            return Ok(self[env_id].values[name].clone());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            return Ok(self.get(&enclosing, name)?);
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangErrorType::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("Tried to get a variable: '{}'", name),
            },
        ))
    }

    pub fn define(&mut self, env_id: &EnvironmentId, name: &str, value: TypedValue) {
        debug!(
            "{}:{} Defining '{}' with value '{:?}' at index '{}'",
            file!(),
            line!(),
            name,
            value,
            env_id.index
        );
        self[env_id].values.insert(name.to_string(), value.clone());
    }

    pub fn assign(
        &mut self,
        env_id: &EnvironmentId,
        name: &str,
        value: TypedValue,
    ) -> Result<(), LangError> {
        debug!(
            "{}:{} Assigning '{}' with value '{:?}' at index '{}'",
            file!(),
            line!(),
            name,
            value,
            env_id.index
        );
        if self[env_id].values.contains_key(name) {
            if let Some(existing_value) = self[env_id].values.get(name) {
                if !TypeChecker::can_convert_implicitly(existing_value, &value) {
                    TypeChecker::check_type(existing_value, &value)?;
                }
            }
            self[env_id].values.insert(name.to_string(), value.clone());
            return Ok(());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            self.assign(&enclosing, name, value)?;
            return Ok(());
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangErrorType::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("tried to assign an undefined variable: '{}'", name),
            },
        ))
    }

    pub fn assign_index_entry(
        &mut self,
        env_id: &EnvironmentId,
        name: &str,
        value: &TypedValue,
        index: usize,
    ) -> Result<(), LangError> {
        if let Some(arr_value) = self[env_id].values.get_mut(name) {
            match arr_value.value {
                Value::Array(ref mut arr) => {
                    if index >= arr.len() {
                        return Err(Lang::error_s(
                            name.into(),
                            &format!(
                                "Index out of bounds. Tried to index at {} for an array of length {}",
                                index,
                                arr.len()
                            ),
                        ));
                    }
                    arr[index].assign_checked(value)?;
                    return Ok(());
                }
                _ => {
                    return Err(Lang::error_s(
                        name,
                        &format!("Tried to assign an undefined variable: '{}'", name.clone(),),
                    ));
                }
            }
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(Lang::error_s(
            name,
            &format!(
                "Tried to assign an undefined variable: '{}', self at time of undefined var: {:?}",
                name.clone(),
                self
            ),
        ))
    }

    pub fn entry_from(&mut self, enclosing: &EnvironmentId) -> EnvironmentId {
        let new_entry = self.new_entry();
        self[&new_entry].enclosing = Some(enclosing.clone());
        new_entry
    }

    pub fn remove_entry(&mut self, env_id: &EnvironmentId) {
        self.entries.remove(env_id.index);
    }

    pub fn is_defined(&self, env_id: &EnvironmentId, name: String) -> bool {
        if self[env_id].values.contains_key(&name) {
            return true;
        }
        if let Some(ref enclosing) = self[env_id].enclosing {
            return self.is_defined(enclosing, name.clone());
        }
        false
    }

    pub fn get_ref(&self, env_id: &EnvironmentId, name: &String) -> Result<&TypedValue, LangError> {
        debug!(
            "{}:{} Looking for token with lexeme '{}' at index '{}' env: {:?}",
            file!(),
            line!(),
            name,
            env_id.index,
            self
        );
        if let Some(value) = self[env_id].values.get(name) {
            return Ok(value);
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            return Ok(self.get_ref(&enclosing, name)?);
        } else {
            Err(Lang::error_s(
                name,
                &format!(
                    "(get) Tried to get an undefined variable: '{}'",
                    name.clone()
                ),
            ))
        }
    }

    pub fn update_value<Closure>(
        &mut self,
        env_id: &EnvironmentId,
        name: &str,
        closure: Closure,
    ) -> Result<(), LangError>
    where
        Closure: FnOnce(&mut TypedValue) -> Result<(), LangError>,
    {
        if let Some(value) = self[env_id].values.get_mut(name) {
            closure(value)?;
        } else {
            debug!("{}:{} Didn't find the thing :O", file!(), line!());
        }
        Ok(())
    }
}
