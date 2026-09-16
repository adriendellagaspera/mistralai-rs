import sys
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))

from sdk_contracts import compare_surface, public_surface


class SdkContractTests(unittest.TestCase):
    def test_duplicate_methods_and_invalid_identifiers_fail_before_emission(self):
        with self.assertRaisesRegex(ValueError, "duplicate emitted symbol"):
            public_surface(
                {"test.rs": "pub struct A; impl A { pub fn x() {} pub fn x() {} }"}
            )
        with self.assertRaisesRegex(ValueError, "invalid emitted Rust"):
            public_surface({"test.rs": "pub struct 123;"})

    def test_surface_changes_are_classified_conservatively(self):
        before = {"A::new": "pub fn new() -> Self"}
        self.assertEqual(compare_surface(before, before)["classification"], "unchanged")
        self.assertEqual(
            compare_surface(before, before | {"A::limit": "pub fn limit(i64)"})[
                "classification"
            ],
            "additive",
        )
        self.assertEqual(
            compare_surface(before, {})["classification"], "review_required"
        )

    def test_grouped_reexport_addition_is_additive(self):
        before = {
            "mod.rs::reexport::pub use facade_types::{A};": "pub use facade_types::{A};",
        }
        after = {
            "mod.rs::reexport::pub use facade_types::{A, B};": "pub use facade_types::{A, B};",
        }
        report = compare_surface(before, after)
        self.assertEqual(report["classification"], "additive")
        self.assertEqual(report["removed"], [])
        self.assertEqual(
            report["added"],
            ["mod.rs::reexport::pub use facade_types::B;"],
        )


if __name__ == "__main__":
    unittest.main()
