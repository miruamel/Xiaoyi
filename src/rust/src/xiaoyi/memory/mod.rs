//! # Memory Systems
//!
//! `memory` provides short-term (STM) and long-term (LTM) memory systems
//! for agent state and knowledge persistence.
//!
//! Path: `xiaoyi::memory`
//!
//! - Layer 0: `memory` — memory abstraction.
//! - Layer 1: `stm` — short-term memory (cache, buffer).
//! - Layer 2: `ltm` — long-term memory (vector, graph, sqlite).
//!
//! @module memory
//! @brief STM and LTM memory systems
//! @group Memory
//! @since 0.1.0
//! @author Miruamel
//! @see crate::memory::stm
//! @see crate::memory::ltm
pub mod stm;
