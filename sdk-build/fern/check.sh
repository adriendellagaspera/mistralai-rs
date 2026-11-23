#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT/sdk-build/fern/build.sh"

# Check the semantic API inventory before the byte-for-byte source comparison so
# generator diagnostics still establish API parity when generated templates drift.
cmp \
  "$ROOT/sdk-build/fern/inventory.candidate.json" \
  "$ROOT/sdk-build/fern/inventory.json"

diff -qr "$ROOT/sdk-build/fern/generated/src" "$ROOT/src"
