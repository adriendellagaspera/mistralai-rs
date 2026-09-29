#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import tempfile
from pathlib import Path

import yaml

from compat import apply_input_compat
from gates import tree_digest, validate, write_inventory
from policy import apply_product_policy
from workarounds import apply_output_workarounds

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
CANONICAL = ROOT / "sdk-build/openapi/published.yaml"
POLICY = HERE / "product-policy.json"
FERN_DIR = HERE / "fern"
PROJECTED = FERN_DIR / "openapi.yaml"
GENERATED = HERE / "generated"
INVENTORY = HERE / "public-inventory.json"
ROOT_SRC = ROOT / "src"


def project_input(destination: Path) -> list[str]:
    doc = yaml.safe_load(CANONICAL.read_text())
    apply_product_policy(doc, POLICY)
    issues = apply_input_compat(doc)
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(yaml.safe_dump(doc, sort_keys=False))
    return issues


def fern_generate() -> None:
    subprocess.run(
        ["fern", "generate", "--group", "rust-sdk", "--local", "--no-prompt", "--force", "--log-level", "info"],
        cwd=HERE,
        check=True,
    )


def generate_once() -> tuple[list[str], list[str]]:
    input_issues = project_input(PROJECTED)
    shutil.rmtree(GENERATED, ignore_errors=True)
    fern_generate()
    output_issues = apply_output_workarounds(GENERATED)
    return input_issues, output_issues


def validate_fresh(input_issues: list[str], output_issues: list[str], *, inventory_must_match: bool) -> dict:
    if not inventory_must_match and INVENTORY.exists():
        INVENTORY.unlink()
    return validate(CANONICAL, PROJECTED, GENERATED, INVENTORY, input_issues, output_issues)


def cmd_generate() -> None:
    input_issues, output_issues = generate_once()
    inventory = validate_fresh(input_issues, output_issues, inventory_must_match=False)
    write_inventory(INVENTORY, inventory)
    if ROOT_SRC.exists():
        shutil.rmtree(ROOT_SRC)
    shutil.copytree(GENERATED / "src", ROOT_SRC)
    print(json.dumps({
        "operation_count": inventory["canonical_operation_count"],
        "generated_accounted": inventory["generated_accounted_operation_count"],
        "workarounds": inventory["workarounds"],
    }, indent=2))


def cmd_check() -> None:
    input_issues, output_issues = generate_once()
    first = tree_digest(GENERATED)
    inventory = validate_fresh(input_issues, output_issues, inventory_must_match=True)

    input_issues_2, output_issues_2 = generate_once()
    second = tree_digest(GENERATED)
    if first != second:
        raise SystemExit(f"Fern generation is not deterministic: {first} != {second}")
    validate_fresh(input_issues_2, output_issues_2, inventory_must_match=True)

    with tempfile.TemporaryDirectory() as tmp:
        committed = Path(tmp) / "src"
        shutil.copytree(ROOT_SRC, committed)
        if tree_digest(committed) != tree_digest(GENERATED / "src"):
            raise SystemExit("Committed src/ differs from deterministic Fern output; run just generate")

    print(json.dumps({
        "operation_count": inventory["canonical_operation_count"],
        "generated_accounted": inventory["generated_accounted_operation_count"],
        "deterministic_digest": first,
        "public_method_count": len(inventory["public_methods"]),
        "public_type_count": len(inventory["public_types"]),
        "workarounds": inventory["workarounds"],
    }, indent=2))


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["generate", "check"])
    args = parser.parse_args()
    {"generate": cmd_generate, "check": cmd_check}[args.command]()


if __name__ == "__main__":
    main()
