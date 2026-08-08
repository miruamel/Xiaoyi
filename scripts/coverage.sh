#!/usr/bin/env bash
set -euo pipefail

echo "==> Running coverage..."
cd src/rust
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
cd ../..

echo "==> Coverage report: src/rust/lcov.info"
