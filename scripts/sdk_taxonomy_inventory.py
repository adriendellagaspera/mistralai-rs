#!/usr/bin/env python3
"""Build the compact, versioned taxonomy IR from official Mistral SDKs."""

from __future__ import annotations

import argparse
import difflib
import hashlib
import json
from pathlib import Path
from typing import Any

import sdk_taxonomy

ROOT = Path(__file__).resolve().parents[1]
LOCK_PATH = ROOT / "codegen.lock"
OUTPUT_PATH = ROOT / "codegen/sdk-taxonomy.json"


def normalized_paths(records: list[dict[str, Any]]) -> list[str]:
    return sorted({record["normalized_public_path"] for record in records})


def canonical_digest(value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return "sha256:" + hashlib.sha256(payload).hexdigest()


def compact_inventory(harvested: dict[str, Any]) -> dict[str, Any]:
    operations: dict[str, list[str]] = {}
    alias_differences: list[dict[str, Any]] = []
    divergences: list[dict[str, Any]] = []

    for operation in harvested["operations"]:
        typescript = normalized_paths(operation["typescript"])
        python = normalized_paths(operation["python"])
        primary = typescript or python
        if primary:
            operations[operation["operation_id"]] = primary
        if not typescript or not python or typescript == python:
            continue
        common = sorted(set(typescript) & set(python))
        difference = {
            "operation_id": operation["operation_id"],
            "typescript": typescript,
            "python": python,
        }
        if common:
            alias_differences.append({
                **difference,
                "common": common,
                "reason": "source_specific_aliases",
            })
        else:
            divergences.append({
                **difference,
                "reason": "public_path_conflict",
            })

    sdk_only: dict[tuple[str, str, str], dict[str, Any]] = {}
    for language in ("typescript", "python"):
        for record in harvested["sdk_only"][language]:
            key = (
                record["operation_id"],
                record["http_method"],
                record["http_path"],
            )
            item = sdk_only.setdefault(
                key,
                {
                    "operation_id": record["operation_id"],
                    "http_method": record["http_method"],
                    "http_path": record["http_path"],
                    "typescript": [],
                    "python": [],
                },
            )
            item[language].append(record["normalized_public_path"])

    sdk_only_operations = []
    for item in sdk_only.values():
        item["typescript"] = sorted(set(item["typescript"]))
        item["python"] = sorted(set(item["python"]))
        sdk_only_operations.append(item)
    sdk_only_operations.sort(
        key=lambda item: (item["http_path"], item["http_method"], item["operation_id"])
    )

    summary = dict(harvested["summary"])
    summary["sdk_only_operations"] = len(sdk_only_operations)
    summary["cross_sdk_alias_differences"] = len(alias_differences)
    summary["cross_sdk_divergences"] = len(divergences)

    return {
        "schema_version": 2,
        "provenance": harvested["provenance"],
        "summary": summary,
        "operations": operations,
        "openapi_only": harvested["openapi_only"],
        "sdk_only_digest": canonical_digest(sdk_only_operations),
        "non_http_resources": harvested["non_http_resources"],
        "cross_sdk_alias_differences": alias_differences,
        "cross_sdk_divergences": divergences,
    }


def generate() -> dict[str, Any]:
    lock = sdk_taxonomy.load_json(LOCK_PATH)
    return compact_inventory(sdk_taxonomy.generate(lock))


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def check(expected: dict[str, Any], actual: dict[str, Any]) -> None:
    expected_text = json.dumps(expected, indent=2, sort_keys=True) + "\n"
    actual_text = json.dumps(actual, indent=2, sort_keys=True) + "\n"
    if expected_text == actual_text:
        print("Official SDK taxonomy IR matches pinned sources byte-for-byte.")
        return
    print("".join(difflib.unified_diff(
        expected_text.splitlines(keepends=True),
        actual_text.splitlines(keepends=True),
        fromfile="committed/sdk-taxonomy.json",
        tofile="regenerated/sdk-taxonomy.json",
    )))
    raise SystemExit("Official SDK taxonomy IR is stale")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["update", "check"])
    args = parser.parse_args()
    actual = generate()
    if args.command == "update":
        write_json(OUTPUT_PATH, actual)
        print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)} from pinned official SDKs.")
    else:
        check(sdk_taxonomy.load_json(OUTPUT_PATH), actual)


if __name__ == "__main__":
    main()
