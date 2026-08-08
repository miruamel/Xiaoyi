//! # Tracing — Exporter & Tracer
//!
//! This module defines the export boundary and the entry-point tracer.
//!
//! [`TraceExporter`] is the async sink contract: implementations ship
//! finished [`Span`]s to a backend (OTLP, Jaeger, logs, …). [`Tracer`] is the
//! user-facing façade — it mints root and child spans and optionally forwards
//! them to a configured exporter through [`Tracer::export_span`].
//!
//! The configured exporter is held behind a `parking_lot::RwLock`, so the
//! [`Tracer`] may be shared across tasks and reconfigured at runtime.
//!
//! Path: `xiaoyi::monitoring::tracing::exporter`
//!
//! - Layer 9: `monitoring` — Monitoring & Tracing.
//!
//! @module xiaoyi::monitoring::tracing::exporter
//! @brief Exporter trait and tracer façade for the tracing layer
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi::monitoring::tracing
//! @see crate::xiaoyi::monitoring::tracing::span::Span
//! @see crate::xiaoyi::monitoring::tracing::context::SpanContext

use crate::xiaoyi::core::error::Result;
use parking_lot::RwLock;
use std::sync::Arc;

use super::context::SpanContext;
use super::span::{Span, SpanKind};

/// Async sink for finished spans.
///
/// Implement this trait to ship spans to an observability backend. The tracer
/// invokes [`TraceExporter::export`] for every span handed to
/// [`crate::xiaoyi::monitoring::tracing::Tracer::export_span`].
///
/// @brief Contract for asynchronously exporting spans
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see Tracer::export_span
/// @see Span
#[async_trait::async_trait]
pub trait TraceExporter: Send + Sync {
    /// Exports a single finished (or in-flight) span.
    ///
    /// @brief Ship a span to the backing store
    /// @param span The span to export
    /// @return `Ok(())` when accepted, or an error on transport/storage failure
    /// @throw [`crate::xiaoyi::core::error::XiaoyiError`] on export failure
    /// @since 0.1.0
    /// @see Tracer::set_exporter
    async fn export(&self, span: &Span) -> Result<()>;
}

/// User-facing tracer that mints spans and (optionally) exports them.
///
/// A `Tracer` is cheap to clone-share via `Arc`: span creation is infallible
/// and the configured exporter is guarded by a `RwLock`, so the tracer is
/// safe for concurrent use.
///
/// @brief Façade for creating and exporting spans
/// @group Monitoring
/// @author Miruamel
/// @see TraceExporter
/// @see Span
/// @see SpanContext
#[derive(Default)]
pub struct Tracer {
    /// Optional async exporter invoked by [`Tracer::export_span`].
    exporter: RwLock<Option<Arc<dyn TraceExporter>>>,
}

impl Tracer {
    /// Creates a new tracer without an exporter.
    ///
    /// @brief Construct a tracer with no exporter
    /// @return A fresh [`Tracer`]
    /// @since 0.1.0
    /// @see set_exporter
    pub fn new() -> Self {
        Self {
            exporter: RwLock::new(None),
        }
    }

    /// Registers the exporter used by [`Tracer::export_span`].
    ///
    /// @brief Configure the active span exporter
    /// @param exporter Shared exporter implementation
    /// @return `Ok(())` on success
    /// @since 0.1.0
    /// @see export_span
    pub fn set_exporter(&self, exporter: Arc<dyn TraceExporter>) -> Result<()> {
        *self.exporter.write() = Some(exporter);
        Ok(())
    }

    /// Removes any configured exporter.
    ///
    /// @brief Clear the active exporter
    /// @return `Ok(())` on success
    /// @since 0.1.0
    /// @see set_exporter
    pub fn clear_exporter(&self) -> Result<()> {
        *self.exporter.write() = None;
        Ok(())
    }

    /// Creates a new root span with no parent.
    ///
    /// The span is tagged [`SpanKind::Internal`] by default; re-tag it via the
    /// returned value if a different kind is required.
    ///
    /// @brief Open a root span
    /// @param name Operation name for the span
    /// @return Newly created root [`Span`]
    /// @since 0.1.0
    /// @see start_child
    pub fn start(&self, name: &str) -> Span {
        Span::new(name, SpanKind::Internal)
    }

    /// Creates a child span nested under the active span in `ctx`.
    ///
    /// @brief Open a child span from a propagation context
    /// @param ctx Active trace context (carries trace + parent span ids)
    /// @param name Operation name for the child span
    /// @return Newly created child [`Span`]
    /// @since 0.1.0
    /// @see SpanContext::child
    pub fn start_child(&self, ctx: &SpanContext, name: &str) -> Span {
        ctx.child(name, SpanKind::Internal)
    }

    /// Exports a span if an exporter is configured.
    ///
    /// When no exporter is set the call is a no-op success; otherwise the span
    /// is forwarded to [`TraceExporter::export`].
    ///
    /// @brief Export a span to the configured backend
    /// @param span The span to export
    /// @return `Ok(())` if dropped or accepted, or the exporter's error
    /// @throw [`crate::xiaoyi::core::error::XiaoyiError`] propagated from the exporter
    /// @since 0.1.0
    /// @see set_exporter
    /// @see TraceExporter::export
    pub async fn export_span(&self, span: &Span) -> Result<()> {
        let exporter = self.exporter.read().clone();
        if let Some(exporter) = exporter {
            exporter.export(span).await?;
        }
        Ok(())
    }
}
