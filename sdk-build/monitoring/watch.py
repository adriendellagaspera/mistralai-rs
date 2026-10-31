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


def _code(value: object) -> str:
    """Display upstream-controlled labels without breaking Markdown."""
    import html
    escaped = html.escape(str(value).replace(chr(96), "'").replace("\n", " ").replace("\r", " "))
    return chr(96) + escaped + chr(96)


def _rev(name: str, revision: str) -> str:
    if revision == "unavailable":
        return "Unavailable"
    label = _code(revision[:12])
    if name in SOURCES and len(revision) == 40:
        return f"[{label}](https://github.com/{SOURCES[name]}/commit/{revision})"
    return label


def _details(title: str, values: list[str]) -> list[str]:
    if not values:
        return []
    result = ["<details>", f"<summary>{title} ({len(values)})</summary>", ""]
    result.extend("- " + _code(value) for value in values)
    return result + ["", "</details>", ""]


def _api_area(operation: str) -> str:
    path = operation.split(" ", 1)[-1].split("#", 1)[0]
    parts = [p for p in path.split("/") if p]
    return "/" + "/".join(parts[:2]) if parts else "Other"


def render_report(state: dict, generation_result: str) -> str:
    from collections import Counter

    info = state.get("details", {}).get("openapi", {})
    delta = info.get("diff")
    errors = state.get("errors", {})
    passed = generation_result.lower() == "success"
    source_names = {"openapi": "OpenAPI catalog", "python": "Python SDK", "typescript": "TypeScript SDK"}
    lines = [
        "# Mistral upstream synchronization", "",
        "**Human review required** | **Generation: " + ("PASSED**" if passed else "FAILED / INCOMPLETE**"),
        "",
        "> Automated candidate only. CI verifies technical consistency; a separate",
        "> GitHub PR approval is required to accept API and compatibility changes.",
        "", "## Summary", "",
        "| Source | State | Previously pinned → observed |",
        "| --- | --- | --- |",
    ]
    for name in ("openapi", "python", "typescript"):
        revision = state.get("details", {}).get(name, {}).get(
            "sha256" if name == "openapi" else "commit", "unavailable"
        )
        status = (
            "Unavailable" if name in errors
            else "Changed" if state["changes"].get(name) else "Unchanged"
        )
        lines.append(
            f"| {source_names[name]} | **{status}** | "
            f"{_rev(name, state['previous'][name])} → {_rev(name, revision)} |"
        )
    lines.append("")
    if delta:
        operations = delta["operations"]
        lines.extend([
            f"**Endpoints:** {operations['count_before']} → {operations['count_after']} "
            f"({len(operations['added'])} added, {len(operations['removed'])} removed, "
            f"{len(operations['changed'])} modified).", "",
        ])
    lines += [
        "## Review checklist", "",
        "- [ ] Verify the public OpenAPI catalog and official SDK source revisions.",
        "- [ ] Inspect removed/modified operations and schemas for breaking changes.",
        "- [ ] Require regenerated Rust sources and a green CI " + _code("gate") + " check.",
        "- [ ] Approve this PR in GitHub after the final push (never via a lockfile flag).",
        "", "**Generation attempt:** " +
        ("successful." if passed else generation_result.upper() + "; the source update is not validated as a Rust SDK."),
        "",
    ]
    if errors:
        lines += ["### Source acquisition errors", ""]
        for name in ("openapi", "python", "typescript"):
            if name in errors:
                lines.append("- **" + source_names[name] + ":** " + _code(errors[name]))
        lines.append("")
    if delta:
        lines += [
            "## OpenAPI surface changes", "",
            "| | Before | After | Added | Removed | Modified |",
            "| --- | ---: | ---: | ---: | ---: | ---: |",
        ]
        for name, item in (("Endpoints", delta["operations"]), ("Schemas", delta["schemas"])):
            lines.append(
                f"| {name} | {item['count_before']} | {item['count_after']} | "
                f"{len(item['added'])} | {len(item['removed'])} | {len(item['changed'])} |"
            )
        lines += [
            "", "A modified schema fingerprint does not necessarily indicate a breaking change.",
            "Removed operations and changed schemas should be reviewed first.", "",
            "### Most affected API areas", "",
            "| Area | Added | Removed | Modified |",
            "| --- | ---: | ---: | ---: |",
        ]
        areas: dict[str, Counter] = {}
        for action in ("added", "removed", "changed"):
            for op in delta["operations"][action]:
                areas.setdefault(_api_area(op), Counter())[action] += 1
        for area, counts in sorted(areas.items(), key=lambda item: (-sum(item[1].values()), item[0]))[:15]:
            lines.append(
                f"| {_code(area)} | {counts['added']} | {counts['removed']} | {counts['changed']} |"
            )
        lines += ["", "### Changes needing compatibility review", ""]
        for kind, action, title in (
            ("operations", "removed", "Removed endpoints"),
            ("operations", "changed", "Modified endpoints"),
            ("schemas", "removed", "Removed schemas"),
            ("schemas", "changed", "Modified schemas"),
        ):
            lines += _details(title, delta[kind][action])
        lines += ["### Additions", ""]
        lines += _details("Added endpoints", delta["operations"]["added"])
        lines += _details("Added schemas", delta["schemas"]["added"])
    lines += ["## Provenance", "", f"- OpenAPI source: <{PUBLIC_URL}>"]
    if info:
        lines.extend([
            "- Proposed SHA-256: " + _code(info["sha256"]),
            "- Previously pinned public SHA-256: " + _code(info["previous_public_sha256"]),
            f"- Catalog size: {info['bytes']:,} bytes",
        ])
    for name in ("python", "typescript"):
        if name in state.get("details", {}) and name not in errors:
            old = state["previous"][name]
            new = state["details"][name]["commit"]
            lines.append(
                f"- {source_names[name]}: [compare commits]"
                f"(https://github.com/{SOURCES[name]}/compare/{old}...{new})"
            )
    lines += [
        "", "## Acceptance", "",
        "The protected main branch requires a GitHub approving review and the CI "
        + _code("gate") + " check.",
        "If source generation fails, this PR is a **drift report, not a mergeable SDK update**.",
        "",
    ]
    return "\n".join(lines)


def finish(generation_result: str) -> None:
    state = load(STATE)
    if not state["changed"]:
        print("No upstream drift detected; pinned revisions remain current.")
        return
    REPORT.write_text(render_report(state, generation_result), encoding="utf-8")
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
