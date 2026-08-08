use std::collections::HashMap;

/// Tool discovery registry.
///
/// @brief Discover available tools
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::tools::registry
#[derive(Debug, Clone, Default)]
pub struct ToolDiscovery {
    pub tools: HashMap<String, String>,
}

impl ToolDiscovery {
    /// List registered tool names.
    ///
    /// @return Tool names
    /// @since 0.1.0
    pub fn list(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Register a tool.
    ///
    /// @param name Tool name
    /// @param description Tool description
    /// @since 0.1.0
    pub fn register(&mut self, name: impl Into<String>, description: impl Into<String>) {
        self.tools.insert(name.into(), description.into());
    }
}
