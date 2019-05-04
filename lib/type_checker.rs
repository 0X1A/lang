use crate::error::*;
use crate::value::TypedValue;

pub struct TypeChecker;

impl TypeChecker {
    /// Checks if `lhs` and `rhs` are of the same type by comparing their type annotations.
    pub fn check_type(lhs: &TypedValue, rhs: &TypedValue) -> Result<(), LangError> {
        if lhs.value_type != rhs.value_type {
            Err(LangError::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "expected type {}, found {}",
                        lhs.value_type.to_string(),
                        rhs.value_type.to_string()
                    ),
                },
            ))
        } else {
            Ok(())
        }
    }
}
