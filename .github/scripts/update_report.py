"""Render a reviewable source-update PR body, separate from exhaustive nightly logs."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess

import nightly

ROOT = Path(__file__).resolve().parents[2]
LOCK = "sdk-build/provenance.lock.json"
SURFACE = "sdk-build/official-sdks/surface.json"
COVERAGE = "src/generated/coverage.json"

SOURCES = (
    ("OpenAPI", ("openapi",), "openapi"),
    ("Python SDK", ("official_sdks", "python"), "official_python"),
    ("TypeScript SDK", ("official_sdks", "typescript"), "official_typescript"),
)


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def from_head(path: str) -> dict:
    return json.loads(git("show", f"HEAD:{path}"))


def current(path: str) -> dict:
    return json.loads((ROOT / path).read_text())


def changed_files() -> set[str]:
    # The workflow stages candidate paths before rendering; diagnostic artifacts
    # remain untracked and must not be considered part of the source-update PR.
    changed = set(git("diff", "--cached", "--name-only").splitlines())
    unexpected = sorted(
        path for path in changed
        if not path.startswith(("sdk-build/", "src/generated/", "src/sdk/"))
    )
    if unexpected:
        raise ValueError(f"Unexpected source-update changes: {unexpected}")
    return changed


def pin(lock: dict, path: tuple[str, ...]) -> dict:
    value = lock
    for key in path:
        value = value[key]
    return value


def pin_link(source: dict) -> str:
    revision = source["commit"]
    repository = source["repository"]
    return f"[\x60{revision[:12]}\x60](https://github.com/{repository}/commit/{revision})"


def short_diagnostic(detail: str, limit: int = 240) -> str:
    """Use the root-cause line, not the full subprocess traceback or shell command."""
    lines = [line.strip() for line in detail.splitlines() if line.strip()]
    cause = lines[-1] if lines else "See the nightly report."
    if len(cause) > limit:
        cause = cause[: limit - 1].rstrip() + "…"
    return cause.replace("|", "\\|").replace("\x60", "'")


def render(
    before_lock: dict,
    after_lock: dict,
    before_surface: dict,
    after_surface: dict,
    before_coverage: dict,
    after_coverage: dict,
    files: set[str],
    rows: list[dict],
    healthy: bool,
    run_url: str,
) -> str:
    counts = {state: sum(row["status"] == state for row in rows) for state in nightly.STATES}
    changed_pins = []
    for title, key, _check in SOURCES:
        before = pin(before_lock, key)
        after = pin(after_lock, key)
        if before["commit"] != after["commit"]:
            changed_pins.append(f"- **{title}:** {pin_link(before)} → {pin_link(after)}")
    if not changed_pins:
        changed_pins = ["- No source revision changed; inspect the generated/input diff."]

    changed_raw = sorted(path for path in files if path.startswith("src/generated/"))
    changed_facade = sorted(path for path in files if path.startswith("src/sdk/"))
    before_operations = before_surface["operations"]
    after_operations = after_surface["operations"]
    before_ids, after_ids = set(before_operations), set(after_operations)
    remapped = sum(before_operations[key] != after_operations[key] for key in before_ids & after_ids)

    lines = [
        "## Summary",
        "",
        "Regenerate the Rust SDK from pinned Mistral sources. This PR contains "
        "generated output and source-tracking changes; it does not update the SDK runtime by hand.",
        "",
        "### Source revisions",
        "",
        *changed_pins,
        "",
        "### Generated impact",
        "",
        f"- Raw generated files changed: **{len(changed_raw)}**; facade files changed: **{len(changed_facade)}**.",
        f"- Generated methods: **{before_coverage['generated_methods']} → {after_coverage['generated_methods']}**.",
        f"- Official SDK operation mappings: **{len(before_ids)} → {len(after_ids)}** "
        f"({len(after_ids - before_ids)} added, {len(before_ids - after_ids)} removed, {remapped} modified).",
    ]
    if COVERAGE in files:
        lines.append("- The generated coverage inventory changed; review its diff alongside the Rust API.")
    lines.extend([
        "",
        "### Nightly checks",
        "",
        f"**{'PASS' if healthy else 'FAIL'}** — {counts['PASS']} passed, "
        f"{counts['FAIL']} failed, {counts['BLOCKED']} blocked, "
        f"{counts['NOT_APPLICABLE']} not applicable. "
        f"[Full run and diagnostics]({run_url}).",
        "",
    ])
    problems = [row for row in rows if row["status"] in {"FAIL", "BLOCKED"}]
    if problems:
        lines.append("Checks requiring attention:")
        for row in problems[:6]:
            lines.append(
                f"- **{row['job']} / {row['check']} — {row['status']}:** "
                f"{short_diagnostic(str(row.get('detail', '')))}"
            )
        if len(problems) > 6:
            lines.append(f"- …and {len(problems) - 6} more; see the full run.")
        lines.append("")
    lines.extend([
        "The nightly result describes source acquisition and the generated candidate; "
        "the PR's own CI and API review run separately against its current base.",
        "",
        "### Review",
        "",
        "- [ ] Review source pins, generated Rust/API changes and coverage.",
        "- [ ] Check the PR's CI and explicit API compatibility review against its current base.",
        "- [ ] Investigate any failed or blocked nightly checks before merging.",
        "",
    ])
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-url", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    rows, healthy = nightly.combined(changed=True)
    body = render(
        from_head(LOCK), current(LOCK),
        from_head(SURFACE), current(SURFACE),
        from_head(COVERAGE), current(COVERAGE),
        changed_files(), rows, healthy, args.run_url,
    )
    args.output.write_text(body)
    print(body)


if __name__ == "__main__":
    main()
