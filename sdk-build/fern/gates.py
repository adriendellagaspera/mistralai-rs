#!/usr/bin/env python3
from __future__ import annotations

import collections
import hashlib
import json
import re
from pathlib import Path
from typing import Any

import yaml

from policy import operations

EXPECTED_OPERATIONS = 288
TYPE_RE = re.compile(r"\bpub\s+(?:struct|enum|type)\s+([A-Za-z_][A-Za-z0-9_]*)")
METHOD_RE = re.compile(r"\bpub\s+async\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)")
HTTP_PAIR_RE = re.compile(
    r"Method::(GET|PUT|POST|DELETE|OPTIONS|HEAD|PATCH|TRACE),\s*"
    r"(?:&format!\()?\s*\"([^\"]+)\"",
    re.MULTILINE,
)
SOURCE_PARAM_RE = re.compile(r"\{[^{}]+\}")
GENERATED_PARAM_RE = re.compile(r"\{\}")


def op_inventory(doc: dict[str, Any]) -> list[dict[str, Any]]:
    result = []
    for path, method, op in operations(doc):
        result.append({
            "method": method.upper(),
            "path": path,
            "operation_id": op.get("operationId"),
            "group": op.get("x-fern-sdk-group-name") or [],
            "sdk_method": op.get("x-fern-sdk-method-name"),
            "streaming": op.get("x-fern-streaming"),
        })
    return sorted(result, key=lambda x: (x["path"], x["method"], x["operation_id"] or ""))


def source_pair(method: str, path: str) -> tuple[str, str]:
    return method.upper(), SOURCE_PARAM_RE.sub("{}", path.lstrip("/"))


def generated_pairs(generated: Path) -> collections.Counter[tuple[str, str]]:
    pairs: collections.Counter[tuple[str, str]] = collections.Counter()
    for path in (generated / "src/api/resources").rglob("*.rs"):
        for method, raw_path in HTTP_PAIR_RE.findall(path.read_text(errors="replace")):
            pairs[(method, GENERATED_PARAM_RE.sub("{}", raw_path))] += 1
    return pairs


def public_types(generated: Path) -> tuple[list[str], dict[str, list[str]]]:
    names: list[str] = []
    locations: dict[str, list[str]] = collections.defaultdict(list)
    root = generated / "src/api/types"
    for path in sorted(root.rglob("*.rs")):
        rel = str(path.relative_to(generated))
        for name in TYPE_RE.findall(path.read_text(errors="replace")):
            names.append(name)
            locations[name].append(rel)
    return names, locations


def public_methods(generated: Path) -> list[dict[str, str]]:
    result = []
    for path in sorted((generated / "src/api/resources").rglob("*.rs")):
        rel = str(path.relative_to(generated))
        for name in METHOD_RE.findall(path.read_text(errors="replace")):
            result.append({"name": name, "path": rel})
    return result


def require_contains(path: Path, *needles: str) -> None:
    text = path.read_text()
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise ValueError(f"{path}: missing production contract markers {missing}")


def validate_transport_contracts(generated: Path) -> None:
    require_contains(
        generated / "src/api/resources/chat/chat.rs",
        "pub async fn complete(",
        "pub async fn complete_stream(",
        ".execute_request(",
        ".execute_sse_request(",
    )
    require_contains(
        generated / "src/api/resources/fim/fim.rs",
        "pub async fn complete(",
        "pub async fn complete_stream(",
        ".execute_request(",
        ".execute_sse_request(",
    )
    require_contains(
        generated / "src/api/resources/files/files.rs",
        "pub async fn download(",
        "Result<ByteStream, ApiError>",
        ".execute_stream_request(",
    )
    require_contains(
        generated / "src/api/resources/audio/speech/audio_speech.rs",
        "pub async fn create(",
    )
    all_types = "\n".join(
        p.read_text(errors="replace") for p in (generated / "src/api/types").rglob("*.rs")
    )
    if "pub audio_data: String" not in all_types:
        raise ValueError("Speech JSON audio_data contract disappeared")
    require_contains(
        generated / "src/api/resources/audio/voices/audio_voices.rs",
        "pub async fn get_sample(",
        "Result<ByteStream, ApiError>",
        ".execute_stream_request(",
    )
    require_contains(
        generated / "src/api/resources/audio/transcriptions/audio_transcriptions.rs",
        "pub async fn complete(",
        "pub async fn stream(",
        ".execute_multipart_sse_request::<TranscriptionStreamEvents>(",
    )
    require_contains(
        generated / "src/core/http_client.rs",
        "pub async fn execute_multipart_sse_request<T>(",
        "https://github.com/fern-api/fern/issues/17928",
    )


def validate(
    canonical_path: Path,
    projected_path: Path,
    generated: Path,
    inventory_path: Path,
    input_workarounds: list[str],
    output_workarounds: list[str],
) -> dict[str, Any]:
    canonical = yaml.safe_load(canonical_path.read_text())
    projected = yaml.safe_load(projected_path.read_text())
    before = op_inventory(canonical)
    after = op_inventory(projected)

    if len(before) != EXPECTED_OPERATIONS or len(after) != EXPECTED_OPERATIONS:
        raise ValueError(
            f"Expected {EXPECTED_OPERATIONS} operations, got canonical={len(before)}, projected={len(after)}"
        )

    before_ids = {(x["method"], x["path"], x["operation_id"]) for x in before}
    after_ids = {(x["method"], x["path"], x["operation_id"]) for x in after}
    if before_ids != after_ids:
        raise ValueError("Product/compat projection changed the canonical operation set")

    missing_policy = [x["operation_id"] for x in after if not x["group"] or not x["sdk_method"]]
    if missing_policy:
        raise ValueError(f"Operations missing Fern public grouping/method identity: {missing_policy}")

    collisions: dict[tuple[tuple[str, ...], str], list[str]] = collections.defaultdict(list)
    for op in after:
        collisions[(tuple(op["group"]), op["sdk_method"])].append(op["operation_id"])
    collisions = {k: v for k, v in collisions.items() if len(v) > 1}
    if collisions:
        raise ValueError(f"Public resource/method collisions: {collisions}")

    generated_counts = generated_pairs(generated)
    missing_generated = []
    for op in before:
        if generated_counts[source_pair(op["method"], op["path"])] < 1:
            missing_generated.append({
                "operation_id": op["operation_id"],
                "method": op["method"],
                "path": op["path"],
            })
    if missing_generated:
        raise ValueError(f"Fern silently dropped source operations: {missing_generated}")

    types, type_locations = public_types(generated)
    duplicate_types = {name: locs for name, locs in type_locations.items() if len(locs) > 1}
    if duplicate_types:
        raise ValueError(f"Generated public type-name collisions: {duplicate_types}")
    if "JudgeOutput" not in type_locations or "JudgeOutputConfig" not in type_locations:
        raise ValueError("Known JudgeOutput collision guard is no longer represented as two public types")

    methods = public_methods(generated)
    validate_transport_contracts(generated)

    inventory = {
        "schema_version": 1,
        "canonical_operation_count": len(before),
        "generated_accounted_operation_count": len(before) - len(missing_generated),
        "fern_cli_version": "5.112.0",
        "fern_rust_sdk_version": "0.48.0",
        "operations": after,
        "public_methods": methods,
        "public_types": sorted(types),
        "workarounds": sorted(set(input_workarounds + output_workarounds)),
    }
    encoded = json.dumps(inventory, indent=2, sort_keys=True) + "\n"
    if inventory_path.exists() and inventory_path.read_text() != encoded:
        raise ValueError(f"Machine-readable public inventory is stale: {inventory_path}")
    return inventory


def write_inventory(path: Path, inventory: dict[str, Any]) -> None:
    path.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")


def tree_digest(root: Path) -> str:
    h = hashlib.sha256()
    for path in sorted(p for p in root.rglob("*") if p.is_file() and ".fern" not in p.parts):
        h.update(str(path.relative_to(root)).encode())
        h.update(b"\0")
        h.update(path.read_bytes())
        h.update(b"\0")
    return h.hexdigest()
