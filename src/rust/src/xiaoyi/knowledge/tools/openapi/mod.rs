//! # Module: knowledge::tools::openapi
//!
//! @module knowledge::tools::openapi
//! @brief OpenAPI specification support for tool definitions and discovery.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools
//!
//! The OpenAPI module provides types and storage for tool specifications.
//! It includes `Schema` types for describing tool inputs/outputs, and an
//! `OpenApiStore` for loading and querying OpenAPI specifications that define
//! tool interfaces, endpoints, and operations.
//! @example
//! use xiaoyi::knowledge::tools::openapi::{OpenApiStore, Schema, SchemaKind};
//! let store = OpenApiStore::default();
//! store.load("my-tool", "{\"openapi\": \"3.0.0\", ...}")?;

pub mod schema;
pub mod openapi;

pub use schema::{Schema, SchemaKind, SchemaField};
pub use openapi::OpenApiStore;