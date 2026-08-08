/// Lint rule result.
///
/// @brief Result of applying a lint rule
/// @since 0.1.0
/// @author Miruamel
#[derive(Debug, Clone)]
pub struct LintRuleResult {
    pub passed: bool,
    pub message: String,
}

impl LintRuleResult {
    /// Create a passing result.
    ///
    /// @param message Result message
    /// @return LintRuleResult instance
    /// @since 0.1.0
    pub fn pass(message: impl Into<String>) -> Self {
        Self {
            passed: true,
            message: message.into(),
        }
    }

    /// Create a failing result.
    ///
    /// @param message Failure message
    /// @return LintRuleResult instance
    /// @since 0.1.0
    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: message.into(),
        }
    }
}
