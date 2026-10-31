#!/usr/bin/env python3
"""Audit pinned official SDK HTTP/non-HTTP evidence against Fern inventory."""
from __future__ import annotations

import argparse
from collections import defaultdict
import json
from pathlib import Path
from typing import Any

import harvest

HERE = Path(__file__).resolve().parent
REPORT_JSON = HERE / "parity-audit.json"
REPORT_MD = HERE / "PARITY_AUDIT.md"
BACKTICK = chr(96)


def _key(item: dict[str, Any]) -> tuple[str, str]:
    return item["http_method"], harvest.normalized_http_path(item["http_path"])


def build_report(inventory: dict[str, Any]) -> dict[str, Any]:
    """Keep official identities, no inference of hidden wire contracts."""
    source_keys = {_key(op) for op in inventory["operations"]}
    groups: dict[tuple[str, str], dict[str, Any]] = {}
    for sdk in ("python", "typescript"):
        for record in inventory["sdk_only"][sdk]:
            key = _key(record)
            group = groups.setdefault(key, {
                "http_method": key[0],
                "http_path": key[1],
                "sdk_evidence": {"python": [], "typescript": []},
            })
            fields = ("operation_id", "public_path", "normalized_public_path",
                      "transport_variant", "reason", "source_file")
            group["sdk_evidence"][sdk].append({
                name: record[name] for name in fields if name in record
            })

    gaps = []
    for key in sorted(groups):
        item = groups[key]
        if key in source_keys:
            item["classification"] = "matching_route_unresolved"
        elif all(item["sdk_evidence"][sdk] for sdk in ("python", "typescript")):
            item["classification"] = "sdk_observed_both"
        else:
            item["classification"] = "sdk_observed_one"
        for sdk in ("python", "typescript"):
            item["sdk_evidence"][sdk].sort(
                key=lambda x: (x["operation_id"], x["public_path"],
                               x["transport_variant"])
            )
        gaps.append(item)

    counts = {kind: sum(item["classification"] == kind for item in gaps)
              for kind in ("sdk_observed_both", "sdk_observed_one",
                           "matching_route_unresolved")}
    non_http = {
        sdk: sorted(inventory.get("non_http_resources", {}).get(sdk, []),
                    key=lambda x: (x.get("normalized_public_path", ""),
                                   x.get("class", "")))
        for sdk in ("python", "typescript")
    }
    return {
        "schema_version": 1,
        "evidence": inventory["provenance"],
        "methodology": {
            "source": "sdk-build/fern/inventory.json",
            "identity": "exact normalized HTTP method/path; operation IDs reconciled by harvest.py",
            "caveat": "SDK-observed routes are candidates, not validated wire contracts. Placeholder renames and transport aliases can cause false positives.",
            "typescript_non_http": "not exhaustively covered by the current parser",
        },
        "summary": {
            **inventory["summary"],
            "candidate_sdk_only_routes_both": counts["sdk_observed_both"],
            "candidate_sdk_only_routes_one": counts["sdk_observed_one"],
            "existing_source_routes_unresolved": counts["matching_route_unresolved"],
            "unmatched_distinct_routes": len(gaps),
        },
        "sdk_only_http": gaps,
        "non_http_resources": non_http,
        "cross_sdk_public_path_divergences": inventory["cross_sdk_divergences"],
        "openapi_only_operation_ids": inventory["openapi_only"],
    }


def render_markdown(report: dict[str, Any]) -> str:
    summary, evidence = report["summary"], report["evidence"]
    lines = [
        "# Pinned Mistral official SDK parity audit", "",
        "**Evidence only: SDK-observed routes are not approved HTTP contracts.**", "",
        "## Pinned sources", "",
        "| Source | Revision |", "| --- | --- |",
        "| OpenAPI | " + BACKTICK + str(evidence["protocol"]["commit"]) + BACKTICK + " |",
        "| Python | " + BACKTICK + evidence["python"]["commit"] + BACKTICK + " |",
        "| TypeScript | " + BACKTICK + evidence["typescript"]["commit"] + BACKTICK + " |", "",
        "## HTTP summary", "", "| Measure | Count |", "| --- | ---: |",
    ]
    measures = (
        ("Published OpenAPI operations", "raw_operations"),
        ("OpenAPI operations with official SDK taxonomy", "upstream_with_official_taxonomy"),
        ("Extracted Python SDK methods", "python_methods"),
        ("Extracted TypeScript SDK methods", "typescript_methods"),
        ("Unresolved Python methods", "python_unresolved"),
        ("Unresolved TypeScript methods", "typescript_unresolved"),
        ("New HTTP routes observed in both SDKs", "candidate_sdk_only_routes_both"),
        ("New HTTP routes observed in only one SDK", "candidate_sdk_only_routes_one"),
        ("Existing OpenAPI routes with unresolved SDK identity", "existing_source_routes_unresolved"),
        ("Public method taxonomy divergences", "cross_sdk_divergences"),
    )
    lines.extend(f"| {label} | {summary[key]} |" for label, key in measures)
    lines += [
        "", "Unresolved methods are not necessarily missing HTTP operations.",
        "Matching uses exact normalized method/path, not inferred request shapes.", "",
        "## Tier 2 — candidates for contract review", "",
        "| HTTP route | Official SDKs | Classification |",
        "| --- | --- | --- |",
    ]
    for item in report["sdk_only_http"]:
        names = [sdk for sdk in ("python", "typescript") if item["sdk_evidence"][sdk]]
        route = item["http_method"] + " " + item["http_path"]
        lines.append(
            "| " + BACKTICK + route.replace("|", "\\|") + BACKTICK
            + " | " + ", ".join(names) + " | " + item["classification"] + " |"
        )
    if not report["sdk_only_http"]:
        lines.append("| No unresolved routes | — | — |")
    lines += [
        "", "Machine-readable evidence contains original operation IDs, public",
        "paths, transport variants, and matching failures.", "",
        "## Tier 3 — non-HTTP resources (Python parser evidence)", "",
    ]
    for r in report["non_http_resources"]["python"]:
        lines.append(
            "- " + BACKTICK + r["public_path"] + BACKTICK + " ("
            + BACKTICK + r["class"] + BACKTICK + "; "
            + BACKTICK + r["source_file"] + BACKTICK + ")"
        )
    if not report["non_http_resources"]["python"]:
        lines.append("- No Python resource candidates in the current parser.")
    lines += [
        "", "**Limitations:** a resource class is not proof of WebSocket or",
        "realtime behavior. The TypeScript parser currently extracts HTTP only;",
        "it cannot prove the absence of non-HTTP TypeScript capabilities.", "",
        "## Next actions", "",
        "1. Validate HTTP request/response/authentication/error semantics for each",
        "   SDK-observed route before accepting a separate Tier-2 contract.",
        "2. Review parameter rename and stream alias false positives.",
        "3. Inventory Python and TypeScript non-HTTP protocol methods and classify",
        "   realtime/WebSocket vs local convenience helpers.", "",
    ]
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=("update", "check"))
    args = parser.parse_args()
    inventory = harvest.generate(harvest.load_json(harvest.LOCK_PATH))
    report = build_report(inventory)
    as_json = json.dumps(report, indent=2, sort_keys=True, ensure_ascii=False) + "\n"
    as_md = render_markdown(report)
    if args.command == "update":
        REPORT_JSON.write_text(as_json)
        REPORT_MD.write_text(as_md)
        print(f"Updated parity evidence: {report['summary']}")
    else:
        if REPORT_JSON.read_text() != as_json or REPORT_MD.read_text() != as_md:
            raise SystemExit("Pinned official SDK parity evidence is not reproducible")
        print("Pinned official SDK parity evidence is reproducible")


if __name__ == "__main__":
    main()
