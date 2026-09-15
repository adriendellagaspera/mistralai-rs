#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen as compiler
import probe_sdk_coverage as probe

openapi = compiler.OpenApiIndex.load(ROOT / "spec/openapi.yaml")
rust = compiler.RustIndex.load(ROOT / "src/generated")
configured = json.loads((ROOT / "codegen/sdk-semantics.json").read_text())
configured, _ = compiler.expand_manifest(
    openapi,
    configured,
    json.loads((ROOT / "codegen/sdk-taxonomy.json").read_text()),
    json.loads((ROOT / "src/generated/coverage.json").read_text()),
    rust,
)
modules, rejected = probe.probes(openapi, rust, configured)
report = {"generated_candidates": sorted(modules), "rejected_candidates": rejected}
digest = hashlib.sha256(
    json.dumps(report, sort_keys=True, separators=(",", ":")).encode()
).hexdigest()
baseline = {
    "schema_version": 1,
    "generated_candidates": len(modules),
    "rejected_candidates": len(rejected),
    "report_sha256": digest,
}
(ROOT / "codegen/sdk-probe-baseline.json").write_text(json.dumps(baseline, indent=2) + "\n")
print(json.dumps(baseline, indent=2))
