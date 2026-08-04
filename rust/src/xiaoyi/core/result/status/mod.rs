//! # Layer 0 - Foundation / Core Result Status
//!
//! `status` models the outcome of an operation before it is lifted into a typed
//! `Result<T>` wrapper. This separation allows workflow and orchestration layers
//! to classify success states independently from error states.
//!
//! Path: `xiaoyi::core::result::status`
//!
//! - Layer 0: `core`
//! - Layer 1: `result`
//! - Layer 2: `status`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,
    Partial,
    Skipped,
    Cancelled,
}

impl Status {
    pub fn is_terminal(self) -> bool {
        matches!(self, Status::Success | Status::Cancelled)
    }

    pub fn can_retry(self) -> bool {
        matches!(self, Status::Partial)
    }
}
