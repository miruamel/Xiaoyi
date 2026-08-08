use crate::xiaoyi::core::error::{XiaoyiError, ErrorKind};
use crate::xiaoyi::core::error::Result;
use serde::{Deserialize, Serialize};

/// Budget tracking structure for controlling spending limits.
///
/// @brief Budget management with limit and spend tracking
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see CostTracker::record
/// @see crate::xiaoyi::core::error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    /// Maximum allowable spending in USD for this budget.
    pub limit_usd: f64,
    /// Current amount spent in USD.
    pub spent_usd: f64,
}

impl Budget {
    /// Creates a new budget with specified limit.
    ///
    /// @brief Initialize a new budget with spending limit
    /// @param limit_usd Maximum spending limit in USD
    /// @return Budget New budget instance with zero spent amount
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn new(limit_usd: f64) -> Self {
        Self {
            limit_usd,
            spent_usd: 0.0,
        }
    }

    /// Calculates remaining budget capacity.
    ///
    /// @brief Compute available budget (limit - spent)
    /// @return f64 Remaining budget amount in USD
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn remaining(&self) -> f64 {
        self.limit_usd - self.spent_usd
    }

    /// Checks if a proposed spending would exceed the budget limit.
    ///
    /// @brief Determine if additional spending would violate budget constraints
    /// @param amount Proposed additional spending amount in USD
    /// @return bool true if total spent would exceed limit, false otherwise
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn would_exceed(&self, amount: f64) -> bool {
        self.spent_usd + amount > self.limit_usd
    }

    /// Adds spending to the budget tracker.
    ///
    /// @brief Record actual spending against budget limit
    /// @param amount Amount spent in USD to add to total
    /// @return Result<()> Success or error if operation fails
    /// @throw ErrorKind::Policy "Budget exceeded"
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn add_spent(&mut self, amount: f64) -> Result<()> {
        if self.would_exceed(amount) {
            return Err(XiaoyiError::new(ErrorKind::Policy, format!("Budget exceeded: limit={}, spent={}", self.limit_usd, self.spent_usd + amount)));
        }
        self.spent_usd += amount;
        Ok(())
    }
}