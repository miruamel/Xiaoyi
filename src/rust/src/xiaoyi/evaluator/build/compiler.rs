/// Compiler toolchain configuration.
///
/// @brief Compiler settings for build verification
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::evaluator::build
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    pub toolchain: String,
    pub features: Vec<String>,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            toolchain: "stable".into(),
            features: Vec::new(),
        }
    }
}
