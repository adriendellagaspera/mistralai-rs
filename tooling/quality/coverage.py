"""Fail generation on missing bindings or runtime stubs; emit an API inventory."""

import json
from pathlib import Path
import re
import sys

METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}
MANIFEST_SCHEMA = "openapi-to-rust.binding-manifest"
MANIFEST_VERSION = 1


def operations(spec):
    return [
        (path, method, op)
        for path, item in spec["paths"].items()
        for method, op in item.items()
        if method in METHODS
    ]


def source_identity(path, method, operation):
    return operation["operationId"], method.upper(), path


def inventory(original, spec, client, manifest):
    if manifest.get("schema") != MANIFEST_SCHEMA:
        raise ValueError("Unexpected binding manifest schema")
    if manifest.get("schema_version") != MANIFEST_VERSION:
        raise ValueError("Unexpected binding manifest version")

    methods = re.findall(r"pub async fn (\w+)\s*\(", client)
    if len(methods) != len(set(methods)):
        raise ValueError("Generated client contains duplicate async method names")

    manifest_operations = manifest.get("operations")
    if not isinstance(manifest_operations, list):
        raise ValueError("Binding manifest operations must be a list")
    manifest_methods = [operation["rust_method_name"] for operation in manifest_operations]
    if len(manifest_methods) != len(set(manifest_methods)):
        raise ValueError("Binding manifest contains duplicate Rust method names")
    if set(methods) != set(manifest_methods):
        missing = sorted(set(manifest_methods) - set(methods))
        extra = sorted(set(methods) - set(manifest_methods))
        raise ValueError(f"Generated method drift: missing={missing}, extra={extra}")

    if (
        re.search(r'HttpError::Config\s*\(\s*"', client)
        or "unimplemented!" in client
        or "todo!" in client
    ):
        raise ValueError("Generated client contains a runtime configuration stub")

    source_operations = {
        source_identity(path, method, operation): operation
        for path, method, operation in operations(spec)
    }
    call_shape_sources = {
        (
            operation["source_operation"]["operation_id"],
            operation["source_operation"]["method"],
            operation["source_operation"]["path"],
        )
        for operation in manifest_operations
        if operation["kind"] == "call_shape"
    }
    expected_sources = set(source_operations)
    if call_shape_sources != expected_sources:
        missing = sorted(expected_sources - call_shape_sources)
        extra = sorted(call_shape_sources - expected_sources)
        raise ValueError(
            f"Binding manifest source-operation drift: missing={missing}, extra={extra}"
        )

    upstream = {operation["operationId"] for _, _, operation in operations(original)}
    rows = []
    for binding in manifest_operations:
        source = binding["source_operation"]
        identity = (
            source["operation_id"],
            source["method"],
            source["path"],
        )
        operation = source_operations.get(identity)
        if operation is None:
            raise ValueError(f"Binding manifest references unknown source operation: {identity}")
        representation = binding["representation"]
        rows.append(
            {
                "operation_id": source["operation_id"],
                "method": source["method"],
                "path": source["path"],
                "rust_method": binding["rust_method_name"],
                "operation_kind": binding["kind"],
                "representation": representation,
                "success_statuses": binding["success_statuses"],
                "upstream": source["operation_id"] in upstream,
                "tags": operation.get("tags", []),
                "success_media": sorted(
                    {
                        media
                        for status, response in operation.get("responses", {}).items()
                        if str(status).startswith("2")
                        for media in response.get("content", {})
                    }
                ),
            }
        )

    return {
        "upstream_operations": len(upstream),
        "source_operations": len(source_operations),
        "generated_methods": len(methods),
        "operations": sorted(
            rows,
            key=lambda operation: (
                operation["operation_id"],
                operation["rust_method"],
            ),
        ),
    }


def main(source, prepared, directory):
    from ruamel.yaml import YAML

    directory = Path(directory)
    original = YAML(typ="safe", pure=True).load(Path(source).read_text())
    spec = json.loads(Path(prepared).read_text())
    client = (directory / "client.rs").read_text()
    manifest = json.loads((directory / "binding-manifest.json").read_text())
    report = inventory(original, spec, client, manifest)
    (directory / "coverage.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n"
    )


if __name__ == "__main__":
    main(*sys.argv[1:])
