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
pub mod env;
pub mod fs;
pub mod id;
pub mod json;
pub mod math;
pub mod net;
pub mod retry;
pub mod string;
pub mod time;
pub mod validation;

pub use env::{get_env_int_or, get_env_or};
pub use fs::{canonicalize, read_file, write_file};
pub use id::generate_id;
pub use json::{from_json, get_string_field, is_json_object, to_json};
pub use math::{clamp, lerp};
pub use net::HttpClientConfig;
pub use retry::RetryConfig;
pub use string::{slugify, truncate};
pub use time::{format_duration, now_millis};
pub use validation::{is_non_empty_string, is_semver, is_url};
