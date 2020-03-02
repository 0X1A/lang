extern crate log;

use crate::error::*;
use crate::lang::*;
use crate::mem::*;
use crate::type_checker::TypeChecker;
use crate::value::*;
use std::convert::*;
use std::{
    collections::HashMap,
    fmt::{self, Debug},
    ops::{Index, IndexMut},
};

pub type EnvironmentEntryIndex = usize;

// TODO: Generationl indices, memory management. Memory management will likely come along with
// a VM
#[derive(Clone, Debug)]
pub struct EnvironmentEntry {
    pub values: HashMap<String, ArenaEntryIndex>,
    pub enclosing: Option<EnvironmentEntryIndex>,
}

pub struct Environment {
    pub root_entry_id: EnvironmentEntryIndex,
    pub current_index: EnvironmentEntryIndex,
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
                    .map(|ref kvp| format!("{} => {}", kvp.0, kvp.1))
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
            root_entry_id: 0,
            current_index: 0,
            entries: Vec::new(),
        }
    }
}

impl Index<EnvironmentEntryIndex> for Environment {
    type Output = EnvironmentEntry;

    fn index(&self, env_id: EnvironmentEntryIndex) -> &Self::Output {
        &self.entries[env_id]
    }
}

impl IndexMut<EnvironmentEntryIndex> for Environment {
    fn index_mut(&mut self, env_id: EnvironmentEntryIndex) -> &mut EnvironmentEntry {
        &mut self.entries[env_id]
    }
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            root_entry_id: 0,
            current_index: 0,
            entries: Vec::new(),
        };
        env.root_entry_id = env.entries.len();
        env.entries.push(EnvironmentEntry {
            values: HashMap::new(),
            enclosing: None,
        });
        env
    }
    pub fn new_entry(&mut self) -> EnvironmentEntryIndex {
        let env_id = self.entries.len();
        self.entries.push(EnvironmentEntry {
            values: HashMap::new(),
            enclosing: None,
        });
        env_id
    }

    pub fn get_at(&mut self, index: usize, name: &str) -> Result<ArenaEntryIndex, LangError> {
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
                    |value| Ok(*value),
                )
            },
        )
    }

    pub fn get(
        &self,
        env_id: EnvironmentEntryIndex,
        name: &str,
    ) -> Result<ArenaEntryIndex, LangError> {
        if self[env_id].values.contains_key(name) {
            return Ok(self[env_id].values[name]);
        } else if let Some(enclosing) = self[env_id].enclosing {
            return Ok(self.get(enclosing, name)?);
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangErrorType::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("(get) Tried to get a variable: '{}'", name),
            },
        ))
    }

    pub fn define(&mut self, env_id: EnvironmentEntryIndex, name: &str, index: ArenaEntryIndex) {
        self[env_id].values.insert(name.to_string(), index);
    }

    pub fn define_and_insert(
        &mut self,
        env_id: EnvironmentEntryIndex,
        arena: &mut Arena<TypedValue>,
        name: &str,
        value: TypedValue,
    ) -> ArenaEntryIndex {
        debug!(
            "{}:{} Defining '{}' with value '{:?}' at index '{}'",
            file!(),
            line!(),
            name,
            value,
            env_id
        );
        let index = arena.insert(value);
        self[env_id].values.insert(name.to_string(), index);
        index
    }

    pub fn assign(
        &mut self,
        env_id: EnvironmentEntryIndex,
        name: &str,
        value: TypedValue,
        arena: &mut Arena<TypedValue>,
    ) -> Result<(), LangError> {
        debug!(
            "{}:{} Assigning '{}' with value '{:?}' at index '{}'",
            file!(),
            line!(),
            name,
            value,
            env_id
        );
        if self[env_id].values.contains_key(name) {
            if let Some(existing_value_index) = self[env_id].values.get(name) {
                let existing_value_entry = &mut arena[*existing_value_index];
                let existing_value: &mut TypedValue = existing_value_entry.try_into()?;
                if !TypeChecker::can_convert_implicitly(existing_value, &value) {
                    TypeChecker::check_type(existing_value, &value)?;
                }
                *existing_value = value;
            }
            return Ok(());
        } else if let Some(enclosing) = self[env_id].enclosing {
            self.assign(enclosing, name, value, arena)?;
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
        &self,
        env_id: EnvironmentEntryIndex,
        name: &str,
        value: &TypedValue,
        arena: &mut Arena<TypedValue>,
        index: usize,
    ) -> Result<(), LangError> {
        if let Some(arr_value_index) = self[env_id].values.get(name) {
            let arr_value_entry = &mut arena[*arr_value_index];
            let arr_value: &mut TypedValue = arr_value_entry.try_into()?;
            match arr_value.value {
                Value::Array(ref mut arr) => {
                    if index >= arr.len() {
                        return Err(Lang::error_s(
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
                    return Err(Lang::error_s(
                        name,
                        &format!("Tried to assign an undefined variable: '{}'", name,),
                    ));
                }
            }
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(Lang::error_s(
            name,
            &format!(
                "Tried to assign an undefined variable: '{}', self at time of undefined var: {:?}",
                name, self
            ),
        ))
    }

    pub fn entry_from(&mut self, enclosing: EnvironmentEntryIndex) -> EnvironmentEntryIndex {
        let new_entry = self.new_entry();
        self[new_entry].enclosing = Some(enclosing);
        new_entry
    }

    pub fn remove_entry(&mut self, env_id: EnvironmentEntryIndex) {
        self.entries.remove(env_id);
    }

    pub fn is_defined(&self, env_id: EnvironmentEntryIndex, name: String) -> bool {
        if self[env_id].values.contains_key(&name) {
            return true;
        }
        if let Some(enclosing) = self[env_id].enclosing {
            return self.is_defined(enclosing, name);
        }
        false
    }

    pub fn update_value<Closure>(
        &mut self,
        env_id: EnvironmentEntryIndex,
        name: &str,
        arena: &mut Arena<TypedValue>,
        closure: Closure,
    ) -> Result<(), LangError>
    where
        Closure: FnOnce(&mut TypedValue) -> Result<(), LangError>,
    {
        if let Some(value) = self[env_id].values.get(name) {
            let value_entry = &mut arena[*value];
            let value = match value_entry {
                ArenaEntry::Occupied(v) => v,
                ArenaEntry::Emtpy => panic!(),
            };
            debug!(
                "{}:{} Updating value in env with identifier '{}'",
                file!(),
                line!(),
                name
            );
            closure(value)?;
        } else {
            return Err(Lang::error_s(
                name,
                &format!("Tried to update an undefined identifier: '{}'", name),
            ));
        }
        Ok(())
    }
}
