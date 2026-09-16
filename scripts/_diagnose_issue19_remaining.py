import copy
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))

import sdk_autoproject as auto
import sdk_compiler as compiler


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
_, report = auto.expand_manifest(openapi, manifest, taxonomy, raw_coverage, raw)
coverage_by_id = {entry["operation_id"]: entry for entry in raw_coverage["operations"]}

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
