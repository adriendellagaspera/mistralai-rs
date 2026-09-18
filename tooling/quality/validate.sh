#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 tooling/quality/agent_contract.py
python3 tooling/tests/run_pinned.py
python3 tooling/pipeline/build.py check
python3 tooling/sources/taxonomy_inventory.py check
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
RUSTDOCFLAGS="-D warnings" cargo test --locked --doc
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
