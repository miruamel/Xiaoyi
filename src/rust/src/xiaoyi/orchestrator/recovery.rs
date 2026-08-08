use crate::xiaoyi::core::error::XiaoyiError;

/// Classifies whether an error is retryable.
///
/// @brief Determine if error is retryable
/// @param error Error to classify
/// @return True if retry is appropriate
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::resilience
pub fn is_retryable(error: &XiaoyiError) -> bool {
    matches!(
        error.kind,
        crate::xiaoyi::core::error::ErrorKind::Tool
            | crate::xiaoyi::core::error::ErrorKind::Workflow
            | crate::xiaoyi::core::error::ErrorKind::Llm
    )
}
