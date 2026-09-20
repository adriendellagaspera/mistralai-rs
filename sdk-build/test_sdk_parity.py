"""Regression tests for exact Mistral SDK coverage and facade parity."""

import unittest
from collections import Counter

from build import require_sdk_parity


class SdkParityTests(unittest.TestCase):
    def test_zero_rejections_and_identical_committed_facade_pass(self):
        require_sdk_parity(Counter({"derived": 169, "overridden": 4}), [])

    def test_full_coverage_is_not_equivalent_to_public_facade_parity(self):
        with self.assertRaisesRegex(ValueError, "committed public baseline in 2 files"):
            require_sdk_parity(
                Counter({"derived": 169, "overridden": 4}),
                ["client.rs", "models.rs"],
            )

    def test_identical_facade_does_not_hide_missing_operations(self):
        with self.assertRaisesRegex(ValueError, "1 OpenAPI operations remain rejected"):
            require_sdk_parity(
                Counter({"derived": 168, "overridden": 4, "rejected": 1}),
                [],
            )


if __name__ == "__main__":
    unittest.main()
