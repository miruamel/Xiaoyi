use crate::xiaoyi::core::error::Result;
use async_trait::async_trait;
use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::VecDeque;

use super::{Alert, AlertRule};

/// Notifier trait for alert delivery mechanisms.
///
/// @brief Defines the interface for delivering alerts to external systems
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see AlertManager
#[async_trait]
pub trait Notifier: Send + Sync {
    /// Sends an alert to the external system.
    ///
    /// @brief Deliver an alert to an external monitoring system
    /// @param alert The alert to deliver
    /// @return `Ok(())` on successful delivery, `Err(XiaoyiError)` otherwise
    /// @since 0.1.0
    async fn notify(&self, alert: &Alert) -> Result<()>;
}

/// Alert manager that stores rules, fires alerts, and maintains history.
///
/// @brief Centralized alert management for threshold-based notifications
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see AlertRule
/// @see Notifier
pub struct AlertManager {
    /// Collection of alert rules.
    ///
    /// @brief Active alert rules
    rules: RwLock<Vec<AlertRule>>,
    /// Optional notifier for delivering alerts.
    ///
    /// @brief Alert delivery target
    notifier: Option<Arc<dyn Notifier>>,
    /// In-memory history of fired alerts.
    ///
    /// @brief History of fired alerts (newest first)
    history: RwLock<VecDeque<Alert>>,
}

impl AlertManager {
    /// Creates a new alert manager.
    ///
    /// @brief Create a new alert manager instance
    /// @return New `AlertManager` instance
    /// @since 0.1.0
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
            notifier: None,
            history: RwLock::new(VecDeque::new()),
        }
    }

    /// Creates a new alert manager with a notifier.
    ///
    /// @brief Create an alert manager with a notifier
    /// @param notifier The notifier to use for alert delivery
    /// @return New `AlertManager` instance
    /// @since 0.1.0
    pub fn with_notifier(notifier: Arc<dyn Notifier>) -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
            notifier: Some(notifier),
            history: RwLock::new(VecDeque::new()),
        }
    }

    /// Adds a new alert rule.
    ///
    /// @brief Register a new alert rule
    /// @param rule The alert rule to add
    /// @since 0.1.0
    pub fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.write();
        rules.push(rule);
    }

    /// Removes an alert rule by name.
    ///
    /// @brief Remove an alert rule
    /// @param name Name of the rule to remove
    /// @since 0.1.0
    pub fn remove_rule(&self, name: &str) {
        let mut rules = self.rules.write();
        rules.retain(|rule| rule.name != name);
    }

    /// Evaluates a metric against all rules and fires alerts if conditions are met.
    ///
    /// @brief Evaluate a metric and fire alerts as needed
    /// @param name Name of the metric to evaluate
    /// @param value Current value of the metric
    /// @since 0.1.0
    /// @see AlertRule::evaluate
    /// @see Notifier::notify
    pub async fn evaluate_metric(&self, _name: String, value: f64) -> Result<()> {
        let rules = self.rules.read();
        for rule in rules.iter() {
            if let Some(mut alert) = rule.evaluate(value) {
                // Mark alert as active and record in history
                alert.is_active = true;
                {
                    let mut history = self.history.write();
                    history.push_front(alert.clone());
                }

                // Deliver alert if notifier is configured
                if let Some(ref notifier) = self.notifier {
                    let _ = notifier.notify(&alert).await;
                }
            }
        }
        Ok(())
    }

    /// Returns all currently active alerts.
    ///
    /// @brief Get active alerts
    /// @return Vector of active alerts
    /// @since 0.1.0
    pub fn active_alerts(&self) -> Vec<Alert> {
        let history = self.history.read();
        history.iter().filter(|a| a.is_active).cloned().collect()
    }

    /// Clears all resolved alerts (sets `is_active` to false).
    ///
    /// @brief Clear all resolved alerts
    /// @since 0.1.0
    pub fn clear_resolved(&self) {
        let mut history = self.history.write();
        for alert in history.iter_mut() {
            alert.is_active = false;
        }
        // Optional: prune old resolved alerts to prevent unbounded growth
        history.retain(|alert| alert.is_active);
    }

    /// Returns the alert history.
    ///
    /// @brief Get alert history
    /// @return Vector of alert history (newest first)
    /// @since 0.1.0
    pub fn history(&self) -> Vec<Alert> {
        let history = self.history.read();
        history.iter().cloned().collect()
    }
}