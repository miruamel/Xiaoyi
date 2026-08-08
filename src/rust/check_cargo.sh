#!/bin/bash
set +o pipefail
exec > /tmp/cargo_raw.log 2>&1
cargo check --features python
echo "RC=$?"
