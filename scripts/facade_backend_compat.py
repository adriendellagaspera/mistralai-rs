#!/usr/bin/env python3
"""Check whether a newer openapi-to-rust revision preserves facade semantics."""

from __future__ import annotations

import argparse
import difflib
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from typing import Any


def run(*args: object, cwd: Path | None = None) -> None:
    subprocess.run([str(arg) for arg in args], cwd=cwd, check=True)


def output(*args: object, cwd: Path | None = None) -> str:
    return subprocess.check_output(
        [str(arg) for arg in args], cwd=cwd, text=True
    ).strip()


def snapshot(root: Path) -> dict[str, bytes]:
    return {
        path.relative_to(root).as_posix(): path.read_bytes()
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def changed_files(before: dict[str, bytes], after: dict[str, bytes]) -> list[str]:
    return sorted(
        name for name in before.keys() | after.keys()
        if before.get(name) != after.get(name)
    )


def changed_line_counts(before: bytes, after: bytes) -> tuple[int, int]:
    added = removed = 0
    for line in difflib.ndiff(
        before.decode(errors="replace").splitlines(),
        after.decode(errors="replace").splitlines(),
    ):
        if line.startswith("+ "):
            added += 1
        elif line.startswith("- "):
            removed += 1
    return added, removed


def generator_version(binary: Path) -> str:
    actual = output(binary, "--version")
    match = re.fullmatch(r"openapi-to-rust\s+(.+)", actual)
    if not match:
        raise RuntimeError(f"unexpected generator version output: {actual!r}")
    return match.group(1)


def prepared_spec(source: Path, destination: Path) -> None:
    document = json.loads(source.read_text())
    document.setdefault("info", {"title": "facade compatibility fixture", "version": "1"})
    destination.write_text(json.dumps(document, indent=2) + "\n")


def generate_fixture(binary: Path, fixture: Path, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    spec = destination.parent / "openapi.json"
    prepared_spec(fixture / "openapi.json", spec)
    config = destination.parent / "openapi-to-rust.toml"
    config.write_text(
        "\n".join(
            [
                "[generator]",
                f'spec_path = "{spec.as_posix()}"',
                f'output_dir = "{destination.as_posix()}"',
                'module_name = "fixture"',
                "",
                "[features]",
                "enable_async_client = true",
                "",
                "[http_client]",
                'base_url = "https://example.invalid"',
                "",
                "[http_client.retry]",
                "max_retries = 0",
                "",
            ]
        )
    )
    run(binary, "generate", "--config", config)


def load_facade(package_root: Path):
    source = package_root / "src"
    sys.path.insert(0, str(source))
    try:
        import openapi_to_rust_facade as facade  # type: ignore
    finally:
        sys.path.pop(0)
    return facade


def compile_fixture(facade: Any, fixture: Path, raw: Path) -> dict[str, str]:
    document = json.loads((fixture / "openapi.json").read_text())
    policy = json.loads((fixture / "policy.json").read_text())
    raw_ir = facade.OpenApiToRustAdapter.load(raw)
    _, files = facade.compile_facade(facade.OpenApiIndex(document), raw_ir, policy)
    return files


def sidecar_files(snapshot_: dict[str, bytes]) -> list[str]:
    return sorted(
        name for name in snapshot_
        if "raw-ir" in name.lower() or "sidecar" in name.lower()
    )


def update_compatibility(
    package_root: Path, candidate_version: str, candidate_commit: str
) -> None:
    compatibility_path = package_root / "COMPATIBILITY.json"
    compatibility = json.loads(compatibility_path.read_text())
    compatibility["backend"]["version"] = candidate_version
    compatibility["backend"]["commit"] = candidate_commit
    compatibility_path.write_text(json.dumps(compatibility, indent=2) + "\n")

    readme_path = package_root / "README.md"
    readme = readme_path.read_text()
    updated = re.sub(
        r"(is validated against `openapi-to-rust` `)[^`]+"
        r"(`, repository `gpu-cli/openapi-to-rust`, commit `)[0-9a-f]+(`)",
        rf"\g<1>{candidate_version}\g<2>{candidate_commit}\g<3>",
        readme,
        count=1,
    )
    if updated == readme:
        raise RuntimeError("could not update README backend compatibility line")
    readme_path.write_text(updated)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--facade-root", type=Path, required=True)
    parser.add_argument("--baseline-generator", type=Path, required=True)
    parser.add_argument("--candidate-generator", type=Path, required=True)
    parser.add_argument("--candidate-commit", required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--update-compatibility", action="store_true")
    args = parser.parse_args()

    package_root = args.facade_root.resolve()
    compatibility = json.loads((package_root / "COMPATIBILITY.json").read_text())
    baseline_commit = compatibility["backend"]["commit"]
    baseline_version = generator_version(args.baseline_generator)
    candidate_version = generator_version(args.candidate_generator)
    expected_baseline = compatibility["backend"]["version"]
    if baseline_version != expected_baseline:
        raise RuntimeError(
            f"baseline generator version mismatch: {baseline_version} != {expected_baseline}"
        )

    facade = load_facade(package_root)
    fixture_root = package_root / "tests" / "fixtures"
    fixture_names = sorted(
        path.name for path in fixture_root.iterdir()
        if path.is_dir() and (path / "openapi.json").exists() and (path / "policy.json").exists()
    )
    if not fixture_names:
        raise RuntimeError("no facade compatibility fixtures found")

    rows: list[dict[str, Any]] = []
    incompatible: list[str] = []
    with tempfile.TemporaryDirectory(prefix="facade-backend-") as temporary:
        work = Path(temporary)
        for name in fixture_names:
            fixture = fixture_root / name
            baseline_raw = work / name / "baseline" / "raw"
            candidate_raw = work / name / "candidate" / "raw"

            generate_fixture(args.baseline_generator, fixture, baseline_raw)
            generate_fixture(args.candidate_generator, fixture, candidate_raw)

            baseline_snapshot = snapshot(baseline_raw)
            candidate_snapshot = snapshot(candidate_raw)
            raw_changed = changed_files(baseline_snapshot, candidate_snapshot)
            raw_added = raw_removed = 0
            for filename in raw_changed:
                added, removed = changed_line_counts(
                    baseline_snapshot.get(filename, b""),
                    candidate_snapshot.get(filename, b""),
                )
                raw_added += added
                raw_removed += removed

            baseline_files = compile_fixture(facade, fixture, baseline_raw)
            candidate_files = compile_fixture(facade, fixture, candidate_raw)
            facade_changed = sorted(
                filename
                for filename in baseline_files.keys() | candidate_files.keys()
                if baseline_files.get(filename) != candidate_files.get(filename)
            )
            if facade_changed:
                incompatible.append(name)

            rows.append(
                {
                    "fixture": name,
                    "raw_changed": raw_changed,
                    "raw_added_lines": raw_added,
                    "raw_removed_lines": raw_removed,
                    "baseline_sidecars": sidecar_files(baseline_snapshot),
                    "candidate_sidecars": sidecar_files(candidate_snapshot),
                    "facade_changed": facade_changed,
                }
            )

    lines = [
        "# openapi-to-rust facade backend compatibility",
        "",
        f"- Baseline: `{baseline_version}` / `{baseline_commit}`",
        f"- Candidate: `{candidate_version}` / `{args.candidate_commit}`",
        f"- Facade package: `{compatibility['package_version']}`",
        "",
        "| Fixture | Raw diff | Sidecar change | Facade diff |",
        "| --- | --- | --- | --- |",
    ]
    for row in rows:
        raw = (
            "none"
            if not row["raw_changed"]
            else f"{len(row['raw_changed'])} files (+{row['raw_added_lines']}/-{row['raw_removed_lines']})"
        )
        sidecars = (
            "none"
            if row["baseline_sidecars"] == row["candidate_sidecars"]
            else f"{row['baseline_sidecars']} -> {row['candidate_sidecars']}"
        )
        facade_diff = (
            "none"
            if not row["facade_changed"]
            else ", ".join(f"`{name}`" for name in row["facade_changed"])
        )
        lines.append(f"| {row['fixture']} | {raw} | {sidecars} | {facade_diff} |")
        if row["raw_changed"]:
            lines.append("")
            lines.append(
                f"Raw files changed for **{row['fixture']}**: "
                + ", ".join(f"`{name}`" for name in row["raw_changed"])
            )

    lines.extend(
        [
            "",
            (
                "**Result: compatible.** Generated facade source maps are byte-for-byte identical."
                if not incompatible
                else "**Result: incompatible.** Facade semantics changed for: "
                + ", ".join(incompatible)
            ),
            "",
        ]
    )
    args.report.write_text("\n".join(lines))

    if incompatible:
        raise SystemExit(
            "candidate backend changes facade output: " + ", ".join(incompatible)
        )
    if args.update_compatibility:
        update_compatibility(package_root, candidate_version, args.candidate_commit)


if __name__ == "__main__":
    main()
