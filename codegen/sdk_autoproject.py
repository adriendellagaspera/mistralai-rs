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


def _nullable(schema: dict[str, Any]) -> tuple[dict[str, Any], bool]:
    branches = schema.get("anyOf", [])
    if not branches:
        return schema, False
    non_null = [item for item in branches if item.get("type") != "null"]
    if len(non_null) == 1 and len(non_null) != len(branches):
        return non_null[0], True
    return schema, False


def _simple_schema(schema: dict[str, Any]) -> bool:
    schema, _ = _nullable(schema)
    if "$ref" in schema or "enum" in schema or "const" in schema or "format" in schema:
        return False
    kind = schema.get("type")
    if kind in {"string", "integer", "number", "boolean"}:
        return True
    if kind == "array":
        return _simple_schema(schema.get("items", {}))
    return False


def _simple_accessor(schema: dict[str, Any], required: bool = True) -> str | None:
    schema, nullable = _nullable(schema)
    # Nullable OpenAPI fields are currently emitted by the raw generator as
    # Option<Option<T>> in several models. Do not guess away that distinction
    # in the semantic layer; a later transport/model primitive can normalize it
    # deliberately. The operation itself remains safely projectable.
    if nullable:
        return None
    optional = not required
    if "$ref" in schema or "enum" in schema or "const" in schema or "format" in schema:
        return None
    kind = schema.get("type")
    if kind == "string":
        return "optional_ref" if optional else "ref"
    if kind in {"integer", "number", "boolean"}:
        return "optional_copy" if optional else "copy"
    return None


def _schema_ref(schema: dict[str, Any]) -> str | None:
    ref = schema.get("$ref")
    return ref.rsplit("/", 1)[-1] if isinstance(ref, str) else None


def _success_contract(operation: dict[str, Any]) -> tuple[str | None, dict[str, Any] | None, str | None]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None, None, "multiple_success_contracts"
    content = success[0].get("content", {})
    if not content:
        return "empty", None, None
    if "application/json" in content:
        if set(content) != {"application/json"}:
            return None, None, "multiple_success_media"
        return "json", content["application/json"].get("schema", {}), None
    if len(content) == 1:
        payload = next(iter(content.values()))
        schema = payload.get("schema", {})
        if schema.get("type") == "string" and schema.get("format") == "binary":
            return "binary", schema, None
    return None, None, "non_json_success"


def _request_json_schema(operation: dict[str, Any]) -> tuple[dict[str, Any] | None, str | None]:
    content = operation.get("requestBody", {}).get("content", {})
    if not content:
        return None, None
    if set(content) != {"application/json"}:
        return None, "request_media_projection"
    schema = content["application/json"].get("schema", {})
    if not _schema_ref(schema):
        return None, "inline_or_unresolved_request"
    return schema, None


def _canonical_public_path(paths: list[str]) -> tuple[str | None, str | None]:
    if not paths:
        return None, "missing_official_taxonomy"
    # Prefer the most structured official path. Multiple equal-depth paths are
    # true aliases or transport conveniences and need an explicit semantic call.
    depth = max(path.count(".") for path in paths)
    candidates = sorted(path for path in paths if path.count(".") == depth)
    if len(candidates) != 1:
        return None, "taxonomy_alias_ambiguity"
    return candidates[0], None


def _view_name(raw: str) -> str:
    return f"{raw}View"


def _request_name(raw: str) -> str:
    return f"{raw}Params"


def _ensure_view_model(models: dict[str, Any], schemas: dict[str, Any], raw: str,
                       borrowed: bool = False) -> str:
    name = _view_name(raw)
    if name in models:
        return name
    schema = schemas.get(raw, {})
    accessors: dict[str, Any] = {}
    required_fields = set(schema.get("required", []))
    for field, field_schema in sorted(schema.get("properties", {}).items()):
        if field in RUST_KEYWORDS or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", field):
            continue
        kind = _simple_accessor(field_schema, field in required_fields)
        if kind:
            accessors[field] = {"kind": kind, "path": [field]}
    # Nested arrays/objects deliberately stay opaque in the first bulk pass.
    # Auto-generating borrowed child wrappers requires proving lifetime shape
    # against the raw Rust AST, which is a separate generic compiler primitive.
    models[name] = {"raw": raw, "borrowed": borrowed, "accessors": accessors}
    return name


def _existing_model_by_raw(models: dict[str, Any], raw: str, request: bool) -> str | None:
    for name, config in models.items():
        if config.get("raw", name) != raw:
            continue
        is_request = "constructor" in config or "union_factory" in config
        if is_request == request:
            return name
    return None


def _ensure_request_model(models: dict[str, Any], schemas: dict[str, Any], raw: str) -> tuple[str | None, str | None]:
    existing = _existing_model_by_raw(models, raw, True)
    if existing:
        return existing, None
    schema = schemas.get(raw)
    if not schema or schema.get("type") != "object":
        return None, "request_model_projection"
    properties = schema.get("properties", {})
    if not all(_simple_schema(value) for value in properties.values()):
        return None, "request_model_projection"
    name = _request_name(raw)
    models[name] = {"raw": raw, "constructor": list(schema.get("required", []))}
    return name, None


def expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],
                    raw_coverage: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:
    """Return overlay + deterministic report for automatically projected operations."""
    expanded = deepcopy(manifest)
    models: dict[str, Any] = expanded.setdefault("models", {})
    resources: dict[str, Any] = expanded.setdefault("resources", {})
    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_coverage.get("operations", []) if item.get("upstream", True)}
    mapped = {item["operation_id"] for resource in resources.values()
              for item in resource.get("operations", {}).values()}
    added: list[str] = []
    rejected: dict[str, str] = {}

    for operation_id, operation in sorted(openapi.operations.items()):
        if operation_id in mapped:
            continue
        paths = taxonomy.get("operations", {}).get(operation_id, [])
        public_path, reason = _canonical_public_path(paths)
        if reason:
            rejected[operation_id] = reason
            continue
        parts = tuple(public_path.split("."))
        if len(parts) < 2:
            rejected[operation_id] = "invalid_official_taxonomy"
            continue
        resource_path, public_name = parts[:-1], parts[-1]
        if any(not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", part) or part in RUST_KEYWORDS
               for part in (*resource_path, public_name)):
            rejected[operation_id] = "invalid_rust_public_name"
            continue
        raw_method = raw_methods.get(operation_id)
        if not raw_method:
            rejected[operation_id] = "raw_symbol_mapping"
            continue

        response_kind, response_schema, reason = _success_contract(operation)
        if reason:
            rejected[operation_id] = reason
            continue
        response = None
        if response_kind == "json":
            response_raw = _schema_ref(response_schema or {})
            if not response_raw:
                rejected[operation_id] = "inline_or_unresolved_response"
                continue
            existing_response = _existing_model_by_raw(models, response_raw, False)
            response = existing_response or _ensure_view_model(models, openapi.schemas, response_raw)

        request_schema, reason = _request_json_schema(operation)
        if reason:
            rejected[operation_id] = reason
            continue
        request = None
        if request_schema:
            request_raw = _schema_ref(request_schema)
            request, reason = _ensure_request_model(models, openapi.schemas, request_raw)
            if reason:
                rejected[operation_id] = reason
                continue

        # Ensure every prefix has a concrete resource node so the Rust backend
        # can emit client.beta().agents() rather than flattening public names.
        for depth in range(1, len(resource_path) + 1):
            path = resource_path[:depth]
            key = _resource_key(path)
            existing = resources.get(key)
            if existing is not None and tuple(existing.get("path", (key,))) != path:
                raise ValueError(f"resource module collision for {'.'.join(path)}")
            resource = resources.setdefault(key, {
                "name": _resource_name(path),
                "path": list(path),
                "operations": {},
            })
            resource.setdefault("path", list(path))
        resource = resources[_resource_key(resource_path)]
        if public_name in resource["operations"]:
            rejected[operation_id] = "public_method_collision"
            continue
        item: dict[str, Any] = {"operation_id": operation_id}
        if response is not None:
            item["response"] = response
        elif response_kind == "empty":
            item["empty_response"] = True
        elif response_kind == "binary":
            item["binary_response"] = True
        if raw_method != operation_id:
            item["raw_method"] = raw_method
        if request:
            item["request"] = request
        resource["operations"][public_name] = item
        mapped.add(operation_id)
        added.append(operation_id)

    report_payload = {"added": sorted(added), "rejected": dict(sorted(rejected.items()))}
    report = {
        "schema_version": 1,
        "added_count": len(added),
        "rejected_count": len(rejected),
        "report_sha256": hashlib.sha256(
            json.dumps(report_payload, sort_keys=True, separators=(",", ":")).encode()
        ).hexdigest(),
        **report_payload,
    }
    return expanded, report