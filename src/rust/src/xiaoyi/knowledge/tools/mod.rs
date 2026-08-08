//! # Module: knowledge::tools
//!
//! @module knowledge::tools
//! @brief Tool management and execution subsystem for Xiaoyi.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge
//!
//! This module provides a plugin-based tool system for extending Xiaoyi's capabilities
//! with external tools and functions. It includes a registry for managing tools, plugin
//! definitions, and OpenAPI specification support for tool documentation and discovery.
//! @example
//! use xiaoyi::knowledge::tools::{ToolRegistry, ToolPlugin};
//! let registry = ToolRegistry::default();
//! registry.register(plugin)?;
//! @see tools::registry
//! @see tools::openapi

pub mod openapi;
pub mod registry;

pub use openapi::{OpenApiStore, Schema, SchemaKind};
pub use registry::{ToolPlugin, ToolRegistry};
