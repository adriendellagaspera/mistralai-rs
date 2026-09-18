"""Fail generation on missing operations or runtime stubs; emit an API inventory."""

import json
from pathlib import Path
import re
import sys

METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}

def manifest_methods(manifest):
    """Return the exact public async method inventory owned by the generator manifest."""
    operations = manifest.get("operations")
    if not isinstance(operations, list) or not operations:
        raise ValueError("Binding manifest has no operations")
    methods = []
    for index, operation in enumerate(operations):
        if not isinstance(operation, dict):
            raise ValueError(f"Binding manifest operation {index} is not an object")
        method = operation.get("rust_method_name")
        if not isinstance(method, str) or not method:
            raise ValueError(
                f"Binding manifest operation {index} has no rust_method_name"
            )
        methods.append(method)
    if len(methods) != len(set(methods)):
        raise ValueError("Binding manifest contains duplicate Rust method names")
    return set(methods)


def manifest_binary_streams(manifest):
    """Map canonical source-operation identity to its generated live binary method."""
    streams = {}
    for index, operation in enumerate(manifest.get("operations", [])):
        if not isinstance(operation, dict):
            raise ValueError(f"Binding manifest operation {index} is not an object")
        if operation.get("kind") != "call_shape":
            continue
        representation = operation.get("representation")
        if not isinstance(representation, dict):
            raise ValueError(
                f"Binding manifest operation {index} has no representation identity"
            )
        if representation.get("kind") != "binary_stream":
            continue
        source = operation.get("source_operation")
        if not isinstance(source, dict):
            raise ValueError(
                f"Binding manifest operation {index} has no source-operation identity"
            )
        try:
            key = (
                source["method"].upper(),
                source["path"],
                source["operation_id"],
            )
            method = operation["rust_method_name"]
        except KeyError as error:
            raise ValueError(
                f"Binding manifest binary stream operation {index} is incomplete"
            ) from error
        if key in streams:
            raise ValueError(
                f"Binding manifest has multiple binary streams for source operation {key}"
            )
        streams[key] = method
    return streams


def operations(spec):
    return [(path, method, op) for path, item in spec["paths"].items()
            for method, op in item.items() if method in METHODS]


def rust_method(operation_id):
    return re.sub(r"_+", "_", operation_id.replace("-", "_")).lower()


def inventory(original, spec, client, expected_methods=None, binary_streams=None):
    methods = re.findall(r"pub async fn (\w+)\s*\(", client)
    if len(methods) != len(set(methods)):
        raise ValueError("Generated client contains duplicate async method names")
    if expected_methods is None:
        expected = {rust_method(op["operationId"]) for _, _, op in operations(spec)}
    else:
        expected = set(expected_methods)
    binary_streams = {} if binary_streams is None else dict(binary_streams)
    actual = set(methods)
    if actual != expected:
        missing = sorted(expected - actual)
        extra = sorted(actual - expected)
        raise ValueError(f"Generated method drift: missing={missing}, extra={extra}")
    if re.search(r'HttpError::Config\s*\(\s*"', client) or "unimplemented!" in client or "todo!" in client:
        raise ValueError("Generated client contains a runtime configuration stub")
    inventory = []
    upstream = {op["operationId"] for _, _, op in operations(original)}
    for path, method, op in operations(spec):
        # Error enum docs retain exact upstream IDs, independent of Rust casing.
        # Untyped-error endpoints still carry their IDs in the generated method.
        snake = rust_method(op["operationId"])
        if snake not in methods:
            raise ValueError(f"Missing generated operation: {op['operationId']} ({snake})")
        entry = {"operation_id": op["operationId"], "method": method.upper(),
                 "path": path.split("#")[0], "rust_method": snake,
                 "upstream": op["operationId"] in upstream,
                 "tags": op.get("tags", []),
                 "success_media": sorted({media for status, response in op.get("responses", {}).items()
                     if str(status).startswith("2") for media in response.get("content", {})})}
        stream_key = (method.upper(), path, op["operationId"])
        if stream_method := binary_streams.get(stream_key):
            entry["binary_stream_method"] = stream_method
        inventory.append(entry)
    return {
        "upstream_operations": len(upstream), "generated_methods": len(methods),
        "operations": sorted(inventory, key=lambda op: op["operation_id"])
    }


def main(source, prepared, directory):
    from ruamel.yaml import YAML
    original = YAML(typ="safe", pure=True).load(Path(source).read_text())
    spec = json.loads(Path(prepared).read_text())
    generated = Path(directory)
    client = (generated / "client.rs").read_text()
    manifest = json.loads((generated / "binding-manifest.json").read_text())
    report = inventory(
        original,
        spec,
        client,
        manifest_methods(manifest),
        manifest_binary_streams(manifest),
    )
    (generated / "coverage.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main(*sys.argv[1:])
