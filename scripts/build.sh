#!/usr/bin/env bash
set -euo pipefail

echo "==> Building all components..."
cd src/rust
cargo build --features python,nodejs
cd ../..

echo "==> Build complete!"
