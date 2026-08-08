use regex::Regex;

/// Formats generated code with basic cleanup.
///
/// @brief Basic code formatter
/// @param code Generated code
/// @return Formatted code
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::builder
pub fn format_code(code: &str) -> String {
    let trimmed = code.trim();
    let re = Regex::new(r"\n{3,}").unwrap();
    re.replace_all(trimmed, "\n\n").into_owned()
}
