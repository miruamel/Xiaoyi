//! # Gates Module
//!
//! `gates` provides quality gates and compliance checking.
//!
//! Path: `xiaoyi::evaluator::gates`
//!
//! @module evaluator::gates
//! @brief Quality gates and compliance checking
//! @group AI Evaluation
//! @since 0.1.0
//! @author Miruamel
//! @see crate::evaluator
//! @see crate::evaluator::analysis
//! @see crate::critic::aggregator

use crate::xiaoyi::evaluator::{GateResult, GateStatus, Severity};
use crate::xiaoyi::evaluator::AnalysisFinding;

/// Quality gate configuration.
///
/// @brief Quality gate thresholds and rules
/// @group AI Evaluation
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct GateConfig {
    /// Minimum test coverage (%)
    pub min_coverage: f64,
    /// Maximum critical findings allowed
    pub max_critical_findings: usize,
    /// Maximum high severity findings
    pub max_high_findings: usize,
    /// Maximum execution time (ms)
    pub max_execution_time_ms: u64,
    /// Maximum memory (bytes)
    pub max_memory_bytes: u64,
    /// Maximum estimated cost (USD)
    pub max_cost_usd: f64,
    /// Require all tests pass
    pub require_all_tests_pass: bool,
}

impl Default for GateConfig {
    fn default() -> Self {
        Self {
            min_coverage: 80.0,
            max_critical_findings: 0,
            max_high_findings: 5,
            max_execution_time_ms: 5000,
            max_memory_bytes: 512 * 1024 * 1024,
            max_cost_usd: 1.0,
            require_all_tests_pass: true,
        }
    }
}

/// Quality gate evaluator.
///
/// @brief Evaluates quality gates against results
/// @group AI Evaluation
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct GateEvaluator {
    config: GateConfig,
}

impl GateEvaluator {
    /// Create new gate evaluator.
    ///
    /// @param config Gate configuration
    /// @return GateEvaluator instance
    /// @since 0.1.0
    pub fn new(config: GateConfig) -> Self {
        Self { config }
    }

    /// Evaluate all quality gates.
    ///
    /// @param test_results Test results
    /// @param findings Analysis findings
    /// @param benchmark Benchmark result
    /// @return Gate status
    /// @since 0.1.0
    pub fn evaluate(
        &self,
        test_results: &[crate::xiaoyi::evaluator::TestResult],
        findings: &[AnalysisFinding],
        benchmark: Option<&crate::xiaoyi::evaluator::BenchmarkResult>,
    ) -> GateStatus {
        let mut gates = Vec::new();
        let mut overall_pass = true;

        // Test pass gate
        let passed = test_results.iter().all(|t| t.passed);
        gates.push(GateResult {
            name: "all_tests_pass".to_string(),
            passed,
            threshold: 1.0,
            actual: if passed { 1.0 } else { 0.0 },
            message: if passed {
                "All tests passed".to_string()
            } else {
                format!("{} tests failed", test_results.iter().filter(|t| !t.passed).count())
            },
        });
        if !passed && self.config.require_all_tests_pass {
            overall_pass = false;
        }

        // Critical findings gate
        let critical_count = findings.iter().filter(|f| f.severity == Severity::Critical).count();
        let passed = critical_count <= self.config.max_critical_findings;
        gates.push(GateResult {
            name: "critical_findings".to_string(),
            passed,
            threshold: self.config.max_critical_findings as f64,
            actual: critical_count as f64,
            message: format!("{} critical findings (max {})", critical_count, self.config.max_critical_findings),
        });
        if !passed {
            overall_pass = false;
        }

        // High findings gate
        let high_count = findings.iter().filter(|f| f.severity == Severity::Error).count();
        let passed = high_count <= self.config.max_high_findings;
        gates.push(GateResult {
            name: "high_findings".to_string(),
            passed,
            threshold: self.config.max_high_findings as f64,
            actual: high_count as f64,
            message: format!("{} high findings (max {})", high_count, self.config.max_high_findings),
        });
        if !passed {
            overall_pass = false;
        }

        // Execution time gate
        if let Some(b) = benchmark {
            let passed = b.execution_time_ms <= self.config.max_execution_time_ms;
            gates.push(GateResult {
                name: "execution_time".to_string(),
                passed,
                threshold: self.config.max_execution_time_ms as f64,
                actual: b.execution_time_ms as f64,
                message: format!("{}ms (max {}ms)", b.execution_time_ms, self.config.max_execution_time_ms),
            });
            if !passed {
                overall_pass = false;
            }
        }

        // Memory gate
        if let Some(b) = benchmark {
            let passed = b.memory_peak_bytes <= self.config.max_memory_bytes;
            gates.push(GateResult {
                name: "memory_usage".to_string(),
                passed,
                threshold: self.config.max_memory_bytes as f64,
                actual: b.memory_peak_bytes as f64,
                message: format!("{}MB (max {}MB)", b.memory_peak_bytes / 1024 / 1024, self.config.max_memory_bytes / 1024 / 1024),
            });
            if !passed {
                overall_pass = false;
            }
        }

        // Cost gate
        if let Some(b) = benchmark {
            let passed = b.estimated_cost_usd <= self.config.max_cost_usd;
            gates.push(GateResult {
                name: "estimated_cost".to_string(),
                passed,
                threshold: self.config.max_cost_usd,
                actual: b.estimated_cost_usd,
                message: format!("${:.4} (max ${:.2})", b.estimated_cost_usd, self.config.max_cost_usd),
            });
            if !passed {
                overall_pass = false;
            }
        }

        GateStatus { overall_pass, gates }
    }
}
