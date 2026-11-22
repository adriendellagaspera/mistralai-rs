#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LEGACY_ROOT="$ROOT/sdk-build/fern"
WORKSPACE="$ROOT/sdk-build/fern-v1"
FERN_DIR="$WORKSPACE/fern"
SURFACE_TOOL="$ROOT/sdk-build/sdk_surface/sdk_surface.py"
COMPILE_STATUS=0

python3 "$LEGACY_ROOT/prepare_fern_source.py" \
  "$ROOT/sdk-build/openapi/published.yaml" \
  "$LEGACY_ROOT/compatible.yaml" \
  "$LEGACY_ROOT/compatibility-report.json"

python3 "$SURFACE_TOOL" compile \
  --source "$LEGACY_ROOT/compatible.yaml" \
  --policy "$LEGACY_ROOT/policy.yaml" \
  --output "$FERN_DIR/openapi.json" \
  --resolution "$WORKSPACE/resolution.json"

inventory_query_types() {
  local style="$1"
  python3 - "$FERN_DIR/generated/src" "$WORKSPACE/query-types-$style.txt" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
names = set()
for path in root.rglob("*.rs"):
    source = path.read_text(errors="replace")
    names.update(re.findall(r"pub\s+(?:struct|enum|type)\s+([A-Za-z0-9_]*QueryRequest)\b", source))
Path(sys.argv[2]).write_text("\n".join(sorted(names)) + ("\n" if names else ""))
print(f"{Path(sys.argv[2]).name}: {len(names)} public QueryRequest types")
for name in sorted(names):
    print(name)
PY
}

run_generation() {
  local style="$1"
  python3 - "$FERN_DIR/sdk-config.yml" "$style" <<'PY'
import sys
from pathlib import Path
import yaml

path = Path(sys.argv[1])
style = sys.argv[2]
config = yaml.safe_load(path.read_text())
config.setdefault("client", {})["requestParameterStyle"] = style
config["targets"][0]["output"]["path"] = "generated"
path.write_text(yaml.safe_dump(config, sort_keys=False))
PY

  rm -rf "$FERN_DIR/generated" "$FERN_DIR/generated-$style"
  (
    cd "$WORKSPACE"
    fern generate --target rust --guest --force --log-level info
  ) 2>&1 | tee "$WORKSPACE/fern-generate-$style.log"
  test -f "$FERN_DIR/generated/Cargo.toml"

  # Inventory before compilation: a broken generated crate must not hide the
  # request type names this experiment is intended to compare.
  inventory_query_types "$style"
  mv "$FERN_DIR/generated" "$FERN_DIR/generated-$style"

  if cargo check --manifest-path "$FERN_DIR/generated-$style/Cargo.toml" --all-features \
    2>&1 | tee "$WORKSPACE/cargo-check-$style.log"; then
    echo "cargo check ($style): success"
  else
    local status=$?
    echo "cargo check ($style): failed with exit code $status" >&2
    COMPILE_STATUS=1
  fi
}

run_generation wrapped
run_generation inline

diff -u "$WORKSPACE/query-types-wrapped.txt" "$WORKSPACE/query-types-inline.txt" \
  | tee "$WORKSPACE/query-types.diff" || true

echo "Fern Rust generator 1.0.0 experiment completed (compile status: $COMPILE_STATUS)."
exit "$COMPILE_STATUS"
