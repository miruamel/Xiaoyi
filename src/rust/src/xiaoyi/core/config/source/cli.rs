use crate::xiaoyi::core::config::{Config, ConfigSource};

/// CLI arguments configuration source.
///
/// @brief Load configuration from CLI args
/// @group Core
/// @since 0.1.0
/// @author Miruamel
/// @see ConfigSource
/// @see Config
#[derive(Debug, Clone, Default)]
pub struct CliConfigSource;

impl ConfigSource for CliConfigSource {
    fn name(&self) -> &str {
        "cli"
    }
}
