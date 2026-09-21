#!/usr/bin/env python3
"""Print bounded, reproducible structural evidence for selected 288 derivation gaps."""
import json
import sys
from pathlib import Path

root = Path("sdk-build/target")
spec = json.loads((root / "candidate-overlaid.json").read_text())
bindings = json.loads((root / "candidate-bindings.json").read_text())
report = json.loads((root / "candidate-derivation-report.json").read_text())

by_id = {}
for path, item in spec["paths"].items():
    for verb, operation in item.items():
        if verb.lower() in {"get", "post", "put", "patch", "delete", "options", "head", "trace"}:
            if isinstance(operation, dict) and "operationId" in operation:
                by_id[operation["operationId"]] = (verb.upper(), path, operation)

def preview(value, size=6500):
    dump = json.dumps(value, sort_keys=True, ensure_ascii=False)
    return dump if len(dump) <= size else dump[:size] + "... [TRUNCATED]"

for id in sys.argv[1:]:
    result = report["operations"].get(id)
    print(f"\\n## {id}")
    if result is None:
        print("UNKNOWN SOURCE OPERATION")
        continue
    print("OUTCOME", preview(result, 2000))
    method, path, operation = by_id[id]
    print("ROUTE", method, path)
    print("REQUEST", preview(operation.get("requestBody"), 7500))
    print("PARAMETERS", preview(operation.get("parameters", []), 3800))
    print("RESPONSES", preview(operation.get("responses", {}), 3500))
    matching = {
        name: binding for name, binding in bindings["operations"].items()
        if binding.get("metadata", {}).get("source_operation", {}).get("operation_id") == id
    }
    for name, binding in matching.items():
        print("BINDING", name, preview(binding, 6500))
        params = binding.get("parameters", [])
        for param in params:
            for typ in [param.get("type", ""), param.get("type_name", "")]:
                typ = typ.replace("Option<", "").replace("Vec<", "").replace(">", "").strip("&")
                if typ in bindings.get("structs", {}):
                    print("STRUCT", typ, preview(bindings["structs"][typ], 5500))
    refs = set()
    def walk(node):
        if isinstance(node, dict):
            if isinstance(node.get("$ref"), str):
                refs.add(node["$ref"])
            for child in node.values():
                walk(child)
        elif isinstance(node, list):
            for child in node:
                walk(child)
    walk(operation.get("requestBody", {}))
    walk(operation.get("responses", {}))
    for ref in sorted(refs):
        name = ref.removeprefix("#/components/schemas/")
        if name in spec.get("components", {}).get("schemas", {}):
            print("SCHEMA", name, preview(spec["components"]["schemas"][name], 4500))
