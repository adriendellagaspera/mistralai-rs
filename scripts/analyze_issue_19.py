#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from ruamel.yaml import YAML

ROOT = Path(__file__).resolve().parents[1]
spec = YAML(typ="safe").load((ROOT / "spec/openapi.yaml").read_text())
coverage = json.loads((ROOT / "src/sdk/coverage.json").read_text())
rejected = coverage["automatic_projection"]["rejected"]
targets = sorted(op for op, reason in rejected.items() if reason == "request_model_projection")
schemas = spec.get("components", {}).get("schemas", {})
operations: dict[str, dict[str, Any]] = {}
for path, item in spec.get("paths", {}).items():
    for method in ("get", "post", "put", "patch", "delete", "head", "options"):
        op = item.get(method)
        if op and op.get("operationId"):
            operations[op["operationId"]] = op


def unwrap_nullable(schema: dict[str, Any]) -> dict[str, Any]:
    branches = schema.get("anyOf", [])
    non_null = [branch for branch in branches if branch.get("type") != "null"]
    if branches and len(non_null) == 1 and len(non_null) != len(branches):
        return non_null[0]
    return schema


def features(schema: dict[str, Any], seen: tuple[str, ...] = ()) -> set[str]:
    schema = unwrap_nullable(schema)
    ref = schema.get("$ref")
    if isinstance(ref, str):
        name = ref.rsplit("/", 1)[-1]
        if name in seen:
            return {"recursive_ref"}
        target = schemas.get(name, {})
        base = {"ref"}
        if target.get("type") == "object":
            base.add("ref_object")
        if target.get("enum") is not None or target.get("const") is not None:
            base.add("ref_enum")
        if target.get("oneOf") or target.get("anyOf"):
            base.add("ref_union")
        return base | features(target, (*seen, name))
    branches = schema.get("oneOf", []) or schema.get("anyOf", [])
    if branches:
        out = {"union"}
        for branch in branches:
            out |= features(branch, seen)
        return out
    if "enum" in schema or "const" in schema:
        return {"enum"}
    kind = schema.get("type")
    if kind == "array":
        return {"array"} | features(schema.get("items", {}), seen)
    if kind == "object":
        out = {"object"}
        props = schema.get("properties", {})
        if props:
            out.add("object_properties")
            for child in props.values():
                out |= features(child, seen)
        additional = schema.get("additionalProperties")
        if additional:
            out.add("map")
            if isinstance(additional, dict):
                out |= features(additional, seen)
        return out
    if schema.get("format"):
        return {"formatted_scalar"}
    if kind in {"string", "integer", "number", "boolean"}:
        return {"scalar"}
    return {"unknown"}


print(f"request_model_projection operations: {len(targets)}")
summary: dict[str, int] = {}
for operation_id in targets:
    operation = operations[operation_id]
    schema = operation.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema", {})
    ref = schema.get("$ref", "<inline>").rsplit("/", 1)[-1]
    observed = sorted(features(schema))
    for feature in observed:
        summary[feature] = summary.get(feature, 0) + 1
    print(f"{operation_id}\t{ref}\t{','.join(observed)}")
print("SUMMARY")
for feature, count in sorted(summary.items(), key=lambda item: (-item[1], item[0])):
    print(f"{feature}\t{count}")
