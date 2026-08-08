//! # Rules Engine
//!
//! `rules` provides the fast-path rules engine for static analysis.
//!
//! Path: `xiaoyi::critic::rules`
//!
//! @module critic::rules
//! @brief Fast-path rules engine (linters/regex)
//! @group AI Review
//! @since 0.1.0
//! @author Miruamel
//! @see crate::critic
//! @see crate::critic::small_llm

use crate::xiaoyi::critic::{RuleFinding, Severity};
use crate::xiaoyi::core::error::Result;

/// Rules engine for fast static analysis.
///
/// @brief Linter/regex based static analysis
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct RulesEngine {
    rules: Vec<Rule>,
}

/// Rule definition.
///
/// @brief Single analysis rule
/// @group AI Review
/// @since 0.1.0
#[derive(Debug, Clone)]
pub struct Rule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Regex pattern to match
    pub pattern: regex::Regex,
    /// Severity
    pub severity: Severity,
    /// Description
    pub description: String,
}

impl RulesEngine {
    /// Create new rules engine with default rules.
    ///
    /// @return RulesEngine instance
    /// @since 0.1.0
    pub fn new() -> Result<Self> {
        let mut rules = Vec::new();

        // Add default rules
        rules.push(Rule {
            id: "TODO_COMMENT".to_string(),
            name: "TODO Comment".to_string(),
            pattern: regex::Regex::new(r"(?i)(TODO|FIXME|HACK|XXX):").expect("invalid regex"),
            severity: Severity::Info,
            description: "TODO/FIXME/HACK/XXX comment found".to_string(),
        });

        rules.push(Rule {
            id: "UNWRAP_CALL".to_string(),
            name: "Unwrap Call".to_string(),
            pattern: regex::Regex::new(r"\.unwrap\(\)").expect("invalid regex"),
            severity: Severity::Warning,
            description: "Use of .unwrap() which may panic".to_string(),
        });

        rules.push(Rule {
            id: "EXPECT_CALL".to_string(),
            name: "Expect Call".to_string(),
            pattern: regex::Regex::new(r"\.expect\(").expect("invalid regex"),
            severity: Severity::Warning,
            description: "Use of .expect() which may panic".to_string(),
        });

        rules.push(Rule {
            id: "PANIC_MACRO".to_string(),
            name: "Panic Macro".to_string(),
            pattern: regex::Regex::new(r"panic!").expect("invalid regex"),
            severity: Severity::Error,
            description: "Use of panic! macro".to_string(),
        });

        rules.push(Rule {
            id: "HARDCODED_SECRET".to_string(),
            name: "Hardcoded Secret".to_string(),
            pattern: regex::Regex::new(r#"(?i)(api_key|secret|password|token)\s*=\s*["'][^"']+["']"#).expect("invalid regex"),
            severity: Severity::Critical,
            description: "Potential hardcoded secret".to_string(),
        });

        Ok(Self { rules })
    }

    /// Analyze code with rules engine.
    ///
    /// @param code Source code to analyze
    /// @param file Optional file path for context
    /// @return Vector of rule findings
    /// @since 0.1.0
    pub fn analyze(&self, code: &str, file: Option<&str>) -> Vec<RuleFinding> {
        let mut findings = Vec::new();

        for (line_num, line) in code.lines().enumerate() {
            for rule in &self.rules {
                if rule.pattern.is_match(line) {
                    findings.push(RuleFinding {
                        rule_id: rule.id.clone(),
                        severity: rule.severity,
                        message: format!("{}: {}", rule.name, rule.description),
                        file: file.map(|s| s.to_string()),
                        line: Some(line_num + 1),
                        column: None,
                    });
                }
            }
        }

        findings
    }
}

impl Default for RulesEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create RulesEngine")
    }
}