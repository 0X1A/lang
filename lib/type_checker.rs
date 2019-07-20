use crate::error::*;
use crate::token::*;

pub struct TypeChecker;

impl TypeChecker {
    /// Checks if `lhs` and `rhs` are of the same type by comparing their type annotations.
    pub fn check_type(lhs: &GetTypeAnnotation, rhs: &GetTypeAnnotation) -> Result<(), LangError> {
        if lhs.get_type_annotation() != rhs.get_type_annotation() {
            Err(LangErrorType::new_runtime_error(
                RuntimeErrorType::InvalidTypeAssignmentError {
                    reason: format!(
                        "expected type {}, found {}",
                        lhs.get_type_annotation().to_string(),
                        rhs.get_type_annotation().to_string()
                    ),
                },
            ))
        } else {
            Ok(())
        }
    }

    pub fn can_convert_implicitly(lhs: &GetTypeAnnotation, rhs: &GetTypeAnnotation) -> bool {
        let lhs_type_annotation = lhs.get_type_annotation();
        let rhs_type_annotation = rhs.get_type_annotation();
        match lhs_type_annotation {
            TypeAnnotation::I64 => match rhs_type_annotation {
                TypeAnnotation::I32 => true,
                _ => false,
            },
            TypeAnnotation::F64 => match rhs_type_annotation {
                TypeAnnotation::F32 => true,
                _ => false,
            },
            TypeAnnotation::Array(lhs_element_type) => match rhs_type_annotation {
                TypeAnnotation::Array(rhs_element_type) => {
                    let a = lhs_element_type;
                    return TypeChecker::can_convert_implicitly(&**a, &**rhs_element_type);
                }
                _ => false,
            },
            _ => false,
        }
    }
}
