"""Regenerate the compact public SDK evidence from pinned official SDKs."""

from __future__ import annotations

import argparse
import difflib
import json
from pathlib import Path

import harvest

HERE = Path(__file__).resolve().parent
LOCK = HERE.parent / "provenance.lock.json"
SURFACE = HERE / "surface.json"


def generate() -> dict:
    lock = harvest.load_json(LOCK)
    inventory = harvest.generate(lock)
    operations = {}
    for operation in inventory["operations"]:
        ts = sorted({entry["normalized_public_path"] for entry in operation["typescript"]})
        py = sorted({entry["normalized_public_path"] for entry in operation["python"]})
        paths = ts or py
        if paths:
            operations[operation["operation_id"]] = paths
    return {"schema_version": 1, "client": "Mistral", "operations": operations}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["update", "check", "pin-latest"])
    parser.add_argument("--source", choices=("python", "typescript"))
    args = parser.parse_args()
    if args.command == "pin-latest":
        harvest.pin_latest(harvest.load_json(LOCK), source=args.source)
        return
    actual = generate()
    rendered = json.dumps(actual, indent=2) + "\n"
    if args.command == "update":
        SURFACE.write_text(rendered)
        print("Updated pinned official SDK surface")
        return
    previous = SURFACE.read_text()
    if json.loads(previous) != actual:
        print("".join(difflib.unified_diff(
            previous.splitlines(keepends=True),
            rendered.splitlines(keepends=True),
            fromfile="committed/official-sdks/surface.json",
            tofile="regenerated/official-sdks/surface.json",
        )))
        raise SystemExit("Official SDK surface differs from pinned evidence")
    print("Official SDK surface matches pinned Python and TypeScript revisions")


if __name__ == "__main__":
    main()
