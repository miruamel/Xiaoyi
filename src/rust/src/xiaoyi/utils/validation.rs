/// Validates that a string is non-empty after trimming.
///
/// @brief Check if string is non-empty
/// @param input Input string
/// @return True if string contains non-whitespace content
/// @since 0.1.0
/// @author Miruamel
pub fn is_non_empty_string(input: &str) -> bool {
    !input.trim().is_empty()
}

/// Validates that a string looks like an HTTP/HTTPS URL.
///
/// @brief Check if string is a valid URL
/// @param input Input string
/// @return True if string starts with http:// or https://
/// @since 0.1.0
/// @author Miruamel
pub fn is_url(input: &str) -> bool {
    input.starts_with("http://") || input.starts_with("https://")
}

/// Validates that a string is a valid semantic version.
///
/// @brief Check if string is a valid semver
/// @param input Input string
/// @return True if string matches semver pattern
/// @since 0.1.0
/// @author Miruamel
pub fn is_semver(input: &str) -> bool {
    let parts: Vec<&str> = input.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok())
}
