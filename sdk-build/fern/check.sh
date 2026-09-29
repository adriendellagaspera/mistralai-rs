#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
bash "$ROOT/sdk-build/fern/build.sh"

diff -qr "$ROOT/sdk-build/fern/generated/src" "$ROOT/src"
cmp \
  "$ROOT/sdk-build/fern/inventory.candidate.json" \
  "$ROOT/sdk-build/fern/inventory.json"
