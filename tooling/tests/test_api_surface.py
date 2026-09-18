import sys
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "quality"))

from api_surface import review_matches


class ApiSurfaceReviewTests(unittest.TestCase):
    def test_exact_review_matches_current_report(self):
        report = {
            "classification": "review_required",
            "removed": [],
            "changed": ["A::x"],
            "added": ["B::y"],
        }
        review = {
            "base": "abc123",
            "removed": [],
            "changed": ["A::x"],
            "added": ["B::y"],
        }
        self.assertTrue(review_matches("abc123", report, review))

    def test_review_rejects_stale_base_or_different_diff(self):
        report = {
            "classification": "review_required",
            "removed": [],
            "changed": ["A::x"],
            "added": ["B::y"],
        }
        stale = {
            "base": "old",
            "removed": [],
            "changed": ["A::x"],
            "added": ["B::y"],
        }
        different = {
            "base": "abc123",
            "removed": [],
            "changed": [],
            "added": ["B::y"],
        }
        self.assertFalse(review_matches("abc123", report, stale))
        self.assertFalse(review_matches("abc123", report, different))


if __name__ == "__main__":
    unittest.main()
