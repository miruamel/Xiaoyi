/// Container runtime configuration.
///
/// @brief Container sandbox configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::evaluator::sandbox
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub image: String,
    pub memory_mb: u64,
    pub cpu_cores: u32,
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            image: "rust:1.99-nightly".into(),
            memory_mb: 512,
            cpu_cores: 1,
        }
    }
}
