use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};

/// Validates that generated code is non-empty.
///
/// @brief Basic generated-code validation
/// @param code Generated code
/// @return Ok or validation error
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::builder
pub fn validate_non_empty(code: &str) -> Result<(), XiaoyiError> {
    if code.trim().is_empty() {
        return Err(XiaoyiError::new(
            ErrorKind::Syntax,
            "generated code is empty",
        ));
    }
    Ok(())
}
