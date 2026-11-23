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

python3 - "$WORKSPACE/resolution.json" "$WORKSPACE/public-method-paths.tsv" <<'PY'
import json
import sys
from pathlib import Path

resolution = json.loads(Path(sys.argv[1]).read_text())
rows = []
for operation in resolution["operations"]:
    if operation["outcome"] != "published":
        continue
    for public_path in operation["public_paths"]:
        rows.append(
            "\t".join(
                [
                    operation["http_method"],
                    operation["http_path"],
                    operation.get("operation_id") or "",
                    public_path,
                ]
            )
        )
output = Path(sys.argv[2])
output.write_text("\n".join(sorted(rows)) + "\n")
print(
    f"public method paths: {len(rows)} across "
    f"{resolution['published_operation_count']} published operations"
)
PY

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
    names.update(re.findall(r"pub\s+(?:struct|enum|type)\s+([A-Za-z0-9_]*QueryRequest[0-9]*)\b", source))
Path(sys.argv[2]).write_text("\n".join(sorted(names)) + ("\n" if names else ""))
print(f"{Path(sys.argv[2]).name}: {len(names)} public QueryRequest types")
for name in sorted(names):
    print(name)
PY
}

inventory_public_functions() {
  local style="$1"
  python3 - "$FERN_DIR/generated/src" "$WORKSPACE/public-functions-$style.txt" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
signatures = set()
pattern = re.compile(r"(?m)^\s*pub\s+(?:async\s+)?fn\s+[A-Za-z0-9_]+")
for path in root.rglob("*.rs"):
    source = path.read_text(errors="replace")
    for match in pattern.finditer(source):
        # The first opening brace after the declaration marks the function body.
        # Collapse formatting so harmless line wrapping does not create a diff.
        body_start = source.find("{", match.end())
        if body_start < 0:
            continue
        signature = " ".join(source[match.start():body_start].split())
        relative_path = path.relative_to(root).as_posix()
        signatures.add(f"{relative_path}: {signature}")
output = Path(sys.argv[2])
output.write_text("\n".join(sorted(signatures)) + ("\n" if signatures else ""))
print(f"{output.name}: {len(signatures)} public function signatures")
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
  # request type names or public method signatures being compared.
  inventory_query_types "$style"
  inventory_public_functions "$style"
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
diff -u "$WORKSPACE/public-functions-wrapped.txt" "$WORKSPACE/public-functions-inline.txt" \
  | tee "$WORKSPACE/public-functions.diff" || true

echo "Fern Rust generator 1.0.0 audit completed (compile status: $COMPILE_STATUS; see cargo-check logs)."
# This job validates generation and the naming/signature inventories. Cargo
# failures are retained as diagnostics, but are not a gate for this audit.
exit 0
