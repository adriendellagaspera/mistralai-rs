"""Summarize reviewed source and SDK changes after regeneration."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
LOCK = "sdk-build/provenance.lock.json"
SURFACE = "sdk-build/official-sdks/surface.json"
SPEC = "sdk-build/openapi/published.yaml"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def from_head(path: str) -> str:
    return git("show", f"HEAD:{path}")


def main() -> None:
    changed = set(git("diff", "--name-only").splitlines())
    changed.update(git("ls-files", "--others", "--exclude-standard").splitlines())
    allowed = ("sdk-build/", "src/generated/", "src/sdk/")
    unexpected = sorted(
        path for path in changed
        if not path.startswith(allowed) and path != "update-report.md"
    )
    if unexpected:
        raise ValueError(f"Unexpected source-update changes: {unexpected}")

    before_lock = json.loads(from_head(LOCK))
    after_lock = json.loads((ROOT / LOCK).read_text())
    before_surface = json.loads(from_head(SURFACE))["operations"]
    after_surface = json.loads((ROOT / SURFACE).read_text())["operations"]
    before_ids = set(before_surface)
    after_ids = set(after_surface)
    changes = sorted(
        key for key in before_ids & after_ids
        if before_surface[key] != after_surface[key]
    )
    lines = [
        "Update the unofficial Rust SDK from tracked Mistral sources.",
        "",
    ]
    for title, old, new in (
        ("OpenAPI", before_lock["openapi"], after_lock["openapi"]),
        ("Python SDK", before_lock["official_sdks"]["python"], after_lock["official_sdks"]["python"]),
        ("TypeScript SDK", before_lock["official_sdks"]["typescript"], after_lock["official_sdks"]["typescript"]),
    ):
        lines.append(f"- {title}: `{old['commit']}` -> `{new['commit']}` ({new['repository']})")
    lines.extend([
        "",
        f"- Official SDK operation mappings added: {len(after_ids - before_ids)}",
        f"- Official SDK operation mappings removed: {len(before_ids - after_ids)}",
        f"- Official SDK public paths changed: {len(changes)}",
        "",
        "Review source, compatibility definition, API/coverage changes and CI before merging.",
    ])
    (ROOT / "update-report.md").write_text("\n".join(lines) + "\n")
    print("Wrote update-report.md")


if __name__ == "__main__":
    main()
