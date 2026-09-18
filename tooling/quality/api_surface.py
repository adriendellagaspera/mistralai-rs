"""Fail closed on changed/removed generated public declarations versus a Git base.

The first facade baseline is established by PR review. This conservative check
does not replace a compiler-aware semver audit of raw types and trait behavior.
"""

import argparse
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "pipeline" / "mistral"))

REVIEW_PATH = ROOT / "tooling" / "quality" / "api-review.json"


def review_matches(base, report, review):
    if review.get("base") != base:
        return False
    return all(
        review.get(key) == report.get(key)
        for key in ("removed", "changed", "added")
    )


def main():
    from sdk_contracts import compare_surface

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", required=True)
    parser.add_argument(
        "--rustdoc",
        action="store_true",
        help="also run pinned compiler-aware semver checks",
    )
    args = parser.parse_args()
    base = subprocess.check_output(
        ["git", "rev-parse", "--verify", args.base + "^{commit}"],
        cwd=ROOT,
        text=True,
    ).strip()
    path = "src/sdk/api-surface.json"
    names = subprocess.check_output(
        ["git", "ls-tree", "--name-only", base, "--", path],
        cwd=ROOT,
        text=True,
    )
    if not names.strip():
        print("No previous facade API inventory: initial baseline requires PR review.")
        return
    before = json.loads(
        subprocess.check_output(["git", "show", f"{base}:{path}"], cwd=ROOT)
    )
    after = json.loads((ROOT / path).read_text())
    report = compare_surface(before, after)
    print(json.dumps(report, indent=2))
    if report["classification"] == "review_required":
        review = json.loads(REVIEW_PATH.read_text()) if REVIEW_PATH.exists() else {}
        if not review_matches(base, report, review):
            raise SystemExit(
                "Generated public API changed: explicit compatibility review required."
            )
        print(f"Accepted reviewed public API change from {REVIEW_PATH.relative_to(ROOT)}.")
    if args.rustdoc:
        version = json.loads((ROOT / "tooling/sources/lock.json").read_text())[
            "cargo_semver_checks_version"
        ]
        install = ROOT / ".tools" / f"cargo-semver-checks-{version}"
        executable = install / "bin/cargo-semver-checks"
        if not executable.exists():
            subprocess.run(
                [
                    "cargo",
                    "install",
                    "cargo-semver-checks",
                    "--version",
                    version,
                    "--locked",
                    "--root",
                    str(install),
                ],
                cwd=ROOT,
                check=True,
            )
        reported = subprocess.check_output([str(executable), "--version"], text=True).strip()
        if reported != f"cargo-semver-checks {version}":
            raise SystemExit(f"unexpected semver tool version: {reported}")
        subprocess.run(
            [str(executable), "semver-checks", "--baseline-rev", base],
            cwd=ROOT,
            check=True,
        )


if __name__ == "__main__":
    main()
