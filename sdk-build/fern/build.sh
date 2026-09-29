#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FERN_ROOT="$ROOT/sdk-build/fern"
SURFACE_TOOL="$ROOT/.tools/sdk-surface/surface/sdk_surface.py"
FERN_BIN="fern"

if [[ ! -f "$SURFACE_TOOL" ]]; then
  echo "SDK surface compiler not found: $SURFACE_TOOL" >&2
  exit 2
fi

python3 "$FERN_ROOT/prepare_fern_source.py" \
  "$ROOT/sdk-build/openapi/published.yaml" \
  "$FERN_ROOT/compatible.yaml" \
  "$FERN_ROOT/compatibility-report.json"

python3 "$SURFACE_TOOL" compile \
  --source "$FERN_ROOT/compatible.yaml" \
  --policy "$FERN_ROOT/policy.yaml" \
  --output "$FERN_ROOT/fern/openapi.json" \
  --resolution "$FERN_ROOT/resolution.json"

(
  cd "$FERN_ROOT"
  "$FERN_BIN" ir fern-ir.json \
    --language rust \
    --disable-examples \
    --from-openapi
)

python3 "$SURFACE_TOOL" verify \
  --source "$FERN_ROOT/compatible.yaml" \
  --policy "$FERN_ROOT/policy.yaml" \
  --fern-ir "$FERN_ROOT/fern-ir.json" \
  --inventory "$FERN_ROOT/inventory.candidate.json"

python3 - "$FERN_ROOT/inventory.candidate.json" <<'PY'
import json
import sys
from pathlib import Path

inventory = json.loads(Path(sys.argv[1]).read_text())
assert inventory["source_operation_count"] == 288, inventory["source_operation_count"]
assert inventory["published_operation_count"] == 288, inventory["published_operation_count"]
assert inventory["excluded_operation_count"] == 0, inventory["excluded_operation_count"]
assert inventory["public_method_count"] >= 288, inventory["public_method_count"]
assert not any(inventory["verification"].values()), inventory["verification"]
PY

normalize_generated_manifest() {
  python3 - "$FERN_ROOT/generated/Cargo.toml" <<'PY'
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
if "\n[workspace]\n" not in f"\n{text}":
    path.write_text(text.rstrip() + "\n\n[workspace]\n")
PY
}

generate() {
  rm -rf "$FERN_ROOT/generated"
  (
    cd "$FERN_ROOT"
    "$FERN_BIN" generate \
      --group rust-sdk \
      --local \
      --no-prompt \
      --force \
      --log-level info
  )
  python3 "$FERN_ROOT/patch_fern_output.py" "$FERN_ROOT/generated"
  # The generated crate lives below sdk-build/, whose retired Rust builder is
  # still a workspace during this one-time cutover. Keep the Fern crate
  # standalone; remove this normalization once that parent manifest is gone.
  normalize_generated_manifest
}

generate
python3 "$SURFACE_TOOL" digest \
  --root "$FERN_ROOT/generated" \
  --output "$FERN_ROOT/digest.first.json"

generate
python3 "$SURFACE_TOOL" digest \
  --root "$FERN_ROOT/generated" \
  --output "$FERN_ROOT/digest.second.json"

cmp "$FERN_ROOT/digest.first.json" "$FERN_ROOT/digest.second.json"

cargo fmt --manifest-path "$FERN_ROOT/generated/Cargo.toml" -- --check
cargo check --manifest-path "$FERN_ROOT/generated/Cargo.toml" --all-features
cargo clippy --manifest-path "$FERN_ROOT/generated/Cargo.toml" --all-features -- \
  -D clippy::correctness \
  -D clippy::suspicious
