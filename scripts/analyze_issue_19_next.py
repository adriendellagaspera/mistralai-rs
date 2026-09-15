#!/usr/bin/env python3
from __future__ import annotations

import json
from collections import Counter
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))

import sdk_autoproject
import sdk_codegen
from rust_types import parse_type

openapi = sdk_codegen.OpenApiIndex.load(ROOT / "spec/openapi.yaml")
rust = sdk_codegen.RustIndex.load(ROOT / "src/generated")
coverage = json.loads((ROOT / "src/sdk/coverage.json").read_text())
rejected = coverage["automatic_projection"]["rejected"]
targets = sorted(op for op, reason in rejected.items() if reason == "request_model_projection")


def describe(schema: dict) -> str:
    schema, nullable = sdk_autoproject._nullable(schema)
    prefix = "nullable:" if nullable else ""
    ref = sdk_autoproject._schema_ref(schema)
    if ref:
        target = openapi.schemas.get(ref, {})
        if target.get("enum") is not None or target.get("const") is not None:
            return prefix + f"ref_enum:{ref}"
        if target.get("oneOf") or target.get("anyOf"):
            return prefix + f"ref_union:{ref}"
        if target.get("type") == "object" and target.get("properties"):
            return prefix + f"ref_object:{ref}"
        if target.get("type") == "object" and target.get("additionalProperties"):
            return prefix + f"ref_map:{ref}"
        return prefix + f"ref_other:{ref}"
    if schema.get("enum") is not None or schema.get("const") is not None:
        return prefix + "enum"
    if schema.get("oneOf") or schema.get("anyOf"):
        return prefix + "union"
    kind = schema.get("type")
    if kind == "array":
        return prefix + "array<" + describe(schema.get("items", {})) + ">"
    if kind == "object" and schema.get("properties"):
        return prefix + "inline_object"
    if kind == "object" and schema.get("additionalProperties"):
        return prefix + "map"
    if kind in {"string", "integer", "number", "boolean"}:
        return prefix + (f"scalar:{schema.get('format')}" if schema.get("format") else "scalar")
    return prefix + f"other:{kind}"


def core_type(raw_type: str) -> str:
    syntax = parse_type(raw_type)
    while (inner := syntax.unary("Option")) is not None:
        syntax = inner
    while (inner := syntax.unary("Vec")) is not None:
        syntax = inner
    return syntax.spelling


def payload_details(name: str) -> str:
    schema = openapi.schemas.get(name, {})
    required = schema.get("required", [])
    properties = ",".join(f"{field}:{describe(value)}" for field, value in sorted(schema.get("properties", {}).items()))
    if name in rust.structs:
        raw = ",".join(f"{field.name.removeprefix('r#')}:{field.type}" for field in rust.fields(name))
        kind = "struct"
    elif name in rust.aliases:
        raw = rust.aliases[name].spelling
        kind = "alias"
    elif name in rust.enums:
        raw = ",".join(f"{variant.name}:{variant.payload or '-'}" for variant in rust.variants(name))
        kind = "enum"
    else:
        raw = "missing"
        kind = "missing"
    return f"{name} {kind} required={required} schema=[{properties}] raw=[{raw}]"


def union_details(schema: dict, raw_type: str) -> tuple[str, list[str]] | None:
    schema, _ = sdk_autoproject._nullable(schema)
    while schema.get("type") == "array":
        schema = schema.get("items", {})
    ref = sdk_autoproject._schema_ref(schema)
    if ref:
        schema = openapi.schemas.get(ref, {})
    branches = schema.get("oneOf", []) or schema.get("anyOf", [])
    if not branches:
        return None
    raw_union = core_type(raw_type)
    variants = rust.enums.get(raw_union)
    raw_desc = "not-enum" if variants is None else ",".join(
        f"{variant.name}:{variant.payload or '-'}" for variant in variants
    )
    branch_desc = []
    payload_rows = []
    for branch in branches:
        branch_ref = sdk_autoproject._schema_ref(branch)
        if branch_ref:
            branch_desc.append(f"ref:{branch_ref}")
            payload_rows.append(payload_details(branch_ref))
        else:
            branch_desc.append(describe(branch))
    discriminator = schema.get("discriminator", {})
    mapping = discriminator.get("mapping", {}) if isinstance(discriminator, dict) else {}
    return f"raw={raw_union}[{raw_desc}] branches=[{','.join(branch_desc)}] mapping={mapping}", payload_rows


summary = Counter()
print(f"remaining request_model_projection operations: {len(targets)}")
for operation_id in targets:
    operation = openapi.operation(operation_id)
    request_schema, reason = sdk_autoproject._request_json_schema(operation)
    if reason or request_schema is None:
        print(f"{operation_id}\trequest:{reason or 'missing'}")
        summary[reason or "missing_request"] += 1
        continue
    raw = sdk_autoproject._schema_ref(request_schema)
    if not raw or raw not in rust.structs:
        print(f"{operation_id}\traw:{raw}\tnot_struct")
        summary["request_not_struct"] += 1
        continue
    schema = openapi.schemas[raw]
    raw_fields = {field.name.removeprefix("r#"): field for field in rust.fields(raw)}
    failures = []
    union_rows = []
    models = {}
    for field, field_schema in sorted(schema.get("properties", {}).items()):
        raw_field = raw_fields.get(field)
        if raw_field is None:
            failures.append((field, "missing_raw_field", describe(field_schema), "<missing>"))
            continue
        _, failure = sdk_autoproject._request_field_adapter(
            models, openapi.schemas, field_schema, raw_field.type, rust, (raw,)
        )
        if failure:
            failures.append((field, failure, describe(field_schema), raw_field.type))
            details = union_details(field_schema, raw_field.type)
            if details:
                union_rows.append((field, *details))
    if not failures:
        failures.append(("<model>", "model_level_mismatch", "-", "-"))
    categories = sorted({shape.split(":", 1)[0] for _, _, shape, _ in failures})
    for category in categories:
        summary[category] += 1
    details = "; ".join(f"{field}={shape} -> {raw_type}" for field, _, shape, raw_type in failures)
    print(f"{operation_id}\t{','.join(categories)}\t{details}")
    for field, union, payloads in union_rows:
        print(f"  UNION {field}: {union}")
        for payload in payloads:
            print(f"    PAYLOAD {payload}")

print("SUMMARY")
for key, count in summary.most_common():
    print(f"{key}\t{count}")
