#!/usr/bin/env bash
set -euo pipefail

echo "==> Running benchmarks..."
cd src/rust
cargo bench --features python,nodejs
cd ../..

echo "==> Benchmarks complete!"
