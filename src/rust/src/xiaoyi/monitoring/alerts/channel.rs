use crate::xiaoyi::monitoring::alerts::alert::Alert;

/// Delivers alerts through an abstract channel.
///
/// @brief Alert delivery channel
/// @since 0.1.0
/// @author Miruamel
/// @see crate::xiaoyi::monitoring::alerts::notifier
pub trait AlertChannel {
    /// Send an alert.
    ///
    /// @param alert Alert to send
    /// @since 0.1.0
    fn send(&self, alert: &Alert);
}
