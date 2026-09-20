"""Public API review is based on checked-out Rust, not historical JSON."""

import unittest
from unittest.mock import patch

from check_api_compatibility import changed_public_rust_files, require_api_review


class ApiReviewTests(unittest.TestCase):
    def test_real_rust_diff_is_selected_without_a_snapshot(self):
        with patch("check_api_compatibility.command", return_value=(
            "src/sdk/chat.rs\nsrc/sdk/api-surface.json\n"
            "src/generated/client.rs\nsrc/lib.rs\n"
        )) as git:
            self.assertEqual(changed_public_rust_files("abc123"), [
                "src/generated/client.rs", "src/lib.rs", "src/sdk/chat.rs",
            ])
            args = git.call_args.args
            self.assertIn("--no-renames", args)
            self.assertIn("src/sdk/", args)
            self.assertIn("src/generated/", args)
            self.assertEqual(args[5], "abc123")

    def test_no_code_change_requires_no_review(self):
        require_api_review("abc123", [], {})

    def test_exact_base_and_file_inventory_must_be_reviewed(self):
        files = ["src/sdk/chat.rs"]
        approved = {"schema_version": 1, "base": "abc123", "changed_rust_files": files}
        require_api_review("abc123", files, approved)
        for invalid in (
            {},
            {**approved, "base": "other"},
            {**approved, "changed_rust_files": []},
            {**approved, "schema_version": 0},
        ):
            with self.subTest(invalid=invalid):
                with self.assertRaisesRegex(SystemExit, "explicit API review"):
                    require_api_review("abc123", files, invalid)


if __name__ == "__main__":
    unittest.main()
