//! # Feedback Module
//!
//! `feedback` provides feedback formulation for the retry loop.
//!
//! Path: `xiaoyi::evaluator::feedback`
//!
//! @module evaluator::feedback
//! @brief Feedback formulator for retry loop
//! @group AI Evaluation
//! @since 0.1.0
//! @author Miruamel
//! @see crate::evaluator
//! @see crate::evaluator::gates
//! @see crate::critic::aggregator

use crate::xiaoyi::evaluator::{EvaluationResult, GateResult, AnalysisFinding};

/// Feedback configuration.
///
/// @brief Feedback generation settings
/// @group AI Evaluation
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct FeedbackConfig {
    /// Include passing tests in feedback
    pub include_passing: bool,
    /// Include all findings or only failures
    pub all_findings: bool,
    /// Max feedback length (chars)
    pub max_length: usize,
    /// Language for feedback
    pub language: String,
}

impl Default for FeedbackConfig {
    fn default() -> Self {
        Self {
            include_passing: false,
            all_findings: true,
            max_length: 4000,
            language: "en".to_string(),
        }
    }
}

/// Feedback formulator for retry loops.
///
/// @brief Formulates actionable feedback from evaluation results
/// @group AI Evaluation
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct FeedbackFormulator {
    config: FeedbackConfig,
}

impl FeedbackFormulator {
    /// Create new feedback formulator.
    ///
    /// @param config Feedback configuration
    /// @return FeedbackFormulator instance
    /// @since 0.1.0
    pub fn new(config: FeedbackConfig) -> Self {
        Self { config }
    }

    /// Formulate feedback from evaluation result.
    ///
    /// @param result Evaluation result
    /// @return Formulated feedback string
    /// @since 0.1.0
    pub fn formulate(&self, result: &EvaluationResult) -> String {
        let mut feedback = String::new();

        // Overall status
        feedback.push_str(&format!(
            "Evaluation {}: {}\n\n",
            if result.gate_status.overall_pass { "PASSED" } else { "FAILED" },
            if result.gate_status.overall_pass {
                "All quality gates passed."
            } else {
                "Some quality gates failed. See details below."
            }
        ));

        // Failed gates
        let failed_gates: Vec<&GateResult> = result
            .gate_status
            .gates
            .iter()
            .filter(|g| !g.passed)
            .collect();

        if !failed_gates.is_empty() {
            feedback.push_str("## Failed Quality Gates\n\n");
            for gate in failed_gates {
                feedback.push_str(&format!(
                    "- **{}**: {} (threshold: {}, actual: {})\n",
                    gate.name, gate.message, gate.threshold, gate.actual
                ));
            }
            feedback.push('\n');
        }

        // Analysis findings
        if !result.analysis_findings.is_empty() {
            let critical: Vec<_> = result
                .analysis_findings
                .iter()
                .filter(|f| f.severity == crate::xiaoyi::evaluator::Severity::Critical)
                .collect();
            let errors: Vec<_> = result
                .analysis_findings
                .iter()
                .filter(|f| f.severity == crate::xiaoyi::evaluator::Severity::Error)
                .collect();
            let warnings: Vec<_> = result
                .analysis_findings
                .iter()
                .filter(|f| f.severity == crate::xiaoyi::evaluator::Severity::Warning)
                .collect();

            if !critical.is_empty() {
                feedback.push_str("## Critical Findings\n\n");
                for finding in critical {
                    feedback.push_str(&self.format_finding(finding));
                }
                feedback.push('\n');
            }

            if !errors.is_empty() {
                feedback.push_str("## Errors\n\n");
                for finding in errors {
                    feedback.push_str(&self.format_finding(finding));
                }
                feedback.push('\n');
            }

            if self.config.all_findings && !warnings.is_empty() {
                feedback.push_str("## Warnings\n\n");
                for finding in warnings {
                    feedback.push_str(&self.format_finding(finding));
                }
                feedback.push('\n');
            }
        }

        // Test failures
        let failed_tests: Vec<_> = result.test_results.iter().filter(|t| !t.passed).collect();
        if !failed_tests.is_empty() {
            feedback.push_str("## Failed Tests\n\n");
            for test in failed_tests {
                feedback.push_str(&format!("- **{}** ({:?}): ", test.name, test.test_type));
                if let Some(msg) = &test.message {
                    feedback.push_str(msg);
                }
                feedback.push('\n');
            }
            feedback.push('\n');
        }

        // Passing tests (if configured)
        if self.config.include_passing {
            let passed_tests: Vec<_> = result.test_results.iter().filter(|t| t.passed).collect();
            if !passed_tests.is_empty() {
                feedback.push_str("## Passing Tests\n\n");
                for test in passed_tests {
                    feedback.push_str(&format!("- **{}** ({:?})\n", test.name, test.test_type));
                }
                feedback.push('\n');
            }
        }

        // Benchmark info
        if let Some(b) = &result.benchmark_results {
            feedback.push_str("## Performance\n\n");
            feedback.push_str(&format!("- Execution time: {}ms\n", b.execution_time_ms));
            feedback.push_str(&format!("- Memory peak: {}MB\n", b.memory_peak_bytes / 1024 / 1024));
            feedback.push_str(&format!("- CPU time: {}ms\n", b.cpu_time_ms));
            if b.estimated_cost_usd > 0.0 {
                feedback.push_str(&format!("- Estimated cost: ${:.4}\n", b.estimated_cost_usd));
            }
            if let Some(t) = &b.token_usage {
                feedback.push_str(&format!("- Tokens: {} total ({} prompt + {} completion)\n", t.total_tokens, t.prompt_tokens, t.completion_tokens));
            }
            feedback.push('\n');
        }

        // Truncate if too long
        if feedback.len() > self.config.max_length {
            feedback.truncate(self.config.max_length);
            feedback.push_str("\n... (truncated)");
        }

        feedback
    }

    /// Format a single finding.
    ///
    /// @param finding Analysis finding
    /// @return Formatted string
    fn format_finding(&self, finding: &AnalysisFinding) -> String {
        let mut s = String::new();
        s.push_str(&format!("- **{}**", finding.message));
        if let Some(file) = &finding.file {
            s.push_str(&format!(" ({}", file));
            if let Some(line) = finding.line {
                s.push_str(&format!(":{}", line));
            }
            s.push(')');
        }
        if let Some(rule) = &finding.rule_id {
            s.push_str(&format!(" [Rule: {}]", rule));
        }
        s.push_str(&format!(" (Tool: {})\n", finding.tool));
        s
    }

    /// Generate retry prompt for agent.
    ///
    /// @param result Evaluation result
    /// @return Retry prompt
    /// @since 0.1.0
    pub fn retry_prompt(&self, result: &EvaluationResult) -> String {
        let mut prompt = String::new();
        prompt.push_str("The previous attempt failed quality gates. Please fix the following issues:\n\n");
        prompt.push_str(&self.formulate(result));
        prompt.push_str("\n\nPlease provide corrected code that addresses all failures.");
        prompt
    }
}
