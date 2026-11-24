#!/usr/bin/env python3
"""Hoist response-local JSON Schema definitions into OpenAPI components.

Fern's importer does not reliably import nested JSON Schema $defs. Hoisting
them to components.schemas gives each definition a stable, valid OpenAPI name.
The summary endpoint's three NDJSON frame shapes are disjoint, so its anyOf
is normalized to the semantically precise oneOf.
"""
from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Any


def pointer_escape(value: str) -> str:
    return value.replace("~", "~0").replace("/", "~1")


def pointer_resolve(document: Any, pointer: str) -> Any:
    if pointer == "#":
        return document
    if not pointer.startswith("#/"):
        raise ValueError(f"Unsupported local JSON Pointer: {pointer}")
    current = document
    for raw in pointer[2:].split("/"):
        token = raw.replace("~1", "/").replace("~0", "~")
        current = current[int(token)] if isinstance(current, list) else current[token]
    return current


def hoist_nested_defs(document: dict[str, Any]) -> int:
    schemas = document.setdefault("components", {}).setdefault("schemas", {})
    hoisted = 0

    def process(node: Any, pointer: str, scope: dict[str, str] | None) -> Any:
        nonlocal hoisted
        if isinstance(node, list):
            return [
                process(value, f"{pointer}/{index}", scope)
                for index, value in enumerate(node)
            ]
        if not isinstance(node, dict):
            return node

        local_defs = node.get("$defs")
        local_scope = scope
        if isinstance(local_defs, dict):
            digest = hashlib.sha256(pointer.encode()).hexdigest()[:8].upper()
            local_scope = {}
            for name in local_defs:
                suffix = "".join(
                    part[:1].upper() + part[1:]
                    for part in re.findall(r"[A-Za-z0-9]+", name)
                )
                component_name = f"FernLocal{digest}{suffix or 'Definition'}"
                if component_name in schemas:
                    raise ValueError(f"Hoisted schema name collision: {component_name}")
                local_scope[name] = component_name
            node.pop("$defs")
            for name, definition in local_defs.items():
                component_name = local_scope[name]
                rewritten = process(
                    definition,
                    f"{pointer}/$defs/{pointer_escape(name)}",
                    local_scope,
                )
                schemas[component_name] = rewritten
                hoisted += 1

        ref = node.get("$ref")
        if isinstance(ref, str) and ref.startswith("#/$defs/"):
            if not local_scope:
                raise ValueError(f"No enclosing $defs scope for {pointer}: {ref}")
            suffix = ref[len("#/$defs/"):]
            name, separator, rest = suffix.partition("/")
            if name not in local_scope:
                raise ValueError(f"Unknown nested $defs reference at {pointer}: {ref}")
            node["$ref"] = (
                f"#/components/schemas/{local_scope[name]}"
                + (f"/{rest}" if separator else "")
            )

        for key in list(node):
            if key != "$ref":
                node[key] = process(
                    node[key], f"{pointer}/{pointer_escape(key)}", local_scope
                )
        return node

    # Paths/responses are processed first so their local definitions are
    # registered before component schemas are traversed.
    for key in list(document):
        if key != "components":
            document[key] = process(
                document[key], f"#/{pointer_escape(key)}", None
            )

    components = document["components"]
    for component_key in list(components):
        if component_key != "schemas":
            components[component_key] = process(
                components[component_key],
                f"#/components/{pointer_escape(component_key)}",
                None,
            )
    for name in list(schemas):
        schemas[name] = process(
            schemas[name], f"#/components/schemas/{pointer_escape(name)}", None
        )
    return hoisted


def normalize_summary_stream_union(document: dict[str, Any]) -> bool:
    path = "/v1/context/search-indexes/{index_name}/summary_field/{language}"
    operation = document.get("paths", {}).get(path, {}).get("post", {})
    schema = (
        operation.get("responses", {})
        .get("200", {})
        .get("content", {})
        .get("application/x-ndjson", {})
        .get("schema", {})
    )
    branches = schema.get("anyOf")
    if not isinstance(branches, list) or len(branches) != 3:
        raise ValueError("Unexpected summary NDJSON response union")
    required_sets = []
    for branch in branches:
        ref = branch.get("$ref") if isinstance(branch, dict) else None
        if not isinstance(ref, str) or not ref.startswith("#/components/schemas/"):
            raise ValueError("Unexpected summary NDJSON frame reference")
        definition = pointer_resolve(document, ref)
        required = definition.get("required", []) if isinstance(definition, dict) else []
        if not isinstance(required, list) or not required:
            raise ValueError(f"Summary NDJSON frame has no required fields: {ref}")
        required_sets.append(set(required))
    if any(
        required_sets[i] & required_sets[j]
        for i in range(3)
        for j in range(i + 1, 3)
    ):
        raise ValueError("Summary NDJSON frame shapes are no longer disjoint")
    schema["oneOf"] = schema.pop("anyOf")
    return True


def validate_local_refs(document: Any) -> None:
    failures = []

    def visit(node: Any, pointer: str) -> None:
        if isinstance(node, dict):
            ref = node.get("$ref")
            if isinstance(ref, str) and ref.startswith("#"):
                try:
                    pointer_resolve(document, ref)
                except (KeyError, IndexError, ValueError, TypeError) as exc:
                    failures.append(f"{pointer}: {ref} ({exc})")
            for key, value in node.items():
                visit(value, f"{pointer}/{pointer_escape(key)}")
        elif isinstance(node, list):
            for index, value in enumerate(node):
                visit(value, f"{pointer}/{index}")

    visit(document, "#")
    if failures:
        raise ValueError(
            "Unresolved local OpenAPI references:\n" + "\n".join(failures)
        )


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: fix_local_schema_refs.py OPENAPI_JSON")
    path = Path(sys.argv[1])
    document = json.loads(path.read_text())
    count = hoist_nested_defs(document)
    normalized = normalize_summary_stream_union(document)
    validate_local_refs(document)
    path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    print(
        f"nested JSON Schema definitions hoisted: {count}; "
        f"summary NDJSON union normalized: {normalized}; "
        "all local references resolve"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
