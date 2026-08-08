#!/usr/bin/env bash
set -euo pipefail

echo "==> Setting up Xiaoyi development environment..."

# Rust
if ! command -v cargo >/dev/null 2>&1; then
  echo "Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.99-nightly
  source "$HOME/.cargo/env"
else
  echo "Rust already installed: $(rustc --version)"
fi

# Python
if ! command -v python3 >/dev/null 2>&1; then
  echo "Please install Python 3.12+ manually"
  exit 1
else
  echo "Python: $(python3 --version)"
fi

# Node.js
if ! command -v node >/dev/null 2>&1; then
  echo "Please install Node.js 20+ manually"
  exit 1
else
  echo "Node.js: $(node --version)"
fi

# Build Rust core
echo "==> Building Rust core..."
cd src/rust
cargo build --features python
cd ../..

echo "==> Setup complete!"
