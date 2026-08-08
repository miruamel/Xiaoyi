//! # Module: knowledge::tools::openapi::schema
//!
//! @module knowledge::tools::openapi::schema
//! @brief Schema types for describing tool inputs and outputs.
//! @group Knowledge
//! @since 0.1.0
//! @author Miruamel
//! @see crate::knowledge::tools::openapi
//!
//! This module defines a simple schema language for tool parameters:
//! `SchemaKind` enumerates JSON‑like types, `SchemaField` attaches a name and
//! required flag to a kind, and `Schema` groups fields under a name and kind.
//! The builder pattern (`Schema::object(...).field(...)`) allows constructing
//! schemas in a readable way.
//! @example
//! use xiaoyi::knowledge::tools::openapi::schema::{Schema, SchemaKind};
//! let schema = Schema::object("my-tool")
//!     .field("count", SchemaKind::Number, true)
//!     .field("enabled", SchemaKind::Boolean, false);

/// Enumeration of JSON‑like types that can appear in a tool's schema.
///
/// These variants correspond to the basic JSON value types, with `Ref`
/// representing a reference to an external schema definition.
///
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone, PartialEq)]
pub enum SchemaKind {
    /// Object type – a JSON object with key‑value pairs.
    Object,
    /// Array type – a JSON array.
    Array,
    /// String type – a JSON string.
    String,
    /// Number type – a JSON number (integer or floating‑point).
    Number,
    /// Boolean type – a JSON boolean.
    Boolean,
    /// Reference to an external schema definition.
    Ref(String),
}

/// A single field in a tool's input or output schema.
///
/// `SchemaField` describes a property of a JSON object, identified by its name,
/// type, and whether it is required. This is used within `Schema` to define the
/// structure of a tool's parameters.
///
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct SchemaField {
    /// Human‑readable name of the field.
    pub name: String,
    /// The JSON‑like type of the field.
    pub ty: SchemaKind,
    /// Whether the field is required (present in JSON Schema `required` list).
    pub required: bool,
}

/// A schema describing a tool's input or output parameters.
///
/// `Schema` is a builder‑friendly representation of a JSON Schema fragment.
/// It includes a name, a kind (e.g., `Object`), and a list of fields that define
/// the structure of the data.
///
/// The `object` constructor is provided as a convenience for creating object
/// schemas, which are the most common type for tool parameters.
///
/// @group Knowledge
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct Schema {
    /// Unique identifier for the schema (e.g., tool name).
    pub name: String,
    /// The overall kind of the schema.
    pub kind: SchemaKind,
    /// The list of fields that compose the schema, when `kind` is `Object`.
    pub fields: Vec<SchemaField>,
}

impl Schema {
    /// Create a new object‑type schema.
    ///
    /// # Arguments
    /// * `name` - The name of the schema (typically the tool's name).
    ///
    /// # Returns
    /// A `Schema` instance with `kind` set to `SchemaKind::Object` and an empty
    /// fields vector.
    pub fn object(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: SchemaKind::Object,
            fields: Vec::new(),
        }
    }

    /// Add a field to this schema.
    ///
    /// This method uses the builder pattern: it takes `self` by value, adds the
    /// field to `fields`, and returns `Self` to allow chaining.
    ///
    /// # Arguments
    /// * `name` - The name of the field.
    /// * `ty` - The `SchemaKind` of the field.
    /// * `required` - Whether the field is required.
    ///
    /// # Returns
    /// The same `Schema` instance with the new field appended.
    pub fn field(mut self, name: impl Into<String>, ty: SchemaKind, required: bool) -> Self {
        self.fields.push(SchemaField {
            name: name.into(),
            ty,
            required,
        });
        self
    }
}
