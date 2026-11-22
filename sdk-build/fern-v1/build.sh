#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LEGACY_ROOT="$ROOT/sdk-build/fern"
WORKSPACE="$ROOT/sdk-build/fern-v1"
FERN_DIR="$WORKSPACE/fern"
SURFACE_TOOL="$ROOT/sdk-build/sdk_surface/sdk_surface.py"

python3 "$LEGACY_ROOT/prepare_fern_source.py" \
  "$ROOT/sdk-build/openapi/published.yaml" \
  "$LEGACY_ROOT/compatible.yaml" \
  "$LEGACY_ROOT/compatibility-report.json"

python3 "$SURFACE_TOOL" compile \
  --source "$LEGACY_ROOT/compatible.yaml" \
  --policy "$LEGACY_ROOT/policy.yaml" \
  --output "$FERN_DIR/openapi.json" \
  --resolution "$WORKSPACE/resolution.json"

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

  rm -rf "$FERN_DIR/generated"
  (
    cd "$WORKSPACE"
    fern generate --target rust --local --no-prompt --force --log-level info
  )
  test -f "$FERN_DIR/generated/Cargo.toml"
  cargo check --manifest-path "$FERN_DIR/generated/Cargo.toml" --all-features

  python3 - "$FERN_DIR/generated/src" "$WORKSPACE/query-types-$style.txt" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
names = set()
for path in root.rglob("*.rs"):
    source = path.read_text(errors="replace")
    names.update(re.findall(r"pub\\s+(?:struct|enum|type)\\s+([A-Za-z0-9_]*QueryRequest)\\b", source))
Path(sys.argv[2]).write_text("\\n".join(sorted(names)) + ("\\n" if names else ""))
print(f"{Path(sys.argv[2]).name}: {len(names)} public QueryRequest types")
for name in sorted(names):
    print(name)
PY

  mv "$FERN_DIR/generated" "$FERN_DIR/generated-$style"
}

run_generation wrapped
run_generation inline

diff -u "$WORKSPACE/query-types-wrapped.txt" "$WORKSPACE/query-types-inline.txt" || true

echo "Fern Rust generator 1.0.0 experiment completed."
