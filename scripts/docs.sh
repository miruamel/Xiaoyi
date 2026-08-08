#!/usr/bin/env bash
set -euo pipefail

echo "==> Building documentation..."
cd src/rust
cargo doc --no-deps --document-private-items --open
cd ../..
