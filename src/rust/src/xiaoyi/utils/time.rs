use std::time::Duration;

/// Formats a duration in a human-readable form.
///
/// @brief Format duration as human-readable string
/// @param duration Time duration to format
/// @return Formatted duration string
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::utils
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3600 {
        let mins = secs / 60;
        let rem = secs % 60;
        format!("{mins}m {rem}s")
    } else {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        format!("{hours}h {mins}m")
    }
}

/// Returns the current Unix timestamp in milliseconds.
///
/// @brief Current Unix epoch time in milliseconds
/// @return Milliseconds since 1970-01-01T00:00:00Z
/// @since 0.1.0
/// @author Miruamel
pub fn now_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
