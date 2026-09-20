"""Review changes to actual Rust API sources and run pinned compiler-aware semver checks.

Historical JSON snapshots are not a substitute for inspecting the Rust that ships:
all generated facade files are independently checked byte-for-byte by the Rust SDK builder.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
REVIEW = Path(__file__).resolve().parent / "review.json"
PUBLIC_RUST_PATHS = (
    "src/lib.rs",
    "src/streaming.rs",
    "src/sdk/",
    "src/generated/",
)


def command(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def changed_public_rust_files(base: str) -> list[str]:
    """Compare the checked-out Rust files, never a separately committed inventory."""
    changed = command(
        "git", "diff", "--no-ext-diff", "--no-renames", "--name-only",
        base, "--", *PUBLIC_RUST_PATHS,
    )
    return sorted(path for path in changed.splitlines() if path.endswith(".rs"))


def require_api_review(base: str, changed_files: list[str], review: dict) -> None:
    if not changed_files:
        return
    if (
        review.get("schema_version") != 1
        or review.get("base") != base
        or review.get("changed_rust_files") != changed_files
    ):
        raise SystemExit(
            "Public Rust sources changed: explicit API review must record the "
            "exact PR base and changed Rust files in sdk-build/api-review/review.json"
        )


def check_rustdoc_semver(base: str) -> None:
    lock = json.loads((ROOT / "sdk-build" / "provenance.lock.json").read_text())
    version = lock["cargo_semver_checks_version"]
    executable = ROOT / ".tools" / f"cargo-semver-checks-{version}" / "bin" / "cargo-semver-checks"
    if not executable.is_file():
        subprocess.run(
            ["cargo", "install", "cargo-semver-checks", "--version", version,
             "--locked", "--root", str(executable.parents[1])],
            cwd=ROOT,
            check=True,
        )
    actual_version = subprocess.check_output(
        [str(executable), "--version"], text=True
    ).strip()
    if actual_version != f"cargo-semver-checks {version}":
        raise ValueError(f"Unexpected semver tool version: {actual_version}")
    subprocess.run(
        [str(executable), "semver-checks", "--baseline-rev", base],
        cwd=ROOT,
        check=True,
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", required=True)
    parser.add_argument("--rustdoc", action="store_true")
    args = parser.parse_args()
    base = command("git", "rev-parse", "--verify", args.base + "^{commit}")
    changed_files = changed_public_rust_files(base)
    print(json.dumps({"changed_rust_files": changed_files}, indent=2))
    review = json.loads(REVIEW.read_text()) if REVIEW.exists() else {}
    require_api_review(base, changed_files, review)
    if args.rustdoc:
        check_rustdoc_semver(base)


if __name__ == "__main__":
    main()
