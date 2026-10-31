#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
inventory = json.loads((HERE / "inventory.json").read_text())
report = (HERE / "API_COMPATIBILITY.md").read_text()
catalog = json.loads((HERE.parent / "openapi/public-catalog.lock.json").read_text())
if catalog.get("reviewed_at") == "PENDING_PR_REVIEW":
    raise SystemExit(
        "The new published OpenAPI catalog has not been reviewed. "
        "A human must verify source provenance and compatibility and replace "
        "PENDING_PR_REVIEW with the actual review date before merge."
    )

if inventory["source_operation_count"] <= 0:
    raise SystemExit("source inventory must contain operations")
if inventory["published_operation_count"] != inventory["source_operation_count"]:
    raise SystemExit("not all source operations were published")
if inventory["excluded_operation_count"] != 0:
    raise SystemExit("unexpected excluded operations")
if any(inventory["verification"].values()):
    raise SystemExit(f"surface verification failed: {inventory['verification']}")

for marker in (
    "Mistral",
    "mistralai::raw",
    "mistralai_sdk",
    "ClientConfig",
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
