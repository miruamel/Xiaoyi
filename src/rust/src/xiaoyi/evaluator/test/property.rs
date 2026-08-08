/// Property-based test configuration.
///
/// @brief Property testing settings
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::evaluator::test
#[derive(Debug, Clone)]
pub struct PropertyTestConfig {
    pub max_cases: u32,
    pub max_shrinks: u32,
}

impl Default for PropertyTestConfig {
    fn default() -> Self {
        Self {
            max_cases: 100,
            max_shrinks: 10,
        }
    }
}
