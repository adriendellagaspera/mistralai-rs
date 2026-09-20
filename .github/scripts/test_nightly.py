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


if __name__ == "__main__":
    unittest.main()
