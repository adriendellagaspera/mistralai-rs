#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
report = (HERE / "API_COMPATIBILITY.md").read_text()
inv = json.loads((HERE / "public-inventory.json").read_text())
if inv["canonical_operation_count"] != 288 or inv["generated_accounted_operation_count"] != 288:
    raise SystemExit("API inventory is not 288/288")
for required in ("Mistral", "mistralai::raw", "chat", "FIM", "files.download", "audio_data", "ByteStream"):
    if required not in report:
        raise SystemExit(f"compatibility report missing {required!r}")
print(
    f"api-compatibility: ok ({len(inv['public_methods'])} methods, "
    f"{len(inv['public_types'])} types, 288/288 operations)"
)
