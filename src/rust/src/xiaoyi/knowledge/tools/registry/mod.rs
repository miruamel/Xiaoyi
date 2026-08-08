//! # Module: knowledge::tools::registry
//!
//! @module knowledge::tools::registry
//! @brief Plugin registry for managing tool plugins in Xiaoyi.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools
//!
//! The registry provides a thread-safe, locking-based storage for `ToolPlugin`
//! instances. Plugins are keyed by their name and can be retrieved, listed,
//! or invoked by the runtime. The registry enforces uniqueness of plugin names
//! to avoid clashes.
//! @example
//! use xiaoyi::knowledge::tools::registry::{ToolRegistry, ToolPlugin};
//! let registry = ToolRegistry::default();
//! registry.register(my_plugin).unwrap();

pub mod plugin;
pub mod registry;

pub use plugin::{ToolHandler, ToolPlugin};
pub use registry::ToolRegistry;
