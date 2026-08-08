/// # Layer 9 — Monitoring (Tracing)
///
/// `monitoring` is the observability substrate. It provides a `Tracer` for span-based tracing
/// and a `TraceExporter` for submitting spans to external backends (e.g., OTLP, Jaeger).
/// This deep-vertical layer implements OpenTelemetry-inspired span semantics within a Rust
/// framework.
///
/// # Module Structure
///
/// - `span` — Core span data types (`SpanKind`, `Span`) and lifecycle management.
/// - `context` — Distributed tracing context (`SpanContext`) for trace propagation and child‑span generation.
/// - `exporter` — Exporter trait and concrete `Tracer` implementation holding an optional async exporter.
///
/// # Path
///
/// - Layer 9: `monitoring` — Monitoring & Tracing.
///
/// @module xiaoyi::monitoring::tracing
/// @brief Monitoring & Tracing substrate with span lifecycle and exporter integration
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring

pub mod span;
pub mod context;
pub mod exporter;

pub use span::{SpanKind, Span};
pub use context::SpanContext;
pub use exporter::{TraceExporter, Tracer};