#!/usr/bin/env bash
set -euo pipefail

echo "==> Cleaning build artifacts..."
cd src/rust
cargo clean
cd ../..

echo "==> Clean complete!"
