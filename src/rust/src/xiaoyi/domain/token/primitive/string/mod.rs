//! # String Primitive
//!
//! `string` provides UTF-8 string type with encoding validation.
//!
//! Path: `xiaoyi::domain::token::primitive::string`
//!
//! - Layer 0: `domain`
//! - Layer 1: `token`
//! - Layer 2: `primitive`
//! - Layer 3: `string`
//!
//! @module domain::token::primitive::string
//! @brief UTF-8 string primitive
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token::primitive
//! @see crate::domain::token::primitive::int

/// String type alias (owned UTF-8).
///
/// @brief Owned String
/// @group Domain
/// @since 0.1.0
pub type String = std::string::String;

/// String slice type alias (borrowed UTF-8).
///
/// @brief Borrowed str
/// @group Domain
/// @since 0.1.0
pub type Str = str;

/// Create new empty string.
///
/// @return Empty String
/// @since 0.1.0
pub fn new() -> String {
    String::new()
}

/// Create string from string slice.
///
/// @param s String slice
/// @return Owned String
/// @since 0.1.0
pub fn from_str(s: &str) -> String {
    s.to_string()
}

/// Check if string is valid UTF-8.
///
/// @param bytes Byte slice
/// @return true if valid UTF-8
/// @since 0.1.0
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Get string length in characters (grapheme clusters approx).
///
/// @param s String slice
/// @return Character count
/// @since 0.1.0
pub fn char_len(s: &str) -> usize {
    s.chars().count()
}