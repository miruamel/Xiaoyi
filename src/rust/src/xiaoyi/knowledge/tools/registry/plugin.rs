//! # Module: knowledge::tools::registry::plugin
//!
//! @module knowledge::tools::registry::plugin
//! @brief Tool plugin definitions and handler traits for Xiaoyi tools.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools::registry
//!
//! This module defines the `ToolHandler` trait that all tool implementations must
//! satisfy, and the `ToolPlugin` struct which bundles a handler with metadata such
//! as name, version, description, and an input schema. The plugin can be registered
//! into the `ToolRegistry` and later invoked at runtime.
//! @example
//! use xiaoyi::knowledge::tools::registry::plugin::{ToolHandler, ToolPlugin};
//! struct MyHandler;
//! impl ToolHandler for MyHandler {
//!     fn run(&self, input: serde_json::Value) -> Result<serde_json::Value> {
//!         Ok(serde_json::json!({"output": input}))
//!     }
//! }
//! let plugin = ToolPlugin::new(
//!     "my-tool",
//!     "1.0.0",
//!     "A sample tool",
//!     schema,
//!     std::sync::Arc::new(MyHandler),
//! );

use std::fmt;
use std::sync::Arc;

use crate::xiaoyi::core::error::Result;
use serde_json;

/// Trait that all tool handlers must implement.
///
/// A `ToolHandler` provides the core logic of a tool. The `run` method takes a
/// JSON value representing the tool's input and returns a JSON value representing
/// the tool's output, or an error if the tool cannot execute.
///
/// This trait is `Send + Sync` to allow safe sharing across threads and async
/// contexts.
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with the provided JSON input.
    ///
    /// # Arguments
    /// * `input` - A `serde_json::Value` containing the tool's arguments.
    ///
    /// # Returns
    /// A `Result<serde_json::Value>` where `Ok` contains the tool's output JSON
    /// and `Err` contains an error if execution fails.
    fn run(&self, input: serde_json::Value) -> Result<serde_json::Value>;
}

/// A plugin that packages a tool's metadata and its handler implementation.
///
/// `ToolPlugin` serves as a container for a tool's description, version,
/// input schema, and the actual handler function. It is designed to be stored
/// in the `ToolRegistry` and invoked by the Xiaoyi runtime.
///
/// The plugin is not `Debug` by default because it contains an `Arc<dyn ToolHandler>`
/// which cannot be derived. A manual `Debug` implementation is provided that
/// omits the handler but prints the plugin's metadata.
#[derive(Clone)]
pub struct ToolPlugin {
    /// Unique name of the tool (used as the key in the registry).
    pub name: String,
    /// Semantic version of the tool.
    pub version: String,
    /// Human-readable description of the tool.
    pub description: String,
    /// JSON schema that describes the tool's expected input parameters.
    pub input_schema: crate::xiaoyi::knowledge::tools::openapi::Schema,
    /// Thread-safe, reference‑counted pointer to the actual handler implementation.
    pub handler: Arc<dyn ToolHandler>,
}

impl ToolPlugin {
    /// Create a new tool plugin.
    ///
    /// # Arguments
    /// * `name` - The plugin's name (must be unique within the registry).
    /// * `version` - Semantic version string (e.g., "1.0.0").
    /// * `description` - Human‑readable description of the plugin.
    /// * `input_schema` - JSON schema describing the plugin's input parameters.
    /// * `handler` - The tool handler implementation wrapped in an `Arc`.
    ///
    /// # Returns
    /// A new `ToolPlugin` instance.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        input_schema: crate::xiaoyi::knowledge::tools::openapi::Schema,
        handler: Arc<dyn ToolHandler>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            input_schema,
            handler,
        }
    }

    /// Invoke the tool with the provided JSON input.
    ///
    /// This method delegates to the plugin's internal handler by calling its `run`
    /// method. The result (or error) is propagated back to the caller.
    ///
    /// # Arguments
    /// * `input` - The tool's input parameters as a JSON value.
    ///
    /// # Returns
    /// A `Result<serde_json::Value>` containing the tool's output JSON if execution
    /// succeeds, or an error if the tool fails.
    pub fn invoke(&self, input: serde_json::Value) -> Result<serde_json::Value> {
        self.handler.run(input)
    }
}

/// Manual `Debug` implementation for `ToolPlugin`.
///
/// Since `Arc<dyn ToolHandler>` cannot be derived, we manually implement `Debug`
/// to print the plugin's metadata (name, version, description) while omitting the
/// handler from the output.
impl fmt::Debug for ToolPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToolPlugin")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("description", &self.description)
            .finish()
    }
}
