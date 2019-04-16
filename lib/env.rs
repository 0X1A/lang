extern crate log;

use error::*;
use lang::*;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};
use token::Token;
use type_checker::TypeChecker;
use value::*;

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
            Err(LangError::new_runtime_error(
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
                    Err(LangError::new_runtime_error(
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

    pub fn direct_get(
        &self,
        env_id: &EnvironmentId,
        name: String,
    ) -> Result<TypedValue, LangError> {
        if self[env_id].values.contains_key(&name) {
            return Ok(self[env_id].values[&name].clone());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            return Ok(self.direct_get(&enclosing, name)?);
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangError::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("Tried to get a variable: '{}'", name),
            },
        ))
    }

    pub fn direct_assign(
        &mut self,
        env_id: &EnvironmentId,
        name: String,
        value: TypedValue,
    ) -> Result<(), LangError> {
        debug!(
            "Env::direct_assign\nAssigning '{}' with value '{:?}' at index '{}'",
            name, value, env_id.index
        );
        if self[env_id].values.contains_key(&name) {
            self[env_id].values.insert(name.clone(), value.clone());
            return Ok(());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            self.direct_assign(&enclosing, name, value)?;
            return Ok(());
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangError::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("Tried to assign a variable: '{}'", name),
            },
        ))
    }

    pub fn assign_at(
        &mut self,
        index: usize,
        name: &Token,
        value: &TypedValue,
    ) -> Result<(), LangError> {
        debug!(
            "Env::assign_at\nAssigning '{}' with value '{:?}' at index '{}'",
            name.lexeme, value, index
        );
        if index < self.entries.len() {
            self.entries[index]
                .values
                .insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        Err(LangError::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("Tried to assign a variable: '{}'", name),
            },
        ))
    }

    pub fn assign_index_entry(
        &mut self,
        env_id: &EnvironmentId,
        name: &Token,
        value: &TypedValue,
        index: usize,
    ) -> Result<(), LangError> {
        if let Some(arr_value) = self[env_id].values.get_mut(&name.lexeme) {
            match arr_value.value {
                Value::Array(ref mut arr) => {
                    if index >= arr.len() {
                        return Err(Lang::error(
                            name,
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
                    return Err(Lang::error(
                        name,
                        &format!(
                            "Tried to assign an undefined variable: '{}'",
                            name.lexeme.clone(),
                        ),
                    ));
                }
            }
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(Lang::error(
            name,
            &format!(
                "Tried to assign an undefined variable: '{}', self at time of undefined var: {:?}",
                name.lexeme.clone(),
                self
            ),
        ))
    }

    pub fn assign(
        &mut self,
        env_id: &EnvironmentId,
        name: &Token,
        value: &TypedValue,
    ) -> Result<(), LangError> {
        debug!(
            "Env::assign\nAssigning '{}' with value '{:?}' at index '{}'",
            name.lexeme, value, env_id.index
        );
        if self[env_id].values.contains_key(&name.lexeme) {
            if let Some(existing_value) = self[env_id].values.get(&name.lexeme) {
                if existing_value.value_type != value.value_type {
                    return Err(Lang::error(
                        name,
                        &format!(
                            "expected type {}, found {}",
                            existing_value.value_type, value.value_type
                        ),
                    ));
                }
                TypeChecker::check_type(&existing_value, &value)?;
            }
            self[env_id]
                .values
                .insert(name.lexeme.clone(), value.clone());
            return Ok(());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            self.assign(&enclosing, name, value)?;
            return Ok(());
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(Lang::error(
            name,
            &format!(
                "Tried to assign an undefined variable: '{}', self at time of undefined var: {:?}",
                name.lexeme.clone(),
                self
            ),
        ))
    }

    pub fn entry_from(&mut self, enclosing: &EnvironmentId) -> EnvironmentId {
        let new_entry = self.new_entry();
        self[&new_entry].enclosing = Some(enclosing.clone());
        new_entry
    }

    // Uses closure to make a temporary env entry
    pub fn in_context<F>(&mut self, enclosing: &EnvironmentId, context: F)
    where
        F: FnOnce(),
    {
        let new_entry = self.new_entry();
        self[&new_entry].enclosing = Some(enclosing.clone());
        context();
        self.remove_entry(&new_entry);
    }

    pub fn remove_entry(&mut self, env_id: &EnvironmentId) {
        self.entries.remove(env_id.index);
    }

    /// Inserts the value `value` into the environment's hash table using `name`
    pub fn define(&mut self, env_id: &EnvironmentId, name: &str, value: &TypedValue) {
        self[env_id].values.insert(name.to_string(), value.clone());
        debug!(
            "Env::define\nDefining '{}' with value '{:?}' at index '{}' env '{:?}'",
            name, value, env_id.index, self
        );
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

    pub fn get(&self, env_id: &EnvironmentId, name: &Token) -> Result<TypedValue, LangError> {
        debug!(
            "Env::get\nLooking for token with lexeme '{}' at index '{}' env:\n{:?}",
            name.lexeme, env_id.index, self
        );
        //debug_assert!(env_id.index >= self.entries.len(), "EnvironmentId {} is out of bounds of environment entries of len {}", env_id.index, self.entries.len());
        if let Some(value) = self[env_id].values.get(&name.lexeme) {
            return Ok(value.clone());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            return Ok(self.get(&enclosing, name)?);
        } else {
            Err(Lang::error(
                name,
                &format!(
                    "(get) Tried to get an undefined variable: '{}'",
                    name.lexeme.clone()
                ),
            ))
        }
    }

    pub fn mutable_action<Closure>(
        &mut self,
        env_id: &EnvironmentId,
        name: &Token,
        mut closure: Closure,
    ) -> Result<(), LangError>
    where
        Closure: FnMut(&mut TypedValue) -> Result<(), LangError>,
    {
        if let Some(value) = self[env_id].values.get_mut(&name.lexeme) {
            closure(value)?;
        }
        Ok(())
    }
}
