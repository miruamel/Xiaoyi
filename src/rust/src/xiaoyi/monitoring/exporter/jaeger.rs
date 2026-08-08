use crate::xiaoyi::monitoring::tracing::exporter::TraceExporter;

/// Jaeger exporter placeholder.
///
/// @brief Jaeger trace exporter configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring::tracing::exporter
pub struct JaegerExporter {
    pub endpoint: String,
}

impl TraceExporter for JaegerExporter {
    fn export(&self, _span: &crate::xiaoyi::monitoring::tracing::Span) {
        // TODO: implement Jaeger export
    }
}
