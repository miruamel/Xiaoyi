//! # Module: knowledge::tools::registry::registry
//!
//! @module knowledge::tools::registry::registry
//! @brief Core registry for tool plugins in Xiaoyi.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools::registry
//!
//! This module defines the `ToolRegistry` struct, which provides thread-safe
//! storage and management of tool plugins. Plugins are keyed by their name to
//! ensure uniqueness, and the registry offers methods to register, retrieve,
//! list, and invoke plugins.
//! @example
//! use xiaoyi::knowledge::tools::registry::{ToolRegistry, ToolPlugin};
//! let registry = ToolRegistry::default();
//! let plugin = ToolPlugin::new(...);
//! registry.register(plugin)?;
//! let result = registry.invoke("my-tool", input)?;

use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::xiaoyi::core::error::{ErrorKind, XiaoyiError};
use crate::xiaoyi::knowledge::tools::registry::plugin::ToolPlugin;

/// A thread‑safe registry for tool plugins.
///
/// `ToolRegistry` stores `ToolPlugin` instances keyed by their name. It provides
/// methods to register new plugins (ensuring uniqueness), retrieve a plugin by name,
/// list all plugin names, and invoke a plugin with JSON input. All access to the
/// underlying storage is protected by a `RwLock` to allow safe concurrent usage.
///
/// The registry is designed to be the central point of tool discovery and execution
/// within the Xiaoyi runtime.
#[derive(Default)]
pub struct ToolRegistry {
    /// Map from plugin name to the plugin instance. The `IndexMap` preserves
    /// insertion order, which can be useful for deterministic listing.
    tools: RwLock<IndexMap<String, ToolPlugin>>,
}

impl ToolRegistry {
    /// Create a new, empty tool registry.
    ///
    /// # Returns
    /// A new `ToolRegistry` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a tool plugin into the registry.
    ///
    /// If a plugin with the same name already exists, an error is returned.
    ///
    /// # Arguments
    /// * `plugin` - The `ToolPlugin` to register.
    ///
    /// # Returns
    /// `Ok(())` if the plugin was successfully registered.
    ///
    /// # Errors
    /// Returns `XiaoyiError` with kind `ErrorKind::Tool` if a plugin with the
    /// same name already exists.
    pub fn register(&self, plugin: ToolPlugin) -> Result<(), XiaoyiError> {
        let mut map = self.tools.write();
        if map.contains_key(&plugin.name) {
            return Err(XiaoyiError::new(
                ErrorKind::Tool,
                format!("Tool plugin '{}' already registered", plugin.name),
            ));
        }
        map.insert(plugin.name.clone(), plugin);
        Ok(())
    }

    /// Retrieve a tool plugin by name.
    ///
    /// # Arguments
    /// * `name` - The name of the plugin to retrieve.
    ///
    /// # Returns
    /// `Some(ToolPlugin)` if a plugin with the given name exists, otherwise `None`.
    /// Note: The returned plugin is a clone; the registry retains ownership.
    pub fn get(&self, name: &str) -> Option<ToolPlugin> {
        let map = self.tools.read();
        map.get(name).cloned()
    }

    /// List all plugin names currently registered.
    ///
    /// # Returns
    /// A `Vec<String>` containing the names of all plugins in the registry, in the
    /// order they were registered (preserved by `IndexMap`).
    pub fn list(&self) -> Vec<String> {
        let map = self.tools.read();
        map.keys().cloned().collect()
    }

    /// Invoke a tool plugin with the given JSON input.
    ///
    /// If the plugin does not exist, an error is returned. Otherwise, the plugin's
    /// `invoke` method is called with the input, and its result is returned.
    ///
    /// # Arguments
    /// * `name` - The name of the plugin to invoke.
    /// * `input` - The input parameters for the tool, as a JSON value.
    ///
    /// # Returns
    /// `Ok(serde_json::Value)` with the tool's output if the tool executes
    /// successfully.
    ///
    /// # Errors
    /// Returns `XiaoyiError` with kind `ErrorKind::Tool` if:
    /// - The plugin with the given name does not exist.
    /// - The plugin's handler returns an error.
    pub fn invoke(
        &self,
        name: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value, XiaoyiError> {
        let map = self.tools.read();
        let plugin = map.get(name).ok_or_else(|| {
            XiaoyiError::new(ErrorKind::Tool, format!("Tool plugin '{}' not found", name))
        })?;
        plugin.invoke(input)
    }
}
