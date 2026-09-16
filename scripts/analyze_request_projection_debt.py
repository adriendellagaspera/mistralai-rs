"""Temporary structural analysis for remaining request-model projection debt."""
from collections import Counter, defaultdict
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))

from rust_types import parse_type
from sdk_autoproject import _nullable, _request_json_schema, _schema_ref, _simple_schema, _safe_alias, _raw_public_leaf
from sdk_codegen import OpenApiIndex, RustIndex


def strip_options(type_name: str):
    syntax = parse_type(type_name)
    while (inner := syntax.unary("Option")) is not None:
        syntax = inner
    return syntax


def target_shape(schema):
    if "$ref" in schema:
        return "ref"
    if schema.get("oneOf") or schema.get("anyOf"):
        return "union"
    if "enum" in schema:
        return "enum"
    if "const" in schema:
        return "const"
    if schema.get("format"):
        return f"format:{schema['format']}"
    kind = schema.get("type")
    if kind == "object":
        if schema.get("properties"):
            return "object"
        if schema.get("additionalProperties"):
            return "map"
    return kind or "unknown"


def request_model_failures(raw, schemas, rust, resolving=()):
    if raw in resolving:
        return {"recursive_model"}
    schema = schemas.get(raw)
    if not schema:
        return {"missing_schema"}
    if schema.get("type") != "object":
        return {f"root_{target_shape(schema)}"}
    if raw not in rust.structs:
        return {"raw_not_struct"}
    props = schema.get("properties", {})
    raw_fields = {field.name.removeprefix("r#"): field for field in rust.fields(raw)}
    if set(raw_fields) != set(props):
        return {"field_set_mismatch"}
    failures = set()
    for name, field_schema in sorted(props.items()):
        failures |= field_failures(field_schema, raw_fields[name].type, schemas, rust, (*resolving, raw))
    return failures


def field_failures(schema, raw_type, schemas, rust, resolving):
    schema, _ = _nullable(schema)
    syntax = strip_options(raw_type)
    if schema.get("type") == "array":
        inner = syntax.unary("Vec")
        if inner is None:
            return {"array_raw_mismatch"}
        return field_failures(schema.get("items", {}), inner.spelling, schemas, rust, resolving)
    if schema.get("oneOf") or schema.get("anyOf"):
        branches = schema.get("oneOf", []) or schema.get("anyOf", [])
        refs = [_schema_ref(branch) for branch in branches if branch.get("type") != "null"]
        if len(refs) < 2 or any(ref is None for ref in refs):
            return {"inline_or_mixed_union"}
        if syntax.spelling not in rust.enums:
            return {"union_raw_not_enum"}
        variants = rust.enums[syntax.spelling]
        payloads = [variant.payload for variant in variants]
        if any(payload is None for payload in payloads):
            return {"union_non_unary_raw_variant"}
        if set(payloads) != set(refs) or len(payloads) != len(refs):
            return {"union_payload_mismatch"}
        nested = set()
        for ref in refs:
            nested |= request_model_failures(ref, schemas, rust, resolving)
        return {f"union_branch:{reason}" for reason in nested}
    ref = _schema_ref(schema)
    if ref:
        target = schemas.get(ref, {})
        if target.get("type") == "object" and target.get("properties"):
            if syntax.spelling != ref:
                return {"ref_object_raw_mismatch"}
            return {f"nested:{reason}" for reason in request_model_failures(ref, schemas, rust, resolving)}
        if target.get("type") == "object" and target.get("additionalProperties"):
            if syntax.spelling in rust.aliases:
                return set() if _safe_alias(rust.aliases[syntax.spelling], rust, (syntax.spelling,)) else {"unsafe_map_alias"}
            return set() if _raw_public_leaf(syntax, rust) else {"map_generated_symbol"}
        if _simple_schema(target):
            if syntax.spelling in rust.aliases:
                return set() if _safe_alias(rust.aliases[syntax.spelling], rust, (syntax.spelling,)) else {"unsafe_simple_alias"}
            return set() if _raw_public_leaf(syntax, rust) else {"simple_generated_symbol"}
        return {f"ref_{target_shape(target)}"}
    if schema.get("type") == "object" and schema.get("additionalProperties"):
        if syntax.spelling in rust.aliases:
            return set() if _safe_alias(rust.aliases[syntax.spelling], rust, (syntax.spelling,)) else {"unsafe_inline_map_alias"}
        return set() if _raw_public_leaf(syntax, rust) else {"inline_map_generated_symbol"}
    if _simple_schema(schema):
        if syntax.spelling in rust.aliases:
            return set() if _safe_alias(rust.aliases[syntax.spelling], rust, (syntax.spelling,)) else {"unsafe_inline_simple_alias"}
        return set() if _raw_public_leaf(syntax, rust) else {"inline_simple_generated_symbol"}
    return {f"inline_{target_shape(schema)}"}


def main():
    openapi = OpenApiIndex.load(ROOT / "spec/openapi.yaml")
    rust = RustIndex.load(ROOT / "src/generated")
    sdk_coverage = json.loads((ROOT / "src/sdk/coverage.json").read_text())
    raw_coverage = json.loads((ROOT / "src/generated/coverage.json").read_text())
    raw_methods = {item["operation_id"]: item["rust_method"] for item in raw_coverage["operations"] if item.get("upstream", True)}
    rejected = sdk_coverage["automatic_projection"]["rejected"]
    targets = [op for op, reason in rejected.items() if reason == "request_model_projection"]
    counts = Counter()
    by_reason = defaultdict(list)
    print(f"request_model_projection={len(targets)}")
    for operation_id in targets:
        operation = openapi.operation(operation_id)
        request_schema, reason = _request_json_schema(operation)
        request_raw = _schema_ref(request_schema or {})
        raw_method = raw_methods.get(operation_id)
        raw_operation = rust.operation(raw_method) if raw_method else None
        body_params = [p.type for p in raw_operation.parameters if request_raw and strip_options(p.type).spelling == request_raw] if raw_operation else []
        failures = request_model_failures(request_raw, openapi.schemas, rust) if request_raw else {reason or "no_named_request"}
        if not failures:
            failures = {"projectable_but_rejected"}
        for failure in sorted(failures):
            counts[failure] += 1
            by_reason[failure].append(operation_id)
        print(f"OP {operation_id}")
        print(f"  request={request_raw} raw_method={raw_method} body_params={body_params}")
        print(f"  failures={','.join(sorted(failures))}")
        if request_raw in rust.structs:
            print("  fields=" + ", ".join(f"{f.name}:{f.type}" for f in rust.fields(request_raw)))
    print("COUNTS")
    for reason, count in counts.most_common():
        print(f"  {count:2d} {reason}: {', '.join(by_reason[reason])}")


if __name__ == "__main__":
    main()
