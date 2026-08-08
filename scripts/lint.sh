#!/usr/bin/env bash
set -euo pipefail

echo "==> Running Rust linters..."
cd src/rust
cargo fmt -- --check
cargo clippy --features python,nodejs -- -D warnings
cd ../..

echo "==> Linting passed!"
