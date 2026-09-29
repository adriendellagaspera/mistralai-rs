#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FERN = Path(sys.argv[1])
OUT = Path(sys.argv[2])

ASYNC_FN = re.compile(
    r"pub\s+async\s+fn\s+([A-Za-z0-9_]+)\s*(?:<[^>{}]*>)?\s*"
    r"\(([\s\S]*?)\)\s*->\s*([^\{\n]+)"
)
PUB_TYPE = re.compile(r"pub\s+(?:struct|enum|type)\s+([A-Za-z0-9_]+)")
RAW_ESCAPE = re.compile(r"pub\s+fn\s+(?:raw|as_raw|into_raw|from_raw)\b")


def pct(values: list[int], p: float) -> int:
    if not values:
        return 0
    values = sorted(values)
    return values[min(len(values) - 1, round((len(values) - 1) * p))]


def scan(paths: list[Path], base: Path) -> dict:
    methods = []
    types = []
    raw_escapes = 0
    generated_path_mentions = 0
    for path in paths:
        text = path.read_text()
        rel = str(path.relative_to(base))
        raw_escapes += len(RAW_ESCAPE.findall(text))
        generated_path_mentions += text.count("crate::generated::")
        for match in ASYNC_FN.finditer(text):
            methods.append(
                {
                    "name": match.group(1),
                    "params": re.sub(r"\s+", " ", match.group(2)).strip(),
                    "return": re.sub(r"\s+", " ", match.group(3)).strip(),
                    "path": rel,
                }
            )
        for match in PUB_TYPE.finditer(text):
            types.append({"name": match.group(1), "path": rel})

    method_lengths = [len(x["name"]) for x in methods]
    type_lengths = [len(x["name"]) for x in types]
    return {
        "file_count": len(paths),
        "public_async_fn_count": len(methods),
        "public_type_count": len(types),
        "raw_escape_fn_count": raw_escapes,
        "generated_path_mentions": generated_path_mentions,
        "method_name_length": {
            "median": statistics.median(method_lengths) if method_lengths else 0,
            "p95": pct(method_lengths, 0.95),
            "max": max(method_lengths, default=0),
        },
        "type_name_length": {
            "median": statistics.median(type_lengths) if type_lengths else 0,
            "p95": pct(type_lengths, 0.95),
            "max": max(type_lengths, default=0),
        },
        "request_options_methods": sum(
            "Option<RequestOptions>" in x["params"] for x in methods
        ),
        "operation_shaped_method_names": sum(
            "_v1" in x["name"]
            or "v1" in x["name"]
            or "api_routes" in x["name"]
            or x["name"].startswith(("users_api_", "jobs_api_", "agents_api_"))
            for x in methods
        ),
        "stream_return_methods": sum(
            "Stream" in x["return"] or "SseStream" in x["return"] for x in methods
        ),
        "longest_methods": sorted(
            methods, key=lambda x: len(x["name"]), reverse=True
        )[:25],
        "longest_types": sorted(types, key=lambda x: len(x["name"]), reverse=True)[:25],
        "methods": methods,
    }


def methods_for_suffix(scan_result: dict, suffix: str) -> list[dict]:
    return [
        x
        for x in scan_result["methods"]
        if x["path"].endswith(suffix)
    ]


current_paths = sorted((ROOT / "src/sdk").glob("*.rs"))
fern_paths = sorted(FERN.rglob("*.rs"))
current = scan(current_paths, ROOT)
fern = scan(fern_paths, FERN)

samples = {}
for name in (
    "chat",
    "files",
    "audio_transcriptions",
    "beta_conversations",
    "workflows_executions",
    "embeddings",
    "ocr",
):
    current_suffix = f"src/sdk/{name}.rs"
    fern_suffix = f"/{name}/{name}.rs"
    samples[name] = {
        "current": methods_for_suffix(current, current_suffix),
        "fern": [
            x for x in fern["methods"] if x["path"].endswith(fern_suffix)
        ],
    }

for data in (current, fern):
    data.pop("methods")

report = {
    "current_main_facade": current,
    "fern_generated": fern,
    "samples": samples,
}
OUT.write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({k: v for k, v in report.items() if k != "samples"}, indent=2))
for name, value in samples.items():
    print(f"\n## {name}")
    print("current:", [x["name"] for x in value["current"]])
    print("fern:", [x["name"] for x in value["fern"]])
