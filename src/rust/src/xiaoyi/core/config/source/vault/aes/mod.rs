//! # AES Primitives
//!
//! `aes` provides low-level AES utilities for vault encryption.
//!
//! Path: `xiaoyi::core::config::source::vault::aes`
//!
//! - Layer 0: `core`
//! - Layer 1: `config`
//! - Layer 2: `source`
//! - Layer 3: `vault`
//! - Layer 4: `aes`
//!
//! @module core::config::source::vault::aes
//! @brief Low-level AES utilities
//! @group Core Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::core::config::source::vault::encrypt
//! @see crate::core::config::source::vault::decrypt
//!
//! # Note
//!
//! This module re-exports AES-GCM types for convenience.
//! Direct use is discouraged; prefer `encrypt`/`decrypt` modules.
pub use aes_gcm::aead::{Aead, KeyInit};
pub use aes_gcm::{Aes256Gcm, Key, Nonce};
