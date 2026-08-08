//! # Critic Plant Module
//!
//! `critic` provides the AI Cascadic Critic Plant for multi-stage code review.
//!
//! Path: `xiaoyi::critic`
//!
//! - Layer 0: `critic` — Critic plant layer.
//! - Layer 1: `rules` — Fast-path rules engine (linters/regex).
//! - Layer 2: `small_llm` — Small LLM critics (style, doc, accessibility).
//! - Layer 3: `large_llm` — Large LLM critics (security, architecture, complex logic).
//! - Layer 4: `router` — Model router (Heavy/Light/Micro).
//! - Layer 5: `aggregator` — Meta-critic aggregator.
//! - Layer 6: `cache` — Semantic cache with vector DB.
//!
//! @module critic
//! @brief AI Cascadic Critic Plant for multi-stage code review
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::builder
//! @see crate::evaluator
//!
//! # Example
//!
//! ```rust
//! use xiaoyi::critic::CriticPlant;
//! use xiaoyi::critic::rules::RulesEngine;
//! use xiaoyi::critic::aggregator::Aggregator;
//!
//! let plant = CriticPlant::new();
//! let result = plant.review(code).await?;
//! ```
pub mod aggregator;
pub mod cache;
pub mod large_llm;
pub mod model_router;
pub mod rules;
pub mod small_llm;

use crate::xiaoyi::core::error::Result;

/// Critic Plant orchestrates multi-stage code review.
///
/// @brief Multi-stage AI code review pipeline
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct CriticPlant {
    // Configuration for the critic plant
}

impl CriticPlant {
    /// Create a new critic plant.
    ///
    /// @return CriticPlant instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {}
    }

    /// Run the full critic pipeline on code.
    ///
    /// @param code Source code to review
    /// @return Review result with findings
    /// @since 0.1.0
    pub async fn review(&self, _code: &str) -> Result<ReviewResult> {
        // Fast-path rules engine
        // Small LLM critics
        // Large LLM critics
        // Aggregation
        Ok(ReviewResult::default())
    }
}

impl Default for CriticPlant {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a code review.
///
/// @brief Aggregated review findings
/// @group AI Review
/// @since 0.1.0
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewResult {
    /// Findings from rules engine
    pub rule_findings: Vec<RuleFinding>,
    /// Findings from small LLM critics
    pub small_llm_findings: Vec<SmallLlmFinding>,
    /// Findings from large LLM critics
    pub large_llm_findings: Vec<LargeLlmFinding>,
    /// Aggregated score (0.0 - 1.0)
    pub score: f32,
}

/// Finding from rules engine.
///
/// @brief Rule-based finding
/// @group AI Review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleFinding {
    /// Rule identifier
    pub rule_id: String,
    /// Severity level
    pub severity: Severity,
    /// Message
    pub message: String,
    /// File path
    pub file: Option<String>,
    /// Line number
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
}

/// Finding from small LLM critic.
///
/// @brief Small LLM finding
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallLlmFinding {
    /// Critic name
    pub critic: String,
    /// Finding description
    pub finding: String,
    /// Confidence score
    pub confidence: f32,
}

/// Finding from large LLM critic.
///
/// @brief Large LLM finding
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeLlmFinding {
    /// Critic name
    pub critic: String,
    /// Finding description
    pub finding: String,
    /// Confidence score
    pub confidence: f32,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Severity levels for findings.
///
/// @brief Finding severity classification
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}