//! # Node.js Bindings (napi-rs)
//!
//! `nodejs_bindings` provides Node.js/TypeScript bindings via napi-rs.
//!
//! Path: `xiaoyi::nodejs_bindings`
//!
//! - Layer 0: `nodejs_bindings` — Node.js bindings.
//!
//! @module nodejs_bindings
//! @brief Node.js bindings via napi-rs
//! @group Bindings
//! @since 0.1.0
//! @author Miruamel
//! @see https://github.com/napi-rs/napi-rs

#[cfg(feature = "nodejs")]
use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Initialize the Node.js module.
///
/// @return NAPI result
/// @since 0.1.0
#[cfg(feature = "nodejs")]
#[napi]
pub fn init() -> napi::Result<()> {
    // Export types and functions
    Ok(())
}

/// Xiaoyi error for Node.js.
///
/// @brief Error class for Node.js
/// @group Bindings
/// @since 0.1.0
#[cfg(feature = "nodejs")]
#[napi(object)]
pub struct XiaoyiError {
    pub kind: String,
    pub message: String,
}

#[cfg(feature = "nodejs")]
impl napi::bindgen_prelude::ObjectFinalize for XiaoyiError {}

#[cfg(feature = "nodejs")]
#[napi]
impl XiaoyiError {
    /// Create new error.
    ///
    /// @param kind Error kind
    /// @param message Error message
    /// @return XiaoyiError
    /// @since 0.1.0
    #[napi(constructor)]
    pub fn new(kind: String, message: String) -> Self {
        Self { kind, message }
    }
}