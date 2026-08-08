use crate::xiaoyi::evaluator::EvaluationResult;

/// Report formatter.
///
/// @brief Format evaluation reports
/// @since 0.1.0
/// @author Miruamel
/// @see EvaluationResult
pub struct ReportFormatter;

impl ReportFormatter {
    /// Format a report as text.
    ///
    /// @param report Evaluation report
    /// @return Formatted text
    /// @since 0.1.0
    pub fn format_text(&self, report: &EvaluationResult) -> String {
        format!(
            "Evaluation Report\ntests={} findings={} feedback={}",
            report.test_results.len(),
            report.analysis_findings.len(),
            report.feedback.as_deref().unwrap_or("<none>")
        )
    }
}
