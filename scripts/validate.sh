#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 scripts/codegen.py check
python3 scripts/sdk_taxonomy_inventory.py check
cargo fmt --all --check
cargo check --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
python3 scripts/codegen.py probe
