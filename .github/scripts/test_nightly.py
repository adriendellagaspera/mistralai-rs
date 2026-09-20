"""Regression tests for exhaustive, non-short-circuiting nightly checks."""

from __future__ import annotations

import contextlib
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

import nightly
import update_report


class NightlyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.results = Path(self.directory.name) / "results"
        replacement = patch.object(nightly, "RESULTS", self.results)
        replacement.start()
        self.addCleanup(replacement.stop)

    def test_failure_does_not_prevent_following_check(self) -> None:
        with contextlib.redirect_stdout(io.StringIO()):
            nightly.run("sources", "openapi", [sys.executable, "-c", "raise SystemExit(5)"])
            nightly.run("sources", "official_python", [sys.executable, "-c", "print('ok')"])
            nightly.run("sources", "official_typescript", [sys.executable, "-c", "print('ok')"])
        entries = nightly.load("sources")["checks"]
        self.assertEqual(entries["openapi"]["status"], "FAIL")
        self.assertIn("exit 5", entries["openapi"]["detail"])
        self.assertEqual(entries["official_python"]["status"], "PASS")
        self.assertEqual(entries.get("official_typescript", {}).get("status"), "PASS")

    def test_failure_diagnostic_includes_process_output(self) -> None:
        with contextlib.redirect_stdout(io.StringIO()):
            nightly.run(
                "candidate", "raw",
                [sys.executable, "-c", "print('test root cause'); raise SystemExit(1)"],
            )
        self.assertIn("test root cause", nightly.load("candidate")["checks"]["raw"]["detail"])

    def test_missing_result_is_blocked_not_success(self) -> None:
        rows, healthy = nightly.combined(False)
        self.assertFalse(healthy)
        self.assertTrue(all(row["status"] == "BLOCKED" for row in rows))

    def test_independent_baseline_success_and_candidate_failure(self) -> None:
        with contextlib.redirect_stdout(io.StringIO()):
            for job, checks in nightly.BASELINE.items():
                for check in checks:
                    nightly.record(job, check, "PASS")
            nightly.record("candidate", "raw", "FAIL", "source failure")
            nightly.record("candidate", "surface", "PASS")
            nightly.record("candidate", "generate", "BLOCKED", "raw failure")
            nightly.record("candidate", "check", "BLOCKED", "generation unavailable")
            nightly.record("candidate", "official_check", "PASS")
            for job, checks in nightly.CANDIDATE.items():
                if job == "candidate":
                    continue
                for check in checks:
                    nightly.record(job, check, "BLOCKED", "generation unavailable")
        self.assertTrue(nightly.combined(False)[1])
        rows, healthy = nightly.combined(True)
        self.assertFalse(healthy)
        self.assertEqual(next(row for row in rows if row["check"] == "raw")["status"], "FAIL")
        self.assertEqual(next(row for row in rows if row["check"] == "python_tests")["status"], "PASS")

    def test_report_contains_every_result(self) -> None:
        output = Path(self.directory.name) / "report.md"
        with contextlib.redirect_stdout(io.StringIO()):
            nightly.record("sources", "openapi", "FAIL", "mirror changed")
            nightly.summary(False, output, "https://example.invalid/run")
        report = output.read_text()
        self.assertIn("mirror changed", report)
        self.assertIn("baseline-tooling", report)
        self.assertIn("BLOCKED", report)
        self.assertIn("Overall: FAIL", report)

    def test_invalid_job_id_is_rejected(self) -> None:
        with self.assertRaises(ValueError):
            nightly.result_path("../arbitrary")


class UpdatePrDescriptionTests(unittest.TestCase):
    def setUp(self) -> None:
        self.before_lock = {
            "openapi": {"repository": "mistralai/platform-docs-public", "commit": "a" * 40},
            "official_sdks": {
                "python": {"repository": "mistralai/client-python", "commit": "b" * 40},
                "typescript": {"repository": "mistralai/client-ts", "commit": "c" * 40},
            },
        }
        self.after_lock = json.loads(json.dumps(self.before_lock))
        self.after_lock["official_sdks"]["python"]["commit"] = "d" * 40
        self.before_surface = {"operations": {"one": ["old"], "gone": ["removed"]}}
        self.after_surface = {"operations": {"one": ["new"], "added": ["fresh"]}}
        self.before_coverage = {"generated_methods": 183}
        self.after_coverage = {"generated_methods": 184}

    def body(self, rows: list[dict], healthy: bool = False) -> str:
        return update_report.render(
            self.before_lock, self.after_lock,
            self.before_surface, self.after_surface,
            self.before_coverage, self.after_coverage,
            {"src/generated/client.rs", "src/generated/coverage.json",
             "src/sdk/mod.rs", "sdk-build/provenance.lock.json"},
            rows, healthy, "https://github.com/owner/repo/actions/runs/123",
        )

    def test_reviewable_pr_body_is_distinct_from_full_check_report(self) -> None:
        body = self.body([
            {"job": "sources", "check": "openapi", "status": "FAIL",
             "detail": "python3 update.py (exit 1)\\nTraceback...\\nValueError: upstream spec differs"},
            {"job": "candidate", "check": "raw", "status": "PASS", "detail": "all good"},
        ])
        self.assertIn("### Source revisions", body)
        self.assertIn("mistralai/client-python/commit/" + "d" * 40, body)
        self.assertNotIn("mistralai/client-ts/commit/", body)
        self.assertIn("Generated methods: **183 → 184**", body)
        self.assertIn("(1 added, 1 removed, 1 modified)", body)
        self.assertIn("**FAIL** — 1 passed, 1 failed", body)
        self.assertIn("ValueError: upstream spec differs", body)
        self.assertNotIn("Traceback", body)
        self.assertNotIn("| Job | Check | Status | Detail |", body)
        self.assertIn("the PR's own CI and API review run separately", body)
        self.assertIn("https://github.com/owner/repo/actions/runs/123", body)

    def test_unchanged_pins_and_healthy_nightly(self) -> None:
        self.after_lock = self.before_lock
        body = self.body([{"job": "candidate", "check": "generate",
                           "status": "PASS"}], healthy=True)
        self.assertIn("No source revision changed", body)
        self.assertIn("**PASS** — 1 passed, 0 failed", body)
        self.assertNotIn("Checks requiring attention:", body)

    def test_diagnostic_escapes_markdown_and_bounds_output(self) -> None:
        body = self.body([{"job": "sources", "check": "openapi",
                           "status": "FAIL", "detail": "irrelevant\\n" + "a|b" * 200}])
        self.assertIn("a\\\\|b", body)
        self.assertIn("…", body)
        self.assertNotIn("irrelevant", body)


if __name__ == "__main__":
    unittest.main()
