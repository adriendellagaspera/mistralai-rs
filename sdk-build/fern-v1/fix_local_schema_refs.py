#!/usr/bin/env python3
"""Normalize nested JSON Schema $defs references for Fern's OpenAPI importer.

The published OpenAPI document embeds JSON Schema fragments under media-type
schemas. A fragment-local "#/$defs/Name" reference is relative to that
fragment, but Fern's importer resolves it against the OpenAPI document root.
Rewrite only those fragment-local references to absolute JSON Pointers and
fail if any local reference remains unresolved.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


def escape_pointer(value: str) -> str:
    return value.replace("~", "~0").replace("/", "~1")


def resolve_pointer(document: Any, pointer: str) -> Any:
    if pointer == "#":
        return document
    if not pointer.startswith("#/"):
        raise ValueError(f"Unsupported local JSON Pointer: {pointer}")
    current = document
    for raw in pointer[2:].split("/"):
        token = raw.replace("~1", "/").replace("~0", "~")
        if isinstance(current, list):
            current = current[int(token)]
        else:
            current = current[token]
    return current


def rewrite_nested_defs(document: Any) -> int:
    rewritten = 0

    def visit(node: Any, pointer: str, defs_scope: str | None) -> None:
        nonlocal rewritten
        if isinstance(node, dict):
            scope = f"{pointer}/$defs" if isinstance(node.get("$defs"), dict) else defs_scope
            ref = node.get("$ref")
            if isinstance(ref, str) and ref.startswith("#/$defs/") and scope:
                suffix = ref[len("#/$defs/"):]
                node["$ref"] = f"#{scope}/{suffix}"
                rewritten += 1
            for key, value in list(node.items()):
                visit(value, f"{pointer}/{escape_pointer(key)}", scope)
        elif isinstance(node, list):
            for index, value in enumerate(node):
                visit(value, f"{pointer}/{index}", defs_scope)

    visit(document, "", None)
    return rewritten


def validate_local_refs(document: Any) -> None:
    failures: list[str] = []

    def visit(node: Any, pointer: str) -> None:
        if isinstance(node, dict):
            ref = node.get("$ref")
            if isinstance(ref, str) and ref.startswith("#"):
                try:
                    resolve_pointer(document, ref)
                except (KeyError, IndexError, ValueError, TypeError) as exc:
                    failures.append(f"{pointer}: {ref} ({exc})")
            for key, value in node.items():
                visit(value, f"{pointer}/{escape_pointer(key)}")
        elif isinstance(node, list):
            for index, value in enumerate(node):
                visit(value, f"{pointer}/{index}")

    visit(document, "#")
    if failures:
        raise ValueError("Unresolved local OpenAPI references:\\n" + "\\n".join(failures))


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: fix_local_schema_refs.py OPENAPI_JSON")
    path = Path(sys.argv[1])
    document = json.loads(path.read_text())
    rewritten = rewrite_nested_defs(document)
    validate_local_refs(document)
    path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\\n")
    print(f"nested JSON Schema references rewritten: {rewritten}; all local references resolve")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
