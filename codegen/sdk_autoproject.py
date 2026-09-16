"""Derive conservative semantic facade projections from official SDK taxonomy.

The official SDK taxonomy owns public resource/method names. OpenAPI owns the
wire contract. This module only auto-projects shapes the generic Rust backend
already knows how to expose without leaking raw generated request field types.
Everything else remains explicit review debt in the projection report.
"""

from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import re
from typing import Any

from rust_sdk_compiler.rust_types import RustType, parse_type


RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
    "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "typeof", "unsized", "virtual", "yield", "try",
}


def _pascal(value: str) -> str:
    return "".join(part[:1].upper() + part[1:] for part in re.split(r"[^A-Za-z0-9]+", value) if part)


def _resource_key(path: tuple[str, ...]) -> str:
    return "_".join(path)


def _resource_name(path: tuple[str, ...]) -> str:
    return "".join(_pascal(part) for part in path)


def _operation_id(entry: dict[str, Any]) -> str | None:
    value = entry.get("operation_id")
    return value if isinstance(value, str) and value else None


def _raw_method(entry: dict[str, Any]) -> str | None:
    value = entry.get("raw_method")
    return value if isinstance(value, str) and value else None


def _operation_shape(openapi, operation_id: str) -> tuple[str | None, str | None]:
    return openapi.request_schema(operation_id), openapi.response_schema(operation_id)


def _configured_operation_ids(manifest: dict[str, Any]) -> set[str]:
    configured: set[str] = set()
    for resource in manifest.get("resources", {}).values():
        if not isinstance(resource, dict):
            continue
        for operation in resource.get("operations", {}).values():
            if not isinstance(operation, dict):
                continue
            operation_id = _operation_id(operation)
            if operation_id:
                configured.add(operation_id)
    return configured


def _operation_taxonomy(taxonomy: dict[str, Any]) -> dict[str, dict[str, Any]]:
    operations = taxonomy.get("operations", {})
    if isinstance(operations, dict):
        return {key: value for key, value in operations.items() if isinstance(value, dict)}
    return {}


def _coverage_operation(raw_coverage: dict[str, Any], operation_id: str) -> dict[str, Any]:
    operations = raw_coverage.get("operations", {})
    if not isinstance(operations, dict):
        return {}
    value = operations.get(operation_id, {})
    return value if isinstance(value, dict) else {}


def _raw_method_for_operation(
    openapi,
    taxonomy_entry: dict[str, Any],
    raw_coverage: dict[str, Any],
    operation_id: str,
) -> str | None:
    raw_method = _raw_method(taxonomy_entry)
    if raw_method:
        return raw_method
    coverage = _coverage_operation(raw_coverage, operation_id)
    raw_method = coverage.get("rust_method")
    if isinstance(raw_method, str) and raw_method:
        return raw_method
    operation = openapi.operations.get(operation_id, {})
    candidate = operation.get("x-sdk-rust-method")
    return candidate if isinstance(candidate, str) and candidate else None


def _resource_path(entry: dict[str, Any]) -> tuple[str, ...] | None:
    value = entry.get("resource")
    if isinstance(value, str) and value:
        return tuple(part for part in value.split(".") if part)
    if isinstance(value, list) and value and all(isinstance(part, str) and part for part in value):
        return tuple(value)
    return None


def _operation_name(operation_id: str, entry: dict[str, Any]) -> str:
    name = entry.get("name")
    if isinstance(name, str) and name:
        return name
    return operation_id


def _rust_identifier(value: str) -> str:
    return f"r#{value}" if value in RUST_KEYWORDS else value


def _raw_field_types(bindings, raw_name: str) -> dict[str, RustType]:
    if raw_name not in bindings.structs:
        return {}
    return {field.name: field.syntax for field in bindings.fields(raw_name)}


def _request_model(
    openapi,
    bindings,
    request_raw: str,
    public_name: str,
) -> dict[str, Any] | None:
    schema = openapi.schema(request_raw)
    if schema.get("type") != "object":
        return None
    fields = _raw_field_types(bindings, request_raw)
    required = schema.get("required", [])
    if not isinstance(required, list):
        return None
    constructor = []
    for field in required:
        if not isinstance(field, str) or field not in fields:
            return None
        constructor.append(_rust_identifier(field))
    return {
        "raw": request_raw,
        "constructor": constructor,
    }


def _response_model(openapi, bindings, response_raw: str) -> dict[str, Any] | None:
    schema = openapi.schema(response_raw)
    if schema.get("type") == "object" and response_raw in bindings.structs:
        return {"raw": response_raw, "borrowed": False, "accessors": {}}
    if schema.get("type") == "string" and response_raw in bindings.enums:
        return {"raw": response_raw, "scalar_enum": {"root": response_raw, "path": []}}
    return None


def _projection_for_operation(openapi, bindings, operation_id: str) -> tuple[dict[str, Any], dict[str, Any]] | None:
    request_raw, response_raw = _operation_shape(openapi, operation_id)
    operation: dict[str, Any] = {"operation_id": operation_id}
    models: dict[str, Any] = {}

    if request_raw:
        public_request = f"{_pascal(operation_id)}Request"
        request = _request_model(openapi, bindings, request_raw, public_request)
        if request is None:
            return None
        models[public_request] = request
        operation["request"] = public_request

    if response_raw:
        public_response = f"{_pascal(operation_id)}Response"
        response = _response_model(openapi, bindings, response_raw)
        if response is None:
            return None
        models[public_response] = response
        operation["response"] = public_response
    else:
        wire_operation = openapi.operation(operation_id)
        successes = [
            response
            for status, response in wire_operation.get("responses", {}).items()
            if str(status).startswith("2")
        ]
        if len(successes) == 1 and not successes[0].get("content"):
            operation["empty_response"] = True
        else:
            return None
    return models, operation


def expand_manifest(
    openapi,
    manifest: dict[str, Any],
    taxonomy: dict[str, Any],
    raw_coverage: dict[str, Any],
    bindings,
) -> tuple[dict[str, Any], dict[str, Any]]:
    """Conservatively add directly projectable official-SDK operations."""
    expanded = deepcopy(manifest)
    configured = _configured_operation_ids(expanded)
    projected: list[str] = []
    rejected: dict[str, str] = {}

    for operation_id, entry in sorted(_operation_taxonomy(taxonomy).items()):
        if operation_id in configured or operation_id not in openapi.operations:
            continue
        raw_method = _raw_method_for_operation(openapi, entry, raw_coverage, operation_id)
        if raw_method is None or raw_method not in bindings.operations:
            rejected[operation_id] = "raw operation unavailable"
            continue
        projection = _projection_for_operation(openapi, bindings, operation_id)
        if projection is None:
            rejected[operation_id] = "shape requires explicit semantic review"
            continue
        path = _resource_path(entry)
        if not path:
            rejected[operation_id] = "official resource path unavailable"
            continue
        models, operation = projection
        operation["raw_method"] = raw_method
        operation_name = _operation_name(operation_id, entry)
        resources = expanded.setdefault("resources", {})
        key = _resource_key(path)
        resource = resources.setdefault(
            key,
            {
                "path": list(path),
                "name": _resource_name(path),
                "operations": {},
            },
        )
        resource.setdefault("operations", {})[operation_name] = operation
        expanded.setdefault("models", {}).update(models)
        projected.append(operation_id)

    digest = hashlib.sha256(
        json.dumps(expanded, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()
    report = {
        "schema_version": 1,
        "projected": projected,
        "rejected": rejected,
        "expanded_manifest_sha256": digest,
    }
    return expanded, report
