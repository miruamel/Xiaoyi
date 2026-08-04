//! # Layer 1 - Domain / Token Primitive Int Normalize
//!
//! Normalization rules for integer literals, including overflow guards and radix
//! interpretation. This keeps parser complexity isolated from token semantics.
//!
//! Path: `xiaoyi::domain::token::primitive::int::normalize`
//!
//! Layer hierarchy:
//! - 1: `domain`
//! - 2: `token`
//! - 3: `primitive`
//! - 4: `int`
//! - 5: `normalize`

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};

pub fn normalize_int(input: &str, base: u32) -> Result<i64> {
    if base < 2 || base > 36 {
        return Err(XiaoyiError::new(
            ErrorKind::Syntax,
            "integer base must be within 2..=36",
        )
        .with_meta("input", input.to_string())
        .with_meta("base", base.to_string()));
    }

    match i64::from_str_radix(input, base) {
        Ok(value) => Ok(value),
        Err(_) => Err(XiaoyiError::new(
            ErrorKind::Parse,
            "failed to parse integer literal",
        )
        .with_meta("input", input.to_string())
        .with_meta("base", base.to_string())),
    }
}
