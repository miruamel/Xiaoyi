#!/usr/bin/env bash
set -euo pipefail

echo "==> Formatting code..."
cd src/rust
cargo fmt
cd ../..

echo "==> Format complete!"
