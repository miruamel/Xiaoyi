use crate::xiaoyi::monitoring::tracing::exporter::TraceExporter;

/// OTLP exporter placeholder.
///
/// @brief OTLP trace exporter configuration
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring::tracing::exporter
pub struct OtlpExporter {
    pub endpoint: String,
}

impl TraceExporter for OtlpExporter {
    fn export(&self, _span: &crate::xiaoyi::monitoring::tracing::Span) {
        // TODO: implement OTLP export
    }
}
