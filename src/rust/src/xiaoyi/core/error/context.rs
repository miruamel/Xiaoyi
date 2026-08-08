use crate::xiaoyi::core::error::XiaoyiError;

/// Adds structured context to an existing XiaoyiError.
///
/// @brief Attach metadata context to an error
/// @param error Base error
/// @param context Key-value metadata
/// @return Enriched error with context
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::core::error
pub fn with_context(error: XiaoyiError, context: serde_json::Value) -> XiaoyiError {
    let mut out = error;
    if let Some(obj) = context.as_object() {
        for (k, v) in obj {
            out = out.with_meta(k.clone(), v.to_string());
        }
    }
    out
}
