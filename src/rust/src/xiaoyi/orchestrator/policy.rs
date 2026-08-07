//! # Orchestrator Policy Module
//!
//! `policy` defines decision policies for the orchestrator.
//!
//! Path: `xiaoyi::orchestrator::policy`
//!
//! @module orchestrator::policy
//! @brief Decision policies for agent orchestration
//! @group Agent Runtime
//! @since 0.1.0
//! @author Miruamel
//! @see crate::orchestrator

/// Execution policy.
///
/// @brief Policy for agent decisions
/// @group Agent Runtime
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Policy {
    pub max_iterations: usize,
    pub timeout_ms: u64,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            timeout_ms: 30000,
        }
    }
}

/// Check if execution should continue.
///
/// @param state Current loop state
/// @param policy Execution policy
/// @return Continue flag
/// @since 0.1.0
pub fn should_continue(
    state: &crate::xiaoyi::orchestrator::loop_::LoopState,
    policy: &Policy,
) -> bool {
    state.iterations < policy.max_iterations
}
