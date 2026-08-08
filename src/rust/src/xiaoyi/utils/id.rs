use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Generates a unique identifier.
///
/// @brief Create a timestamp-based unique id
/// @return Hex-encoded id combining timestamp and UUID
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::utils::time
pub fn generate_id() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let uuid = Uuid::new_v4();
    format!("{ts:016x}-{uuid}")
}
