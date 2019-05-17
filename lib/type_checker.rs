use crate::error::*;
use crate::token::*;

pub struct TypeChecker;

impl TypeChecker {
    /// Checks if `lhs` and `rhs` are of the same type by comparing their type annotations.
    pub fn check_type(lhs: &GetTypeAnnotation, rhs: &GetTypeAnnotation) -> Result<(), LangError> {
        if lhs.get() != rhs.get() {
            Err(LangError::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "expected type {}, found {}",
                        lhs.get().to_string(),
                        rhs.get().to_string()
                    ),
                },
            ))
        } else {
            Ok(())
        }
    }
}
