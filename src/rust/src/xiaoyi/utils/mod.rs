//! # Utils Module
//!
//! `utils` provides common helper utilities used across Xiaoyi layers.
//!
//! Path: `xiaoyi::utils`
//!
//! - Layer 1: `id` — identifier generation.
//! - Layer 2: `time` — time formatting and duration helpers.
//! - Layer 3: `string` — string manipulation utilities.
//!
//! @module xiaoyi::utils
//! @brief Common helper utilities for Xiaoyi
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi::core
pub mod id;
pub mod string;
pub mod time;

pub use id::generate_id;
pub use string::{slugify, truncate};
pub use time::{format_duration, now_millis};
