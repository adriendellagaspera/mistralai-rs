"""Detect upstream OpenAPI/SDK drift and prepare a reviewable, fail-closed PR.

The deployed OpenAPI catalog is the monitored source for *candidate* migrations.
A candidate must pass Fern/Rust CI and be reviewed before its new pins reach main.
Upstream checks are independent: one missing source never suppresses the others.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import urllib.request

import yaml

ROOT = Path(__file__).resolve().parents[2]
LOCK = ROOT / "sdk-build/provenance.lock.json"
CATALOG_LOCK = ROOT / "sdk-build/openapi/public-catalog.lock.json"
SOURCE = ROOT / "sdk-build/openapi/published.yaml"
REPORT = ROOT / "sdk-build/monitoring/STATUS.md"
STATE = Path(os.environ.get("RUNNER_TEMP", "/tmp")) / "mistral-upstream-watch.json"
PUBLIC_URL = "https://docs.mistral.ai/openapi.yaml"
SOURCES = {
    "python": "mistralai/client-python",
    "typescript": "mistralai/client-ts",
}


def sha256(contents: bytes) -> str:
    return hashlib.sha256(contents).hexdigest()


def load(path: Path) -> dict:
    return json.loads(path.read_text())


def save(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def download(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": "mistralai-rs-upstream-watch"})
    with urllib.request.urlopen(request, timeout=45) as response:
        return response.read()


def latest_commit(repo: str) -> str:
    completed = subprocess.run(
        ["git", "ls-remote", f"https://github.com/{repo}.git", "refs/heads/main"],
        capture_output=True, text=True, check=True, timeout=45,
    )
    entry = completed.stdout.strip().split()
    if len(entry) != 2 or entry[1] != "refs/heads/main" or len(entry[0]) != 40:
        raise ValueError(f"Unexpected upstream ref for {repo}: {completed.stdout!r}")
    return entry[0]


def surface(spec: bytes) -> dict:
    parsed = yaml.safe_load(spec)
    if not isinstance(parsed, dict) or not str(parsed.get("openapi", "")).startswith("3.1."):
        raise ValueError("Unexpected upstream OpenAPI dialect or empty document")
    paths = parsed.get("paths")
    if not isinstance(paths, dict) or not paths:
        raise ValueError("No paths in published OpenAPI")
    operations = {}
    for path, methods in paths.items():
        if not isinstance(methods, dict):
            continue
        for method, definition in methods.items():
            if method.lower() not in {"get", "post", "put", "delete", "patch", "head", "options", "trace"}:
                continue
            if not isinstance(definition, dict):
                raise ValueError(f"Malformed operation {method} {path}")
            name = f"{method.upper()} {path}"
            operations[name] = sha256(json.dumps(
                definition, sort_keys=True, separators=(",", ":"), default=str
            ).encode())
    if not operations:
        raise ValueError("No HTTP operations in published OpenAPI")
    schemas = parsed.get("components", {}).get("schemas", {})
    return {
        "operations": operations,
        "schemas": {
            name: sha256(json.dumps(value, sort_keys=True, default=str).encode())
            for name, value in schemas.items()
        },
    }


def diff(before: dict, after: dict) -> dict:
    result = {}
    for key in ("operations", "schemas"):
        old, new = before[key], after[key]
        result[key] = {
            "added": sorted(set(new) - set(old)),
            "removed": sorted(set(old) - set(new)),
            "changed": sorted(k for k in set(old) & set(new) if old[k] != new[k]),
            "count_before": len(old),
            "count_after": len(new),
        }
    return result


def prepare() -> None:
    pinned = load(LOCK)
    catalog = load(CATALOG_LOCK)
    baseline = SOURCE.read_bytes()
    state = {
        "errors": {},
        "changes": {},
        "details": {},
        "previous": {
            "openapi": pinned["openapi"]["sha256"],
            "python": pinned["official_sdks"]["python"]["commit"],
            "typescript": pinned["official_sdks"]["typescript"]["commit"],
        },
    }

    # Fetch and validate the public spec before changing ANY pinned input.
    try:
        current = download(PUBLIC_URL)
        current_surface = surface(current)
        old_surface = surface(baseline)
        digest = sha256(current)
        state["changes"]["openapi"] = (
            digest != state["previous"]["openapi"]
            or pinned["openapi"].get("source_kind") != "published_catalog"
            or digest != catalog["sha256"]
        )
        state["details"]["openapi"] = {
            "sha256": digest,
            "bytes": len(current),
            "diff": diff(old_surface, current_surface),
            "previous_public_sha256": catalog["sha256"],
        }
        if state["changes"]["openapi"]:
            SOURCE.write_bytes(current)
            pinned["openapi"].update({
                "source_kind": "published_catalog",
                "url": PUBLIC_URL,
                "sha256": digest,
            })
            # Remove provenance that no longer describes the candidate source.
            for obsolete in ("repository", "path", "commit"):
                pinned["openapi"].pop(obsolete, None)
            # GitHub PR approval governs acceptance, not a mutable lockfile flag.
            catalog["sha256"] = digest
            catalog.pop("reviewed_at", None)
            save(CATALOG_LOCK, catalog)
    except Exception as exc:
        state["errors"]["openapi"] = f"{type(exc).__name__}: {exc}"
        state["changes"]["openapi"] = False

    # Probe each official SDK independently, even when the OpenAPI fetch fails.
    for name, repository in SOURCES.items():
        try:
            latest = latest_commit(repository)
            state["changes"][name] = latest != state["previous"][name]
            state["details"][name] = {"repository": repository, "commit": latest}
            if state["changes"][name]:
                pinned["official_sdks"][name]["commit"] = latest
        except Exception as exc:
            state["changes"][name] = False
            state["errors"][name] = f"{type(exc).__name__}: {exc}"

    changed = any(state["changes"].values()) or bool(state["errors"])
    state["changed"] = changed
    if changed:
        save(LOCK, pinned)
    STATE.write_text(json.dumps(state, indent=2) + "\n")
    output = os.environ.get("GITHUB_OUTPUT")
    if output:
        with open(output, "a", encoding="utf-8") as handle:
            handle.write(f"changed={'true' if changed else 'false'}\n")
    print(json.dumps({
        "changed": changed,
        "changes": state["changes"],
        "errors": state["errors"],
    }, indent=2))


def finish(generation_result: str) -> None:
    state = load(STATE)
    if not state["changed"]:
        print("No upstream drift detected; reviewed pins remain current.")
        return
    lines = [
        "# Automatic Mistral upstream synchronization",
        "",
        "This is an **unreviewed candidate**. Do not merge until CI, provenance,",
        "source licensing, and any breaking API changes have been reviewed.",
        "",
        "## Source status",
        "",
        "| Source | Status | Previous pin | Observed revision |",
        "| --- | --- | --- | --- |",
    ]
    for name in ("openapi", "python", "typescript"):
        error = state["errors"].get(name)
        changed = state["changes"].get(name, False)
        observed = state["details"].get(name, {}).get(
            "sha256" if name == "openapi" else "commit", "unavailable"
        )
        status = "UNAVAILABLE" if error else "DRIFT" if changed else "MATCH"
        lines.append(f"| {name} | {status} | \`{state['previous'][name]}\` | \`{observed}\` |")
    lines += ["", "## Candidate validation", "", f"Fern/Rust regeneration: **{generation_result.upper()}**.", ""]
    if state["errors"]:
        lines += ["## Acquisition errors", ""]
        for name, error in state["errors"].items():
            lines.append(f"- **{name}:** \`{error.replace(chr(96), chr(39))}\`")
        lines.append("")
    if "openapi" in state["details"]:
        d = state["details"]["openapi"]
        lines += [
            "## OpenAPI semantic changes relative to committed Rust input",
            "",
            f"Public SHA-256: \`{d['sha256']}\`; downloaded bytes: {d['bytes']}.",
            f"Previously pinned public SHA-256: \`{d['previous_public_sha256']}\`.",
            "",
        ]
        for kind in ("operations", "schemas"):
            changes = d["diff"][kind]
            lines += [f"### {kind.title()} ({changes['count_before']} → {changes['count_after']})", ""]
            for action in ("added", "removed", "changed"):
                values = changes[action]
                lines.append(f"- {action}: **{len(values)}**")
                for value in values[:35]:
                    lines.append(f"  - \`{value}\`")
                if len(values) > 35:
                    lines.append(f"  - … and {len(values)-35} more")
            lines.append("")
    lines += [
        "## Approval through GitHub PR review",
        "",
        "This branch contains a proposed update, never a pre-approved source.",
        "Review the upstream provenance, semantic differences and breaking",
        "changes before approving in GitHub. The main-branch ruleset enforces",
        "a separate approving review, last-push approval and a passing CI gate.",
        "GitHub records approvals; no lockfile approval marker is needed.",
        "",
        "## What this PR does",
        "",
        "When the published spec changes, the PR proposes replacing the pinned source",
        "with the deployed Mistral OpenAPI catalog. This is an explicit source",
        "migration, **not** an assumed byte mirror of the previous GitHub source.",
        "The Fern candidate is regenerated, checked for complete operation accounting,",
        "and tested. Official Python and TypeScript commits advance independently.",
        "",
        "If generation is failing, this is a remediation PR, **not** a claim of compatibility.",
        "Check its CI logs and correct the generator or policy before merging.",
        "",
    ]
    REPORT.write_text("\n".join(lines))
    print(f"Wrote {REPORT}")



def assert_healthy(scope: str) -> None:
    """Keep badges independent while both schedules update the same remediation PR."""
    state = load(STATE)
    sources = ("openapi",) if scope == "spec" else ("python", "typescript")
    invalid = [
        name for name in sources
        if state["changes"].get(name) or name in state["errors"]
    ]
    if invalid:
        raise SystemExit(f"Unreviewed {scope} drift or acquisition failure: {', '.join(invalid)}")
    print(f"{scope} upstream alignment: pinned revisions match live sources")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("prepare", "finish", "health"))
    parser.add_argument("--generation", default="skipped")
    parser.add_argument("--scope", choices=("spec", "sdk"), default="sdk")
    args = parser.parse_args()
    if args.command == "prepare":
        prepare()
    elif args.command == "finish":
        finish(args.generation)
    else:
        assert_healthy(args.scope)


if __name__ == "__main__":
    main()
