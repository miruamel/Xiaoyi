use crate::xiaoyi::monitoring::tracing::span::SpanKind;

/// Trace sampling strategy.
///
/// @brief Sampler for trace spans
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring::tracing
#[derive(Debug, Clone, Copy)]
pub enum Sampler {
    /// Record all spans.
    AlwaysOn,
    /// Record no spans.
    AlwaysOff,
    /// Record based on ratio.
    TraceIdRatio(f64),
}

impl Sampler {
    /// Decide whether to sample a span.
    ///
    /// @brief Sample decision
    /// @param sampler Sampler strategy
    /// @return True if span should be recorded
    /// @since 0.1.0
    pub fn should_sample(&self, _kind: SpanKind) -> bool {
        matches!(self, Sampler::AlwaysOn)
    }
}
