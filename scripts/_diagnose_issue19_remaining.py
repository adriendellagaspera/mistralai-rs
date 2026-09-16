import copy
import collections
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_autoproject as auto
import sdk_compiler as compiler
import probe_sdk_coverage as probe


def compact_schema(schema):
    if not isinstance(schema, dict):
        return schema
    result = {}
    for key in ("$ref", "type", "const", "enum", "required", "additionalProperties", "discriminator"):
        if key in schema:
            result[key] = schema[key]
    for key in ("oneOf", "anyOf", "allOf"):
        if key in schema:
            result[key] = [compact_schema(branch) for branch in schema[key]]
    if "properties" in schema:
        result["properties"] = {
            name: compact_schema(value) for name, value in schema["properties"].items()
        }
    if "items" in schema:
        result["items"] = compact_schema(schema["items"])
    return result


def raw_shape(raw, name):
    if name in raw.structs:
        return {"kind": "struct", "fields": [(f.name, f.type) for f in raw.fields(name)]}
    if name in raw.enums:
        return {"kind": "enum", "variants": [
            (v.name, v.payload, v.wire_name) for v in raw.variants(name)
        ]}
    if name in raw.aliases:
        return {"kind": "alias", "type": raw.aliases[name].spelling}
    return {"kind": "missing"}


openapi = compiler.OpenApiIndex.load(ROOT / "spec/openapi.yaml")
raw = compiler.load_raw_ir(ROOT / "src/generated")
manifest = json.loads((ROOT / "codegen/sdk-semantics.json").read_text())
taxonomy = json.loads((ROOT / "codegen/sdk-taxonomy.json").read_text())
raw_coverage = json.loads((ROOT / "src/generated/coverage.json").read_text())
sdk_coverage = json.loads((ROOT / "src/sdk/coverage.json").read_text())
expanded, report = auto.expand_manifest(openapi, manifest, taxonomy, raw_coverage, raw)
coverage_by_id = {entry["operation_id"]: entry for entry in raw_coverage["operations"]}
public_ids = {entry["operation_id"] for entry in raw_coverage["operations"] if entry.get("upstream")}
inv = sdk_coverage["inventory"]

print("=== GLOBAL_SURFACE ===")
print("openapi_operations", len(public_ids))
print("coverage_inventory", len(inv))
print("public_status", dict(sorted(collections.Counter(inv[op]["status"] for op in public_ids).items())))
print("all_status", dict(sorted(collections.Counter(value["status"] for value in inv.values()).items())))
reason_counts = collections.Counter()
reason_ops = collections.defaultdict(list)
for op in sorted(public_ids):
    item = inv[op]
    if item["status"] == "mapped":
        continue
    for reason in item.get("review_reasons", []):
        reason_counts[reason] += 1
        reason_ops[reason].append(op)
print("public_review_reason_counts", dict(reason_counts.most_common()))
for reason, ops in sorted(reason_ops.items(), key=lambda kv: (-len(kv[1]), kv[0])):
    print("REASON", reason, len(ops), ",".join(ops))
print("autoproject", report["added_count"], report["rejected_count"])
print("autoproject_reason_counts", dict(collections.Counter(report["rejected"].values()).most_common()))
for reason, ops in sorted(collections.defaultdict(list, {
    reason: sorted(op for op, value in report["rejected"].items() if value == reason)
    for reason in set(report["rejected"].values())
}).items(), key=lambda kv: (-len(kv[1]), kv[0])):
    print("AUTO_REASON", reason, len(ops), ",".join(ops))
modules, probe_rejected = probe.probes(openapi, raw, expanded)
print("probe_compileable", len(modules), ",".join(sorted(modules)))
print("probe_rejected", len(probe_rejected))
print("probe_reason_counts", dict(collections.Counter(probe_rejected.values()).most_common()))
for reason in sorted(set(probe_rejected.values())):
    ops = sorted(op for op, value in probe_rejected.items() if value == reason)
    print("PROBE_REASON", reason, len(ops), ",".join(ops))

print("\n=== REQUEST_MODEL_DETAILS ===")
for operation_id, reason in sorted(report["rejected"].items()):
    if reason != "request_model_projection":
        continue
    root = openapi.request_schema(operation_id)
    entry = coverage_by_id[operation_id]
    raw_op = raw.operation(entry["rust_method"])
    print("\n===", operation_id, "===")
    print("root:", root)
    print("raw_parameters:", [(p.name, p.type) for p in raw_op.parameters])
    if root is None:
        print("root_schema: <inline-or-none>")
        continue
    schema = openapi.schema(root)
    print("root_schema:", json.dumps(compact_schema(schema), sort_keys=True))
    print("root_raw:", raw_shape(raw, root))
    if root not in raw.structs:
        continue
    raw_fields = {field.name.removeprefix("r#"): field for field in raw.fields(root)}
    for field_name, field_schema in sorted(schema.get("properties", {}).items()):
        field = raw_fields.get(field_name)
        if field is None:
            print("field", field_name, "RAW_MISSING", json.dumps(compact_schema(field_schema), sort_keys=True))
            continue
        models = copy.deepcopy(manifest.get("models", {}))
        adapter, field_reason = auto._request_field_adapter(
            models,
            openapi.schemas,
            field_schema,
            field.type,
            raw,
            (root,),
            root,
            (field_name,),
        )
        if field_reason:
            print("FAILED_FIELD", field_name, "raw=", field.type, "reason=", field_reason)
            print("  schema=", json.dumps(compact_schema(field_schema), sort_keys=True))
            syntax = auto._strip_options(auto.parse_type(field.type))
            print("  raw_target=", raw_shape(raw, syntax.spelling))
        elif adapter:
            print("ADAPTED_FIELD", field_name, "raw=", field.type, "adapter=", adapter)
