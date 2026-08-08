use std::env;

/// Reads an environment variable with a default fallback.
///
/// @brief Get env var with default
/// @param key Variable name
/// @param default Default value if not set
/// @return Variable value or default
/// @since 0.1.0
/// @author Miruamel
pub fn get_env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Reads an environment variable as an integer with a default fallback.
///
/// @brief Get env var as integer with default
/// @param key Variable name
/// @param default Default value if not set or invalid
/// @return Parsed integer or default
/// @since 0.1.0
/// @author Miruamel
pub fn get_env_int_or(key: &str, default: i64) -> i64 {
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
