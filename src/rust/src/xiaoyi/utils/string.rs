/// Converts a string to a URL-friendly slug.
///
/// @brief Convert text to lowercase hyphenated slug
/// @param input Input string
/// @return Slugified string
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::utils
pub fn slugify(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// Truncates a string to `max_len` characters, appending `…` if truncated.
///
/// @brief Truncate string with ellipsis
/// @param input Input string
/// @param max_len Maximum length including ellipsis
/// @return Truncated string
/// @since 0.1.0
/// @author Miruamel
pub fn truncate(input: &str, max_len: usize) -> String {
    if input.chars().count() <= max_len {
        return input.to_string();
    }
    let mut out = String::new();
    for (i, ch) in input.chars().enumerate() {
        if i >= max_len.saturating_sub(1) {
            out.push('…');
            break;
        }
        out.push(ch);
    }
    out
}
