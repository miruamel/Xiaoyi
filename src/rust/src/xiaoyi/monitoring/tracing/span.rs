//! # Tracing — Span & Kind
//!
//! This module defines the core span data model used by the Xiaoyi tracing
//! subsystem. A [`Span`] represents a single unit of work within a distributed
//! trace, carrying timing, kind, and free-form attributes. [`SpanKind`]
//! classifies the relationship between the span and the local workload
//! (internal, server, client, producer, consumer), following the
//! OpenTelemetry span-kind taxonomy.
//!
//! Spans are immutable once finished: [`Span::finish`] records the end
//! timestamp and refuses to overwrite an already-finished span.
//!
//! Path: `xiaoyi::monitoring::tracing::span`
//!
//! - Layer 9: `monitoring` — Monitoring & Tracing.
//!
//! @module xiaoyi::monitoring::tracing::span
//! @brief Span data model and span-kind classification for the tracing layer
//! @group Monitoring
//! @since 0.1.0
//! @author Miruamel
//! @see crate::xiaoyi::monitoring::tracing
//! @see crate::xiaoyi::monitoring::tracing::context::SpanContext

use crate::xiaoyi::core::error::{ErrorKind, Result, XiaoyiError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
/// Monotonic process-unique counter backing span/trace id generation.
static SEQ: AtomicU64 = AtomicU64::new(0);
/// Returns the current wall-clock time as seconds since the Unix epoch.
///
/// @brief Current Unix epoch time in seconds
/// @return Seconds since `1970-01-01T00:00:00Z`
/// @since 0.1.0
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
/// Generates a process-unique id for spans and traces.
///
/// @brief Unique identifier generator for tracing artefacts
/// @return Hex-encoded identifier combining timestamp and a monotonic counter
/// @since 0.1.0
fn new_id() -> String {
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let ts = now_secs();
    format!("{ts:016x}-{seq:016x}")
}

/// Classification of a span relative to the local unit of work.
///
/// Mirrors the OpenTelemetry `SpanKind` taxonomy and is used by exporters to
/// set relational semantics (client/server, producer/consumer) on the wire.
///
/// @brief Relationship between a span and the local workload
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see Span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanKind {
    /// Default kind. Used for spans that do not cross a process boundary.
    ///
    /// @brief Internal / in-process unit of work
    /// @since 0.1.0
    Internal,
    /// Span covering the server side of an RPC or HTTP exchange.
    ///
    /// @brief Server-side request handler
    /// @since 0.1.0
    Server,
    /// Span covering the client side of an outbound RPC or HTTP exchange.
    ///
    /// @brief Client-side request initiator
    /// @since 0.1.0
    Client,
    /// Span covering the producer side of a messaging/queue publish.
    ///
    /// @brief Message producer / publisher
    /// @since 0.1.0
    Producer,
    /// Span covering the consumer side of a messaging/queue receive.
    ///
    /// @brief Message consumer / subscriber
    /// @since 0.1.0
    Consumer,
}

/// A single timed unit of work within a distributed trace.
///
/// A `Span` records when it started (see [`Span::finish`]), the kind of work it
/// represents, an optional parent span (for nesting), and arbitrary string
/// attributes. Call [`Span::finish`] to seal the span; a finished span cannot
/// be re-finished.
///
/// @brief Timed, attributed unit of work in a trace
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see SpanKind
/// @see crate::xiaoyi::monitoring::tracing::context::SpanContext
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    /// Unique identifier for this span within the process.
    ///
    /// @brief Span identifier
    /// @since 0.1.0
    pub id: String,
    /// Identifier of the parent span, if this span is nested.
    ///
    /// @brief Parent span identifier (`None` for root spans)
    /// @since 0.1.0
    pub parent_id: Option<String>,
    /// Human-readable operation name (e.g. `"db.query"`).
    ///
    /// @brief Operation name
    /// @since 0.1.0
    pub name: String,
    /// Classification of the span's relationship to local work.
    ///
    /// @brief Span kind
    /// @since 0.1.0
    pub kind: SpanKind,
    /// Start timestamp in seconds since the Unix epoch.
    ///
    /// @brief Span start time (Unix epoch seconds)
    /// @since 0.1.0
    pub start: u64,
    /// End timestamp in seconds since the Unix epoch, if finished.
    ///
    /// @brief Span end time (Unix epoch seconds), `None` until [`Span::finish`]
    /// @since 0.1.0
    pub end: Option<u64>,
    /// Free-form key-value attributes attached to the span.
    ///
    /// @brief Span attributes
    /// @since 0.1.0
    pub attributes: HashMap<String, String>,
}

impl Span {
    /// Creates a new root span with no parent.
    ///
    /// @brief Construct a root span
    /// @param name Operation name for the span
    /// @param kind Classification of the span
    /// @return Newly created, unfinished root [`Span`]
    /// @since 0.1.0
    /// @see finish
    pub fn new(name: impl Into<String>, kind: SpanKind) -> Self {
        Self {
            id: new_id(),
            parent_id: None,
            name: name.into(),
            kind,
            start: now_secs(),
            end: None,
            attributes: HashMap::new(),
        }
    }

    /// Creates a child span nested under `parent_id`.
    ///
    /// @brief Construct a nested span
    /// @param parent_id Identifier of the parent span
    /// @param name Operation name for the span
    /// @param kind Classification of the span
    /// @return Newly created, unfinished child [`Span`]
    /// @since 0.1.0
    /// @see crate::xiaoyi::monitoring::tracing::context::SpanContext::child
    pub fn child_of(parent_id: impl Into<String>, name: impl Into<String>, kind: SpanKind) -> Self {
        Self {
            id: new_id(),
            parent_id: Some(parent_id.into()),
            name: name.into(),
            kind,
            start: now_secs(),
            end: None,
            attributes: HashMap::new(),
        }
    }

    /// Seals the span by recording its end timestamp.
    ///
    /// Refuses to overwrite an already-finished span and returns a
    /// [`ErrorKind::State`] error in that case.
    ///
    /// @brief Record the span's end time
    /// @return `Ok(())` on success, or an error if already finished
    /// @throw [`XiaoyiError`] with [`ErrorKind::State`] when called twice
    /// @since 0.1.0
    pub fn finish(&mut self) -> Result<()> {
        if self.end.is_some() {
            return Err(XiaoyiError::new(
                ErrorKind::State,
                "span has already been finished",
            )
            .with_meta("span_id", self.id.clone()));
        }
        self.end = Some(now_secs());
        Ok(())
    }

    /// Attaches a string attribute to the span.
    ///
    /// @brief Set a span attribute
    /// @param key Attribute key
    /// @param value Attribute value
    /// @return `Ok(())` on success
    /// @since 0.1.0
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        self.attributes.insert(key.into(), value.into());
        Ok(())
    }

    /// Whether the span has been finished.
    ///
    /// @brief Finished state query
    /// @return `true` if [`Span::finish`] has been called
    /// @since 0.1.0
    pub fn is_finished(&self) -> bool {
        self.end.is_some()
    }
}
