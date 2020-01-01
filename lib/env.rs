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

#[derive(Clone, Debug)]
pub struct EnvironmentId {
    pub index: usize,
}

// TODO: Generationl indices, memory management. Memory management will likely come along with
// a VM
#[derive(Clone, Debug)]
pub struct EnvironmentEntry {
    pub values_two: HashMap<String, ArenaEntryIndex>,
    pub enclosing: Option<EnvironmentId>,
}

pub struct Environment {
    pub root_entry_id: EnvironmentId,
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
                    .values_two
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
            root_entry_id: EnvironmentId { index: 0 },
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
    pub fn new() -> Environment {
        let mut env = Environment {
            root_entry_id: EnvironmentId { index: 0 as usize },
            entries: Vec::new(),
        };
        env.root_entry_id = EnvironmentId {
            index: env.entries.len(),
        };
        env.entries.push(EnvironmentEntry {
            values_two: HashMap::new(),
            enclosing: None,
        });
        env
    }
    pub fn new_entry(&mut self) -> EnvironmentId {
        let env_id = self.entries.len();
        self.entries.push(EnvironmentEntry {
            values_two: HashMap::new(),
            enclosing: None,
        });
        EnvironmentId { index: env_id }
    }

    pub fn get_at_two(&mut self, index: usize, name: &str) -> Result<ArenaEntryIndex, LangError> {
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
                env_entry.values_two.get(name).map_or(
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

    pub fn get_two(
        &self,
        env_id: &EnvironmentId,
        name: &str,
    ) -> Result<ArenaEntryIndex, LangError> {
        if self[env_id].values_two.contains_key(name) {
            return Ok(self[env_id].values_two[name]);
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            return Ok(self.get_two(&enclosing, name)?);
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangErrorType::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("(get_two) Tried to get a variable: '{}'", name),
            },
        ))
    }

    pub fn define_two(
        &mut self,
        env_id: &EnvironmentId,
        arena: &mut Arena<TypedValue>,
        name: &str,
        value: TypedValue,
    ) {
        debug!(
            "{}:{} Defining '{}' with value '{:?}' at index '{}'",
            file!(),
            line!(),
            name,
            value,
            env_id.index
        );
        let index = arena.insert(value);
        self[env_id].values_two.insert(name.to_string(), index);
    }

    pub fn assign_two(
        &mut self,
        env_id: &EnvironmentId,
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
            env_id.index
        );
        if self[env_id].values_two.contains_key(name) {
            if let Some(existing_value_index) = self[env_id].values_two.get(name) {
                let existing_value_entry = &mut arena[*existing_value_index];
                let existing_value: &mut TypedValue = existing_value_entry.try_into()?;
                if !TypeChecker::can_convert_implicitly(existing_value, &value) {
                    TypeChecker::check_type(existing_value, &value)?;
                }
                *existing_value = value;
            }
            return Ok(());
        } else if let Some(enclosing) = self[env_id].enclosing.clone() {
            self.assign_two(&enclosing, name, value, arena)?;
            return Ok(());
        }
        // We error when an assignment is attempted on a variable that hasn't been instantiated
        Err(LangErrorType::new_runtime_error(
            RuntimeErrorType::UndefinedVariable {
                reason: format!("tried to assign an undefined variable: '{}'", name),
            },
        ))
    }

    pub fn assign_index_entry_two(
        &self,
        env_id: &EnvironmentId,
        name: &str,
        value: &TypedValue,
        arena: &mut Arena<TypedValue>,
        index: usize,
    ) -> Result<(), LangError> {
        if let Some(arr_value_index) = self[env_id].values_two.get(name) {
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

    pub fn entry_from(&mut self, enclosing: &EnvironmentId) -> EnvironmentId {
        let new_entry = self.new_entry();
        self[&new_entry].enclosing = Some(enclosing.clone());
        new_entry
    }

    pub fn remove_entry(&mut self, env_id: &EnvironmentId) {
        self.entries.remove(env_id.index);
    }

    pub fn is_defined_two(&self, env_id: &EnvironmentId, name: String) -> bool {
        if self[env_id].values_two.contains_key(&name) {
            return true;
        }
        if let Some(ref enclosing) = self[env_id].enclosing {
            return self.is_defined_two(enclosing, name);
        }
        false
    }

    pub fn update_value_two<Closure>(
        &mut self,
        env_id: &EnvironmentId,
        name: &str,
        arena: &mut Arena<TypedValue>,
        closure: Closure,
    ) -> Result<(), LangError>
    where
        Closure: FnOnce(&mut TypedValue) -> Result<(), LangError>,
    {
        if let Some(value) = self[env_id].values_two.get(name) {
            let value_entry = &mut arena[*value];
            let value = match value_entry {
                ArenaEntry::Occupied(v) => v,
                ArenaEntry::Emtpy => panic!(),
            };
            closure(value)?;
        } else {
            debug!("{}:{} Didn't find the thing :O", file!(), line!());
        }
        Ok(())
    }
}
