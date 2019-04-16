use error::*;
use value::TypedValue;

pub struct TypeChecker;

impl TypeChecker {
    /// Checks if `lhs` and `rhs` are of the same type by comparing their type annotations.
    pub fn check_type(lhs: &TypedValue, rhs: &TypedValue) -> Result<(), LangError> {
        if lhs.value_type != rhs.value_type {
            Err(LangError::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "tried to assign value of type {} to value of type {}",
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
