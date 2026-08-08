//! # Module: knowledge::tools::openapi::openapi
//!
//! @module knowledge::tools::openapi::openapi
//! @brief OpenAPI specification storage and query utilities.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools::openapi
//!
//! The `OpenApiStore` struct provides a convenient interface for loading, storing,
//! and querying OpenAPI specifications. It is designed to support tool discovery
//! and introspection by exposing methods to list available endpoints, retrieve
//! operation details, and enumerate stored specifications.
//! @example
//! use xiaoyi::knowledge::tools::openapi::{OpenApiStore, Schema};
//! let store = OpenApiStore::default();
//! store.load("my-tool", "{\"openapi\": \"3.0.0\", \"info\": {\"title\": \"...\"}}")?;
//! let names = store.names();


use indexmap::IndexMap;
use parking_lot::RwLock;

use serde_json;
use crate::xiaoyi::core::error::Result;

/// A store for OpenAPI specifications that enables tool discovery and introspection.
///
/// `OpenApiStore` maintains a thread‑safe collection of OpenAPI specifications,
/// each keyed by a user‑friendly name. The store provides methods to load raw JSON
/// specifications, enumerate stored specifications, retrieve endpoints for a given
/// specification, and query individual operations.
///
/// The internal storage uses an `IndexMap` to preserve insertion order, which is
/// useful for deterministic iteration over stored specifications.
///
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Default)]
pub struct OpenApiStore {
    /// Map from specification name to the raw JSON representation.
    specs: RwLock<IndexMap<String, serde_json::Value>>,
}

impl OpenApiStore {
    /// Create a new, empty OpenAPI store.
    ///
    /// # Returns
    /// A new `OpenApiStore` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load an OpenAPI specification into the store.
    ///
    /// The specification is parsed from a JSON string using `serde_json::from_str`
    /// and stored under the given name. If the name already exists, the previous
    /// specification is overwritten.
    ///
    /// # Arguments
    /// * `name` - A user‑friendly identifier for the specification.
    /// * `json` - A JSON string containing the OpenAPI specification.
    ///
    /// # Returns
    /// `Ok(())` on successful parsing and storage, or an error if the JSON is
    /// invalid or parsing fails.
    ///
    /// # Errors
    /// Returns `serde_json::Error` if the input string is not valid JSON.
    pub fn load(&self, name: impl Into<String>, json: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(json)?;
        let mut specs = self.specs.write();
        specs.insert(name.into(), value);
        Ok(())
    }

    /// Retrieve all endpoint paths for a given specification.
    ///
    /// This method walks the `paths` object within the OpenAPI specification and
    /// collects all HTTP methods and paths into a list of tuples.
    ///
    /// # Arguments
    /// * `name` - The name of the specification to query.
    ///
    /// # Returns
    /// A `Vec<(String, String)>` where each tuple contains `(method, path)`.
    /// Returns an empty vector if the specification does not exist or has no
    /// `paths` field.
    pub fn endpoints(&self, name: &str) -> Vec<(String, String)> {
        let specs = self.specs.read();
        let spec = match specs.get(name) {
            Some(s) => s,
            None => return Vec::new(),
        };
        let paths = match spec.get("paths").and_then(|v| v.as_object()) {
            Some(p) => p,
            None => return Vec::new(),
        };
        let mut result = Vec::new();

        for (path, methods) in paths {
            if let Some(methods_obj) = methods.as_object() {
                for (method, _) in methods_obj {
                    result.push((method.clone(), path.clone()));
                }
            }
        }

        result
    }

    /// Retrieve the details of a specific operation within a specification.
    ///
    /// This method looks up the operation defined by `method` and `path` within the
    /// OpenAPI specification identified by `name`. It returns the operation object
    /// as a `serde_json::Value` if found.
    ///
    /// # Arguments
    /// * `name` - The name of the specification to query.
    /// * `method` - The HTTP method (e.g., "get", "post").
    /// * `path` - The path template (e.g., "/users/{id}").
    ///
    /// # Returns
    /// `Some(serde_json::Value)` containing the operation object if the operation
    /// exists, otherwise `None`.
    pub fn operation(&self, name: &str, method: &str, path: &str) -> Option<serde_json::Value> {
        let specs = self.specs.read();
        let spec = specs.get(name)?;
        let paths = spec.get("paths")?.as_object()?;
        let methods = paths.get(path)?.as_object()?;
        methods.get(method).cloned()
    }

    /// List all specification names currently stored.
    ///
    /// # Returns
    /// A `Vec<String>` containing the names of all stored OpenAPI specifications,
    /// in the order they were inserted (preserved by `IndexMap`).
    pub fn names(&self) -> Vec<String> {
        let specs = self.specs.read();
        specs.keys().cloned().collect()
    }
}