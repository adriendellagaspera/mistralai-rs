"""Regression tests for the isolated SDK-build coverage guard."""

import unittest

from coverage_gate import validate_coverage


class CoverageGateTests(unittest.TestCase):
    def setUp(self):
        self.baseline = {
            "schema_version": 1,
            "total_operations": 3,
            "previously_rejected_operations": ["a"],
            "approved_overridden_operations": ["b"],
        }
        self.report = {
            "a": {"status": "rejected"},
            "b": {"status": "overridden"},
            "c": {"status": "derived"},
        }

    def check(self, report=None, baseline=None, source=None):
        return validate_coverage(
            report if report is not None else self.report,
            baseline if baseline is not None else self.baseline,
            source if source is not None else self.report,
        )

    def test_existing_coverage_accepted(self):
        self.assertEqual(
            self.check(), {"derived": 1, "overridden": 1, "rejected": 1}
        )

    def test_previous_rejection_can_be_proven(self):
        self.report["a"]["status"] = "derived"
        self.report["b"]["status"] = "derived"
        self.assertEqual(self.check(), {"derived": 3})

    def test_regression_to_rejected_fails(self):
        self.report["c"]["status"] = "rejected"
        with self.assertRaisesRegex(ValueError, "new_rejections=\\['c'\\]"):
            self.check()

    def test_unreviewed_override_fails(self):
        self.report["c"]["status"] = "overridden"
        with self.assertRaisesRegex(ValueError, "unreviewed_overrides=\\['c'\\]"):
            self.check()

    def test_missing_operation_fails(self):
        del self.report["c"]
        with self.assertRaisesRegex(ValueError, "inventory drift"):
            self.check(source=("a", "b", "c"))

    def test_replaced_operation_identity_fails(self):
        self.report["d"] = self.report.pop("c")
        with self.assertRaisesRegex(ValueError, "inventory drift"):
            self.check(source=("a", "b", "c"))

    def test_unknown_source_operation_fails(self):
        with self.assertRaisesRegex(ValueError, "inventory drift"):
            self.check(source=("a", "b", "d"))

    def test_unknown_status_fails(self):
        self.report["c"]["status"] = "skipped"
        with self.assertRaisesRegex(ValueError, "unknown SDK derivation statuses"):
            self.check()

    def test_unreviewed_baseline_identity_fails(self):
        self.baseline["previously_rejected_operations"].append("not-in-source")
        with self.assertRaisesRegex(ValueError, "invalid or unknown"):
            self.check()


if __name__ == "__main__":
    unittest.main()
