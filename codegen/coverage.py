"""Fail generation on missing operations or runtime stubs; emit an API inventory."""

import json
from pathlib import Path
import re
import sys
from ruamel.yaml import YAML

METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}


def operations(spec):
    return [(path, method, op) for path, item in spec["paths"].items()
            for method, op in item.items() if method in METHODS]


def main(source, prepared, directory):
    original = YAML(typ="safe", pure=True).load(Path(source).read_text())
    spec = json.loads(Path(prepared).read_text())
    client = (Path(directory) / "client.rs").read_text()
    methods = re.findall(r"pub async fn (\w+)\s*\(", client)
    if len(methods) != len(operations(spec)):
        raise ValueError(f"Expected {len(operations(spec))} generated methods, found {len(methods)}")
    if re.search(r'HttpError::Config\s*\(\s*"', client) or "unimplemented!" in client or "todo!" in client:
        raise ValueError("Generated client contains a runtime configuration stub")
    inventory = []
    upstream = {op["operationId"] for _, _, op in operations(original)}
    for path, method, op in operations(spec):
        # Error enum docs retain exact upstream IDs, independent of Rust casing.
        # Untyped-error endpoints still carry their IDs in the generated method.
        snake = re.sub(r"_+", "_", op["operationId"].replace("-", "_")).lower()
        if snake not in methods:
            raise ValueError(f"Missing generated operation: {op['operationId']} ({snake})")
        inventory.append({"operation_id": op["operationId"], "method": method.upper(),
                          "path": path.split("#")[0], "rust_method": snake,
                          "upstream": op["operationId"] in upstream,
                          "tags": op.get("tags", []),
                          "success_media": sorted({media for status, response in op.get("responses", {}).items()
                              if str(status).startswith("2") for media in response.get("content", {})})})
    (Path(directory) / "coverage.json").write_text(json.dumps({
        "upstream_operations": len(upstream), "generated_methods": len(methods),
        "operations": sorted(inventory, key=lambda op: op["operation_id"])
    }, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main(*sys.argv[1:])
