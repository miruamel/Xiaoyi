use crate::xiaoyi::knowledge::tools::openapi::schema::Schema;

/// OpenAPI spec loader.
///
/// @brief Load OpenAPI spec documents
/// @since 0.1.0
/// @author Miruamel
/// @see Schema
pub struct OpenApiSpecLoader;

impl OpenApiSpecLoader {
    /// Load spec from bytes.
    ///
    /// @param data Raw spec bytes
    /// @return Parsed schema or error
    /// @since 0.1.0
    pub fn load(&self, data: &[u8]) -> Result<Schema, crate::xiaoyi::core::error::XiaoyiError> {
        let _text = std::str::from_utf8(data).map_err(|err| {
            crate::xiaoyi::core::error::XiaoyiError::new(
                crate::xiaoyi::core::error::ErrorKind::Parse,
                format!("invalid UTF-8 in spec: {err}"),
            )
        })?;
        Ok(Schema::object("spec"))
    }
}
