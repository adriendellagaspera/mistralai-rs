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

from rust_types import RustType, parse_type


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
    if "$ref" in schema or "enum" in schema or "const" in schema:
        return False
    kind = schema.get("type")
    if kind in {"string", "integer", "number", "boolean"}:
        return True
    if kind == "array":
        return _simple_schema(schema.get("items", {}))
    if kind == "object" and not schema.get("properties"):
        additional = schema.get("additionalProperties")
        return additional is True or (isinstance(additional, dict) and _simple_schema(additional))
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


def _binary_media(operation: dict[str, Any]) -> dict[str, dict[str, Any]]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return {}
    result = {}
    for media, payload in success[0].get("content", {}).items():
        schema = payload.get("schema", {})
        if schema.get("type") == "string" and schema.get("format") == "binary":
            result[media] = schema
    return result


def _success_contract(operation: dict[str, Any], preferred_transport: str | None = None) -> tuple[str | None, dict[str, Any] | None, str | None]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None, None, "multiple_success_contracts"
    content = success[0].get("content", {})
    if not content:
        return "empty", None, None
    if preferred_transport == "binary_stream":
        binary = _binary_media(operation)
        if len(binary) == 1:
            return "binary", next(iter(binary.values())), None
        return None, None, "official_binary_transport_mismatch"
    if "application/json" in content:
        if set(content) != {"application/json"}:
            return None, None, "multiple_success_media"
        return "json", content["application/json"].get("schema", {}), None
    if len(content) == 1:
        binary = _binary_media(operation)
        if len(binary) == 1:
            return "binary", next(iter(binary.values())), None
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


def _alias_name(raw: str) -> str:
    return f"{raw}Value"


def _union_name(raw: str) -> str:
    return f"{raw}Value"


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


def _strip_options(syntax: RustType) -> RustType:
    while (inner := syntax.unary("Option")) is not None:
        syntax = inner
    return syntax


def _raw_public_leaf(syntax: RustType, rust: Any) -> bool:
    if syntax.spelling in rust.aliases or syntax.spelling in rust.symbol_modules:
        return False
    if syntax.kind == "generic_type":
        return all(_raw_public_leaf(argument, rust) for argument in syntax.arguments)
    return True


def _safe_alias(syntax: RustType, rust: Any, seen: tuple[str, ...] = ()) -> bool:
    if syntax.spelling in rust.aliases:
        if syntax.spelling in seen:
            return False
        return _safe_alias(rust.aliases[syntax.spelling], rust, (*seen, syntax.spelling))
    if syntax.spelling in rust.symbol_modules:
        return False
    if syntax.kind == "generic_type":
        return all(_safe_alias(argument, rust, seen) for argument in syntax.arguments)
    return True


def _ensure_alias_model(models: dict[str, Any], raw: str, rust: Any) -> tuple[str | None, str | None]:
    if raw not in rust.aliases or not _safe_alias(rust.aliases[raw], rust, (raw,)):
        return None, "request_model_projection"
    existing = _existing_model_by_raw(models, raw, False)
    if existing:
        return existing, None
    name = _alias_name(raw)
    if name in models and models[name].get("raw", name) != raw:
        return None, "request_model_projection"
    models[name] = {"raw": raw, "type_alias": True}
    return name, None


def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:
    schema, _ = _nullable(schema)
    reference = _schema_ref(schema)
    target = schemas.get(reference, {}) if reference else schema
    values = target.get("enum")
    singleton = (isinstance(values, list) and len(values) == 1) or "const" in target
    if not singleton:
        return False
    syntax = _strip_options(parse_type(raw_type))
    return syntax.spelling in rust.enums and len(rust.enums[syntax.spelling]) == 1


def _ensure_union_model(models: dict[str, Any], schemas: dict[str, Any], schema: dict[str, Any],
                        raw: str, rust: Any, resolving: tuple[str, ...]) -> tuple[str | None, str | None]:
    existing = _existing_model_by_raw(models, raw, False)
    if existing:
        return existing, None
    branches = schema.get("oneOf", []) or schema.get("anyOf", [])
    non_null = [branch for branch in branches if branch.get("type") != "null"]
    references = [_schema_ref(branch) for branch in non_null]
    if len(non_null) < 2 or any(reference is None for reference in references) or raw not in rust.enums:
        return None, "request_model_projection"
    variants = rust.enums[raw]
    if any(variant.payload is None for variant in variants):
        return None, "request_model_projection"
    by_payload = {variant.payload: variant.name for variant in variants}
    if len(by_payload) != len(variants) or set(by_payload) != set(references):
        return None, "request_model_projection"

    discriminator = schema.get("discriminator", {})
    mapping = discriminator.get("mapping", {}) if isinstance(discriminator, dict) else {}
    tags_by_ref = {
        value.rsplit("/", 1)[-1]: tag
        for tag, value in mapping.items()
        if isinstance(value, str)
    }
    if mapping and set(tags_by_ref) != set(references):
        return None, "request_model_projection"

    facade_variants: dict[str, Any] = {}
    seen_public: set[str] = set()
    for reference in references:
        adapter, reason = _ensure_request_model(models, schemas, reference, rust, resolving)
        if reason or adapter is None:
            return None, reason or "request_model_projection"
        public = _pascal(tags_by_ref.get(reference, reference).lower() if reference in tags_by_ref else reference)
        if not public or public in seen_public:
            return None, "request_model_projection"
        seen_public.add(public)
        facade_variants[by_payload[reference]] = {"name": public, "adapter": adapter}

    name = _union_name(raw)
    if name in models and models[name].get("raw", name) != raw:
        return None, "request_model_projection"
    models[name] = {"raw": raw, "simple_union": {"variants": facade_variants}}
    return name, None


def _request_field_adapter(models: dict[str, Any], schemas: dict[str, Any], schema: dict[str, Any],
                           raw_type: str, rust: Any, resolving: tuple[str, ...]) -> tuple[str | None, str | None]:
    schema, _ = _nullable(schema)
    syntax = _strip_options(parse_type(raw_type))
    if schema.get("type") == "array":
        inner = syntax.unary("Vec")
        if inner is None:
            return None, "request_model_projection"
        return _request_field_adapter(models, schemas, schema.get("items", {}), inner.spelling, rust, resolving)

    if schema.get("oneOf") or schema.get("anyOf"):
        return _ensure_union_model(models, schemas, schema, syntax.spelling, rust, resolving)

    reference = _schema_ref(schema)
    if reference:
        target = schemas.get(reference, {})
        if target.get("type") == "object" and target.get("properties"):
            if syntax.spelling != reference:
                return None, "request_model_projection"
            return _ensure_request_model(models, schemas, reference, rust, resolving)
        if target.get("type") == "object" and target.get("additionalProperties"):
            if syntax.spelling in rust.aliases:
                return _ensure_alias_model(models, syntax.spelling, rust)
            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")
        if _simple_schema(target):
            if syntax.spelling in rust.aliases:
                return _ensure_alias_model(models, syntax.spelling, rust)
            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")
        return None, "request_model_projection"

    if schema.get("type") == "object" and schema.get("additionalProperties"):
        if syntax.spelling in rust.aliases:
            return _ensure_alias_model(models, syntax.spelling, rust)
        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")
    if _simple_schema(schema):
        if syntax.spelling in rust.aliases:
            return _ensure_alias_model(models, syntax.spelling, rust)
        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")
    return None, "request_model_projection"


def _ensure_request_model(models: dict[str, Any], schemas: dict[str, Any], raw: str,
                          rust: Any | None = None, resolving: tuple[str, ...] = ()) -> tuple[str | None, str | None]:
    existing = _existing_model_by_raw(models, raw, True)
    if existing:
        return existing, None
    schema = schemas.get(raw)
    if not schema or schema.get("type") != "object" or raw in resolving:
        return None, "request_model_projection"
    properties = schema.get("properties", {})
    if rust is None:
        if not all(_simple_schema(value) for value in properties.values()):
            return None, "request_model_projection"
        name = _request_name(raw)
        models[name] = {"raw": raw, "constructor": list(schema.get("required", []))}
        return name, None
    if raw not in rust.structs:
        return None, "request_model_projection"
    raw_fields = {field.name.removeprefix("r#"): field for field in rust.fields(raw)}
    if set(raw_fields) != set(properties):
        return None, "request_model_projection"
    adapters: dict[str, str] = {}
    excluded: list[str] = []
    required = set(schema.get("required", []))
    for field, field_schema in sorted(properties.items()):
        if field not in required and _omittable_singleton_enum(
            field_schema, raw_fields[field].type, schemas, rust
        ):
            excluded.append(field)
            continue
        adapter, reason = _request_field_adapter(
            models, schemas, field_schema, raw_fields[field].type, rust, (*resolving, raw)
        )
        if reason:
            return None, reason
        if adapter:
            adapters[field] = adapter
    name = _request_name(raw)
    if name in models and models[name].get("raw", name) != raw:
        return None, "request_model_projection"
    config: dict[str, Any] = {"raw": raw, "constructor": list(schema.get("required", []))}
    if adapters:
        config["adapters"] = adapters
    if excluded:
        config["exclude"] = excluded
    models[name] = config
    return name, None


def expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],
                    raw_coverage: dict[str, Any], rust: Any | None = None) -> tuple[dict[str, Any], dict[str, Any]]:
    """Return overlay + deterministic report for automatically projected operations."""
    expanded = deepcopy(manifest)
    models: dict[str, Any] = expanded.setdefault("models", {})
    resources: dict[str, Any] = expanded.setdefault("resources", {})
    raw_operations = raw_coverage.get("operations", [])
    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_operations if item.get("upstream", True)}
    raw_by_id = {item["operation_id"]: item for item in raw_operations}
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

        response_transport = taxonomy.get("operation_transports", {}).get(operation_id)
        response_kind, response_schema, reason = _success_contract(operation, response_transport)
        if reason:
            rejected[operation_id] = reason
            continue
        if response_kind == "binary":
            binary_media = set(_binary_media(operation))
            success = [response for status, response in operation.get("responses", {}).items()
                    if str(status).startswith("2")]
            content = success[0].get("content", {}) if len(success) == 1 else {}
            if len(content) == 1:
                stream_method = raw_by_id.get(operation_id, {}).get("binary_stream_method")
                if not stream_method:
                    rejected[operation_id] = "raw_binary_transport_mapping"
                    continue
                raw_method = stream_method
            else:
                route_path = operation.get("x-sdk-path", "").split("#", 1)[0].rstrip("/") or "/"
                route_method = operation.get("x-sdk-method", "").upper()
                candidates = [
                    item for item in raw_operations
                    if item.get("method") == route_method
                    and (item.get("path", "").split("#", 1)[0].rstrip("/") or "/") == route_path
                    and item.get("success_media")
                    and item.get("binary_stream_method")
                    and set(item["success_media"]).issubset(binary_media)
                ]
                if len(candidates) == 1:
                    raw_method = candidates[0]["binary_stream_method"]
                else:
                    rejected[operation_id] = "raw_binary_transport_mapping"
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
            model_snapshot = set(models)
            request, reason = _ensure_request_model(models, openapi.schemas, request_raw, rust)
            if reason:
                for model_name in set(models) - model_snapshot:
                    del models[model_name]
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