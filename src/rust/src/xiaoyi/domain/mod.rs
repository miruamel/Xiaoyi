//! # Domain Primitives
//!
//! `domain` provides fundamental token primitives and syntax definitions
//! for the Xiaoyi type system.
//!
//! Path: `xiaoyi::domain`
//!
//! - Layer 0: `domain` — domain primitives.
//! - Layer 1: `token` — token representation.
//! - Layer 2: `primitive` — primitive types (int, float, bool, string).
//! - Layer 3: `syntax` — syntax-level tokens.
//! - Layer 4-5: kind/width/rep/normalize — type details.
//!
//! @module domain
//! @brief Fundamental token primitives and syntax
//! @group Domain
//! @since 0.1.0
//! @author Miruamel
//! @see crate::domain::token
//! @see crate::lexer
pub mod token;