//! # Gateway CLI Module
//!
//! `cli` provides command-line interface.
//!
//! Path: `xiaoyi::gateway::cli`
//!
//! @module gateway::cli
//! @brief Command-line interface
//! @group User Interface
//! @since 0.1.0
//! @author Miruamel
//! @see crate::gateway

use clap::Parser;

/// CLI arguments.
///
/// @brief CLI argument parser
/// @group User Interface
/// @since 0.1.0
#[derive(Parser, Debug, Clone)]
#[command(name = "xiaoyi", version, about = "Xiaoyi AI Agent Framework")]
pub struct CliArgs {
    /// Config file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Agent to run
    #[arg(short, long)]
    pub agent: Option<String>,

    /// Run in daemon mode
    #[arg(long)]
    pub daemon: bool,
}

/// Parse and run CLI.
///
/// @return Exit code
/// @since 0.1.0
pub fn run() -> crate::xiaoyi::core::error::Result<i32> {
    let args = CliArgs::parse();
    println!("Starting Xiaoyi with args: {:?}", args);
    Ok(0)
}
