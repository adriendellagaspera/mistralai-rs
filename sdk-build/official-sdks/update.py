"""Regenerate the compact public SDK evidence from pinned official SDKs."""

from __future__ import annotations

import argparse
import difflib
import json
from pathlib import Path

import harvest
import audit

HERE = Path(__file__).resolve().parent
LOCK = HERE.parent / "provenance.lock.json"
SURFACE = HERE / "surface.json"


def surface_from_inventory(inventory: dict) -> dict:
    operations = {}
    for operation in inventory["operations"]:
        ts = sorted({entry["normalized_public_path"] for entry in operation["typescript"]})
        py = sorted({entry["normalized_public_path"] for entry in operation["python"]})
        paths = ts or py
        if paths:
            operations[operation["operation_id"]] = paths
    return {"schema_version": 1, "client": "Mistral", "operations": operations}


def generate() -> dict:
    """Retain the public surface-only API used by existing callers."""
    return surface_from_inventory(harvest.generate(harvest.load_json(LOCK)))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["update", "check", "pin-latest"])
    parser.add_argument("--source", choices=("python", "typescript"))
    args = parser.parse_args()
    if args.command == "pin-latest":
        harvest.pin_latest(harvest.load_json(LOCK), source=args.source)
        return
    inventory = harvest.generate(harvest.load_json(LOCK))
    actual = surface_from_inventory(inventory)
    rendered = json.dumps(actual, indent=2) + "\n"
    report = audit.build_report(inventory)
    report_json = json.dumps(report, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
    report_md = audit.render_markdown(report)

    if args.command == "update":
        SURFACE.write_text(rendered)
        audit.REPORT_JSON.write_text(report_json)
        audit.REPORT_MD.write_text(report_md)
        print("Updated pinned official SDK surface and Tier-2/Tier-3 parity evidence")
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
    if audit.REPORT_JSON.read_text() != report_json:
        raise SystemExit("Pinned SDK parity-audit.json differs from official SDK revisions")
    if audit.REPORT_MD.read_text() != report_md:
        raise SystemExit("Pinned SDK PARITY_AUDIT.md differs from official SDK revisions")
    print("Official SDK surface and Tier-2/Tier-3 parity evidence match pinned revisions")


if __name__ == "__main__":
    main()
