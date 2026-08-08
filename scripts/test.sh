#!/usr/bin/env bash
set -euo pipefail

echo "==> Running Rust tests..."
cd src/rust
cargo test --features python,nodejs -- --test-threads=4
cd ../..

echo "==> All tests passed!"
