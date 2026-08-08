//! # Meta-Critic Aggregator
//!
//! `aggregator` combines findings from all critic stages using
//! weighted Pareto frontier optimization.
//!
//! Path: `xiaoyi::critic::aggregator`
//!
//! @module critic::aggregator
//! @brief Meta-critic aggregator with Pareto frontier
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::critic::rules
//! @see crate::critic::small_llm
//! @see crate::critic::large_llm

use crate::xiaoyi::critic::{
    LargeLlmFinding, ReviewResult, RuleFinding, Severity, SmallLlmFinding,
};

/// Aggregator weights for different critic stages.
///
/// @brief Weight configuration for aggregation
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct AggregatorWeights {
    /// Weight for rules engine findings
    pub rules_weight: f32,
    /// Weight for small LLM findings
    pub small_llm_weight: f32,
    /// Weight for large LLM findings
    pub large_llm_weight: f32,
}

impl Default for AggregatorWeights {
    fn default() -> Self {
        Self {
            rules_weight: 0.2,
            small_llm_weight: 0.3,
            large_llm_weight: 0.5,
        }
    }
}

/// Severity weight mapping.
///
/// @brief Maps severity to numeric weight
/// @group AI Review
/// @since 0.1.0
fn severity_weight(severity: Severity) -> f32 {
    match severity {
        Severity::Info => 0.1,
        Severity::Warning => 0.3,
        Severity::Error => 0.7,
        Severity::Critical => 1.0,
    }
}

/// Meta-critic aggregator.
///
/// @brief Aggregates findings from all critic stages
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Aggregator {
    weights: AggregatorWeights,
}

impl Aggregator {
    /// Create new aggregator.
    ///
    /// @param weights Aggregation weights
    /// @return Aggregator instance
    /// @since 0.1.0
    pub fn new(weights: AggregatorWeights) -> Self {
        Self { weights }
    }

    /// Aggregate all findings into final review result.
    ///
    /// @param rule_findings Findings from rules engine
    /// @param small_llm_findings Findings from small LLM critics
    /// @param large_llm_findings Findings from large LLM critics
    /// @return Aggregated review result
    /// @since 0.1.0
    pub fn aggregate(
        &self,
        rule_findings: Vec<RuleFinding>,
        small_llm_findings: Vec<SmallLlmFinding>,
        large_llm_findings: Vec<LargeLlmFinding>,
    ) -> ReviewResult {
        let mut score = 1.0f32;

        // Calculate weighted penalty from rule findings
        for finding in &rule_findings {
            let penalty = severity_weight(finding.severity) * self.weights.rules_weight;
            score -= penalty * 0.1; // Scale down
        }

        // Calculate weighted penalty from small LLM findings
        for finding in &small_llm_findings {
            let penalty = finding.confidence * self.weights.small_llm_weight;
            score -= penalty * 0.15;
        }

        // Calculate weighted penalty from large LLM findings
        for finding in &large_llm_findings {
            let penalty = finding.confidence * self.weights.large_llm_weight;
            score -= penalty * 0.2;
        }

        // Clamp score to [0, 1]
        score = score.clamp(0.0, 1.0);

        ReviewResult {
            rule_findings,
            small_llm_findings,
            large_llm_findings,
            score,
        }
    }

    /// Get Pareto-optimal subset of findings.
    ///
    /// Finds the Pareto frontier of findings based on severity and confidence.
    ///
    /// @param findings All findings
    /// @return Pareto-optimal findings
    /// @since 0.1.0
    /// Get Pareto-optimal subset of findings.
    ///
    /// Finds the Pareto frontier of findings based on severity and confidence.
    ///
    /// @param findings All findings
    /// @return Pareto-optimal findings (cloned)
    /// @since 0.1.0
    pub fn pareto_frontier(&self, findings: &[RuleFinding]) -> Vec<RuleFinding> {
        let mut pareto = Vec::new();

        for finding in findings {
            let is_dominated = pareto.iter().any(|p: &RuleFinding| {
                severity_weight(p.severity) >= severity_weight(finding.severity)
                    && (p.line.is_some()
                        && finding.line.is_some()
                        && p.line.unwrap() <= finding.line.unwrap())
            });

            if !is_dominated {
                pareto.push(finding.clone());
            }
        }

        pareto
    }
}

impl Default for Aggregator {
    fn default() -> Self {
        Self::new(AggregatorWeights::default())
    }
}
