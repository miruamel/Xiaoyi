//! # Workflow DAG
//!
//! `workflow` provides a directed acyclic graph (DAG) execution engine
//! for workflow orchestration.
//!
//! Path: `xiaoyi::workflow`
//!
//! - Layer 0: `workflow` — workflow orchestration.
//! - Layer 1: `dag` — DAG structure and execution.
//! - Layer 2: `graph` — graph implementation.
//! - Layer 3: `node`/`edge`/`cycle` — graph primitives.
//!
//! @module workflow
//! @brief DAG-based workflow execution engine
//! @group Orchestration
//! @since 0.1.0
//! @author Miruamel
//! @see crate::workflow::dag
//! @see crate::orchestrator
pub mod dag;