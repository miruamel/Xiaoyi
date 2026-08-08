use std::collections::HashMap;

/// Stores OpenAPI schemas by name.
///
/// @brief OpenAPI schema store
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::tools::openapi
#[derive(Debug, Clone, Default)]
pub struct SchemaStore {
    pub schemas: HashMap<String, String>,
}

impl SchemaStore {
    /// Register a schema.
    ///
    /// @brief Add schema to store
    /// @param name Schema name
    /// @param schema Schema JSON
    /// @since 0.1.0
    pub fn register(&mut self, name: impl Into<String>, schema: impl Into<String>) {
        self.schemas.insert(name.into(), schema.into());
    }

    /// Retrieve a schema by name.
    ///
    /// @brief Get schema from store
    /// @param name Schema name
    /// @return Schema JSON if found
    /// @since 0.1.0
    pub fn get(&self, name: &str) -> Option<&String> {
        self.schemas.get(name)
    }
}
