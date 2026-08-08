//! # Tracing — Span Context
//!
//! This module defines [`SpanContext`], the carrier of trace propagation
//! state. A context holds the active trace identifier and the identifier of
//! the currently-open span; calling [`SpanContext::child`] derives a new
//! nested [`Span`] that automatically links itself to the parent via
//! `parent_id`.
//!
//! Path: `xiaoyi::monitoring::tracing::context`
//!
//! - Layer 9: `monitoring` — Monitoring & Tracing.
//!
//! @module xiaoyi::monitoring::tracing::context
//! @brief Trace propagation context and child-span derivation
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi::monitoring::tracing
//! @see crate::xiaoyi::monitoring::tracing::span::Span

use crate::xiaoyi::core::error::Result;
use serde::{Deserialize, Serialize};

use super::span::{Span, SpanKind};

/// Propagation context for an in-flight trace.
///
/// A `SpanContext` carries the trace identifier and the identifier of the
/// span currently executing. It is the unit passed between scopes so that new
/// spans can be correctly nested under the active parent.
///
/// @brief Trace identifier + active span identifier carrier
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see Span
/// @see Span::child_of
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanContext {
    /// Identifier of the trace this context belongs to.
    ///
    /// @brief Trace identifier
    /// @since 0.1.0
    pub trace_id: String,
    /// Identifier of the currently active (parent) span.
    ///
    /// @brief Active span identifier
    /// @since 0.1.0
    pub current_span_id: String,
}

impl SpanContext {
    /// Creates a context from explicit trace and span identifiers.
    ///
    /// @brief Construct a span context
    /// @param trace_id Identifier of the owning trace
    /// @param current_span_id Identifier of the active parent span
    /// @return New [`SpanContext`]
    /// @since 0.1.0
    /// @see Span::new
    pub fn new(trace_id: impl Into<String>, current_span_id: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            current_span_id: current_span_id.into(),
        }
    }

    /// Derives a new child [`Span`] nested under the active span.
    ///
    /// The returned span carries `current_span_id` as its `parent_id`,
    /// preserving the trace topology through the context's `trace_id`.
    ///
    /// @brief Create a nested span under the active parent
    /// @param name Operation name for the child span
    /// @param kind Classification of the child span
    /// @return Newly created child [`Span`]
    /// @since 0.1.0
    /// @see Span::child_of
    pub fn child(&self, name: impl Into<String>, kind: SpanKind) -> Span {
        Span::child_of(self.current_span_id.clone(), name, kind)
    }

    /// Returns a copy of this context with the active span replaced by `span_id`.
    ///
    /// Used to advance the context after opening a new child span so that
    /// further children nest beneath it.
    ///
    /// @brief Replace the active span identifier
    /// @param span_id New active span identifier
    /// @return Updated [`SpanContext`] sharing the same `trace_id`
    /// @since 0.1.0
    pub fn with_span(&self, span_id: impl Into<String>) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            current_span_id: span_id.into(),
        }
    }

    /// Advances the context to a new child span, returning both artefacts.
    ///
    /// Convenience helper that creates the child span and a context pointing
    /// at it in a single call, keeping the trace identifier intact.
    ///
    /// @brief Open a child span and advance the context atomically
    /// @param name Operation name for the child span
    /// @param kind Classification of the child span
    /// @return Tuple of `(child_span, advanced_context)`
    /// @since 0.1.0
    pub fn spawn(&self, name: impl Into<String>, kind: SpanKind) -> (Span, SpanContext) {
        let span = self.child(name, kind);
        let ctx = self.with_span(span.id.clone());
        (span, ctx)
    }

    /// Builds a root context whose trace and span ids are freshly generated.
    ///
    /// @brief Construct a root context for a new trace
    /// @return New root [`SpanContext`]
    /// @since 0.1.0
    pub fn root() -> Self {
        let id = Span::new("", SpanKind::Internal).id;
        Self::new(id.clone(), id)
    }

    /// Reconstructs a context from a root [`Span`].
    ///
    /// The trace and active span identifiers are taken from the span's own
    /// id; this is the bridge from [`crate::xiaoyi::monitoring::tracing::Tracer`]
    /// output into the context propagation API.
    ///
    /// @brief Derive a context from a root span
    /// @param span A root span (typically produced by [`crate::xiaoyi::monitoring::tracing::Tracer::start`])
    /// @return [`SpanContext`] sharing the span's id as both trace and span id
    /// @since 0.1.0
    pub fn from_span(span: &Span) -> Result<Self> {
        Ok(Self::new(span.id.clone(), span.id.clone()))
    }
}
