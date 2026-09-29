#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
"$ROOT/sdk-build/fern/build.sh"

rm -rf "$ROOT/src"
cp -R "$ROOT/sdk-build/fern/generated/src" "$ROOT/src"
mv \
  "$ROOT/sdk-build/fern/inventory.candidate.json" \
  "$ROOT/sdk-build/fern/inventory.json"

echo "Fern production SDK materialized in src/"
