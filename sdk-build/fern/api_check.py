#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
inventory = json.loads((HERE / "inventory.json").read_text())
report = (HERE / "API_COMPATIBILITY.md").read_text()

if inventory["source_operation_count"] != 288:
    raise SystemExit("source inventory is not 288 operations")
if inventory["published_operation_count"] != 288:
    raise SystemExit("published inventory is not 288 operations")
if inventory["excluded_operation_count"] != 0:
    raise SystemExit("unexpected excluded operations")
if any(inventory["verification"].values()):
    raise SystemExit(f"surface verification failed: {inventory['verification']}")

for marker in (
    "Mistral",
    "mistralai::raw",
    "mistralai_sdk",
    "ApiClient",
    "ByteStream",
    "audio_data",
    "fern-api/fern#17928",
):
    if marker not in report:
        raise SystemExit(f"compatibility report missing {marker!r}")

print(
    "api-compatibility: ok "
    f"({inventory['published_operation_count']}/"
    f"{inventory['source_operation_count']} operations)"
)
