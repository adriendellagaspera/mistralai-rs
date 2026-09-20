"""Regression tests for repository policy and pull-request conventions."""

import sys
import unittest
from unittest.mock import patch

import policy
import pr_title
import update_report


class RepositoryScriptsTests(unittest.TestCase):
    def test_repository_policy(self):
        policy.main()

    def test_update_report_resolves_repository_sources(self):
        self.assertEqual(update_report.ROOT, policy.ROOT)
        for path in (update_report.LOCK, update_report.SURFACE, update_report.SPEC):
            with self.subTest(path=path):
                self.assertTrue((update_report.ROOT / path).is_file())

    def test_valid_pr_titles(self):
        titles = (
            "refactor: clarify sdk-build boundaries",
            "fix(sdk-build): preserve parity",
            'Revert "fix: example"',
        )
        for title in titles:
            with self.subTest(title=title), patch.object(sys, "argv", ["pr_title.py", title]):
                pr_title.main()

    def test_invalid_pr_titles(self):
        for title in ("refactor without separator", "unknown: title", "fix:"):
            with self.subTest(title=title), patch.object(sys, "argv", ["pr_title.py", title]):
                with self.assertRaises(SystemExit):
                    pr_title.main()


if __name__ == "__main__":
    unittest.main()
