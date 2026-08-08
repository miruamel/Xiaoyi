use super::{Budget, CostEstimate};
use crate::xiaoyi::core::error::Result;
use parking_lot::RwLock;

/// Cost tracking structure for monitoring and managing resource consumption against budgets.
///
/// @brief Cost tracker that records estimates and enforces budget constraints
/// @group Monitoring
/// @since 0.1.0
/// @author Miruamel
/// @see CostEstimate
/// @see Budget
/// @see crate::xiaoyi::core::error
pub struct CostTracker {
    /// Budget constraint with thread-safe read/write access.
    budget: RwLock<Budget>,
    /// Historical ledger of cost estimates.
    ledger: Vec<CostEstimate>,
}

impl CostTracker {
    /// Creates a new cost tracker with an initial budget limit.
    ///
    /// @brief Initialize a cost tracker with specified budget limit
    /// @param limit_usd Maximum allowable spending in USD
    /// @return CostTracker New instance with empty ledger and given budget
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn new(limit_usd: f64) -> Self {
        Self {
            budget: RwLock::new(Budget::new(limit_usd)),
            ledger: Vec::new(),
        }
    }

    /// Records a cost estimate against the budget.
    ///
    /// @brief Add a cost estimate to ledger and update budget if within limits
    /// @param estimate CostEstimate to record
    /// @return Result<()> Success or ErrorKind::Policy if budget would be exceeded
    /// @throw ErrorKind::Policy "Budget exceeded"
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn record(&mut self, estimate: CostEstimate) -> Result<()> {
        let mut budget = self.budget.write();
        budget.add_spent(estimate.cost_usd)?;
        self.ledger.push(estimate);
        Ok(())
    }

    /// Calculates total spent according to current budget state.
    ///
    /// @brief Compute total amount spent based on budget tracking
    /// @return f64 Current total spent in USD
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn total_spent(&self) -> f64 {
        let budget = self.budget.read();
        budget.spent_usd
    }

    /// Resets the budget tracker and ledger to initial state.
    ///
    /// @brief Clear all recorded costs and reset budget to zero spent
    /// @return Result<()> Success of reset operation
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn reset(&mut self) -> Result<()> {
        let mut budget = self.budget.write();
        budget.spent_usd = 0.0;
        self.ledger.clear();
        Ok(())
    }

    /// Accesses a read-only view of the budget.
    ///
    /// @brief Retrieve current budget state without mutation
    /// @return Budget Current budget limits and spending
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn get_budget(&self) -> Budget {
        let budget = self.budget.read();
        budget.clone()
    }

    /// Accesses a read-only view of the cost estimate ledger.
    ///
    /// @brief Retrieve immutable reference to cost estimate history
    /// @return &[CostEstimate] Slice of recorded cost estimates
    /// @group Monitoring
    /// @since 0.1.0
    /// @author Miruamel
    pub fn get_ledger(&self) -> &[CostEstimate] {
        &self.ledger
    }
}