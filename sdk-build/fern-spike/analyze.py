#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import statistics
import sys
from pathlib import Path

import yaml

HTTP_METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}


def percentile(values: list[int], p: float) -> int | None:
    if not values:
        return None
    ordered = sorted(values)
    idx = round((len(ordered) - 1) * p)
    return ordered[idx]


def main() -> int:
    spec_path = Path(sys.argv[1])
    generated = Path(sys.argv[2])

    spec = yaml.safe_load(spec_path.read_text())
    operations = []
    for path, item in (spec.get("paths") or {}).items():
        if not isinstance(item, dict):
            continue
        for method, operation in item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(operation, dict):
                continue
            operations.append(
                {
                    "method": method.upper(),
                    "path": path,
                    "operation_id": operation.get("operationId"),
                }
            )

    rust_files = sorted(generated.rglob("*.rs"))
    source = "\n".join(path.read_text(errors="replace") for path in rust_files)

    public_types = re.findall(r"\bpub\s+(?:struct|enum|type)\s+([A-Za-z_][A-Za-z0-9_]*)", source)
    public_async_fns = re.findall(r"\bpub\s+async\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)", source)
    lengths = [len(name) for name in public_types]

    report = {
        "source_operation_count": len(operations),
        "source_operation_ids": sum(1 for op in operations if op["operation_id"]),
        "generated_rust_file_count": len(rust_files),
        "generated_public_async_fn_count": len(public_async_fns),
        "generated_public_type_count": len(public_types),
        "public_type_name_length": {
            "median": statistics.median(lengths) if lengths else None,
            "p95": percentile(lengths, 0.95),
            "max": max(lengths) if lengths else None,
        },
        "longest_public_types": sorted(public_types, key=lambda value: (len(value), value), reverse=True)[:25],
        "transport_markers": {
            "stream_mentions": len(re.findall(r"\bStream\b", source)),
            "sse_mentions": len(re.findall(r"SSE|ServerSent|EventSource|event_stream", source, re.IGNORECASE)),
            "multipart_mentions": len(re.findall(r"multipart", source, re.IGNORECASE)),
            "bytes_mentions": len(re.findall(r"\bBytes\b", source)),
        },
        "operations": operations,
    }

    out = generated.parent / "fern-spike-report.json"
    out.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "operations"}, indent=2))
    print(f"report={out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
