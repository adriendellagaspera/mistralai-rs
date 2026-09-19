"""Review committed API inventory changes and run pinned compiler-aware semver checks."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
REVIEW = ROOT / "sdk-build" / "api-review.json"
SURFACE = ROOT / "src" / "sdk" / "api-surface.json"


def command(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", required=True)
    parser.add_argument("--rustdoc", action="store_true")
    args = parser.parse_args()
    base = command("git", "rev-parse", "--verify", args.base + "^{commit}")
    before_text = command("git", "show", f"{base}:src/sdk/api-surface.json")
    before, after = json.loads(before_text), json.loads(SURFACE.read_text())
    removed = sorted(before.keys() - after.keys())
    changed = sorted(name for name in before.keys() & after.keys() if before[name] != after[name])
    added = sorted(after.keys() - before.keys())
    report = {"removed": removed, "changed": changed, "added": added}
    print(json.dumps(report, indent=2))
    if removed or changed:
        review = json.loads(REVIEW.read_text()) if REVIEW.exists() else {}
        if review.get("base") != base or any(review.get(key) != report[key] for key in report):
            raise SystemExit("Public API change requires explicit review")
    if not args.rustdoc:
        return

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


if __name__ == "__main__":
    main()
