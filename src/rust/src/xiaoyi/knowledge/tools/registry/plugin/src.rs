/// Plugin source descriptor.
///
/// @brief Plugin source metadata
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::knowledge::tools::registry::plugin
#[derive(Debug, Clone)]
pub struct PluginSource {
    pub name: String,
    pub version: String,
    pub source_url: String,
}
