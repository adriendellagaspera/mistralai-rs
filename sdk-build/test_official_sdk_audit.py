"""Offline regression tests for Tier-2 and Tier-3 official SDK evidence."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE / "official-sdks"))
spec = importlib.util.spec_from_file_location(
    "official_sdk_audit", HERE / "official-sdks" / "audit.py"
)
assert spec is not None and spec.loader is not None
audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(audit)


def candidate(method: str, path: str, name: str, reason: str = "no_http_match") -> dict:
    return {
        "http_method": method, "http_path": path, "operation_id": name,
        "public_path": "beta." + name, "normalized_public_path": "beta." + name,
        "transport_variant": "default", "reason": reason,
    }


def fixture() -> dict:
    return {
        "provenance": {
            "protocol": {"repository": "https://docs.mistral.ai/openapi.yaml",
                         "commit": "a" * 64, "artifact": "sdk-build/fern/inventory.json"},
            "python": {"repository": "mistralai/client-python", "commit": "b" * 40},
            "typescript": {"repository": "mistralai/client-ts", "commit": "c" * 40},
        },
        "summary": {
            "raw_operations": 2, "upstream_operations": 2,
            "upstream_with_official_taxonomy": 1,
            "openapi_only": 1, "python_methods": 3, "typescript_methods": 3,
            "python_non_http_resources": 1, "typescript_unresolved": 2,
            "python_unresolved": 3, "cross_sdk_divergences": 0,
        },
        "operations": [
            {"http_method": "GET", "http_path": "/v1/already", "operation_id": "existing"},
            {"http_method": "POST", "http_path": "/v1/other", "operation_id": "other"},
        ],
        "sdk_only": {
            "python": [
                candidate("GET", "/v1/new", "py_new"),
                candidate("GET", "/v1/solo", "py_solo"),
                candidate("GET", "/v1/already", "py_alias"),
            ],
            "typescript": [
                candidate("GET", "/v1/new", "ts_new"),
                candidate("GET", "/v1/already", "ts_alias"),
            ],
        },
        "non_http_resources": {"python": [
            {"class": "Realtime", "public_path": "beta.realtime",
             "normalized_public_path": "beta.realtime",
             "source_file": "src/mistralai/realtime.py", "reason": "non_http_resource"},
        ]},
        "cross_sdk_divergences": [],
        "openapi_only": ["other"],
    }


class AuditTests(unittest.TestCase):
    def test_method_path_classification_keeps_all_evidence(self) -> None:
        report = audit.build_report(fixture())
        self.assertEqual(report["summary"]["candidate_sdk_only_routes_both"], 1)
        self.assertEqual(report["summary"]["candidate_sdk_only_routes_one"], 1)
        self.assertEqual(report["summary"]["existing_source_routes_unresolved"], 1)
        pairs = {(r["http_method"], r["http_path"]): r for r in report["sdk_only_http"]}
        self.assertEqual(pairs["GET", "/v1/new"]["classification"], "sdk_observed_both")
        self.assertEqual(pairs["GET", "/v1/already"]["classification"], "matching_route_unresolved")
        self.assertEqual(pairs["GET", "/v1/solo"]["classification"], "sdk_observed_one")
        self.assertEqual(
            pairs["GET", "/v1/new"]["sdk_evidence"]["typescript"][0]["operation_id"],
            "ts_new",
        )
        self.assertEqual(len(report["non_http_resources"]["python"]), 1)
        self.assertEqual(report["non_http_resources"]["typescript"], [])

    def test_markdown_is_deterministic_and_names_caveats(self) -> None:
        report = audit.build_report(fixture())
        markdown = audit.render_markdown(report)
        self.assertEqual(markdown, audit.render_markdown(report))
        self.assertIn("GET /v1/new", markdown)
        self.assertIn("matching_route_unresolved", markdown)
        self.assertIn("TypeScript parser currently extracts HTTP only", markdown)
        self.assertIn("SDK-observed routes are not approved HTTP contracts", markdown)

    def test_method_participates_in_identity(self) -> None:
        f = fixture()
        f["sdk_only"]["python"].append(candidate("POST", "/v1/new", "py_new_post"))
        report = audit.build_report(f)
        self.assertEqual(report["summary"]["candidate_sdk_only_routes_one"], 2)


if __name__ == "__main__":
    unittest.main()
