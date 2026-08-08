use crate::xiaoyi::monitoring::alerts::alert::Alert;

/// Email alert sink.
///
/// @brief Send alerts by email
/// @since 0.1.0
/// @author Miruamel
/// @see Alert
pub struct EmailAlertSink;

impl EmailAlertSink {
    /// Send alert by email.
    ///
    /// @param alert Alert to send
    /// @since 0.1.0
    pub fn send(&self, _alert: &Alert) {
        // email sink placeholder
    }
}
