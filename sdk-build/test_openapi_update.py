"""Pinning regressions for the versioned OpenAPI source, not the docs catalog."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("openapi_update", HERE / "openapi" / "update.py")
assert SPEC is not None and SPEC.loader is not None
update = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(update)

OLD = b"openapi: 3.1.0\ninfo:\n  title: Old\n"
NEW = b"openapi: 3.1.0\ninfo:\n  title: New\n"
COMMIT_OLD = "a" * 40
COMMIT_NEW = "b" * 40


class OpenApiPinTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.openapi = root / "openapi"
        self.openapi.mkdir()
        self.lock = root / "provenance.lock.json"
        self.source = {
            "repository": "mistralai/platform-docs-public",
            "path": "public/openapi.yaml",
            "commit": COMMIT_OLD,
            "sha256": hashlib.sha256(OLD).hexdigest(),
            # An older lock may still contain the erroneous mirror field.
            "published_url": "https://docs.mistral.ai/openapi.yaml",
        }
        self.lock.write_text(json.dumps({"openapi": self.source}, indent=2) + "\n")
        (self.openapi / "published.yaml").write_bytes(OLD)
        for name, value in (("LOCK", self.lock), ("HERE", self.openapi)):
            replacement = patch.object(update, name, value)
            replacement.start()
            self.addCleanup(replacement.stop)

    @staticmethod
    def latest(commit: str) -> bytes:
        return json.dumps([{"sha": commit}]).encode()

    def test_unchanged_source_does_not_fetch_distinct_docs_catalog(self) -> None:
        initial = self.lock.read_bytes()
        with patch.object(update, "fetch", side_effect=[self.latest(COMMIT_OLD), OLD]) as fetch:
            update.main()
        self.assertEqual(fetch.call_count, 2)
        self.assertEqual(
            fetch.call_args_list[1].args[0],
            f"https://raw.githubusercontent.com/mistralai/platform-docs-public/{COMMIT_OLD}/public/openapi.yaml",
        )
        self.assertEqual(self.lock.read_bytes(), initial)
        self.assertEqual((self.openapi / "published.yaml").read_bytes(), OLD)

    def test_changed_source_advances_pin_after_fetching_license(self) -> None:
        with patch.object(
            update, "fetch",
            side_effect=[self.latest(COMMIT_NEW), NEW, b"Apache-2.0"],
        ) as fetch, patch.object(update, "optional_notice", return_value=None):
            update.main()
        self.assertEqual(fetch.call_count, 3)
        self.assertTrue(fetch.call_args_list[2].args[0].endswith(f"/{COMMIT_NEW}/LICENSE"))
        lock = json.loads(self.lock.read_text())["openapi"]
        self.assertEqual(lock["commit"], COMMIT_NEW)
        self.assertEqual(lock["sha256"], hashlib.sha256(NEW).hexdigest())
        self.assertEqual((self.openapi / "published.yaml").read_bytes(), NEW)
        self.assertEqual((self.openapi / "LICENSE").read_bytes(), b"Apache-2.0")

    def test_failed_license_acquisition_leaves_pin_and_snapshot_intact(self) -> None:
        initial = self.lock.read_bytes()
        with patch.object(
            update, "fetch",
            side_effect=[self.latest(COMMIT_NEW), NEW, RuntimeError("license unavailable")],
        ):
            with self.assertRaisesRegex(RuntimeError, "license unavailable"):
                update.main()
        self.assertEqual(self.lock.read_bytes(), initial)
        self.assertEqual((self.openapi / "published.yaml").read_bytes(), OLD)

    def test_invalid_upstream_dialect_leaves_pin_and_snapshot_intact(self) -> None:
        initial = self.lock.read_bytes()
        invalid = b"openapi: 3.0.3\n"
        with patch.object(update, "fetch", side_effect=[self.latest(COMMIT_NEW), invalid]):
            with self.assertRaisesRegex(ValueError, "dialect changed"):
                update.main()
        self.assertEqual(self.lock.read_bytes(), initial)
        self.assertEqual((self.openapi / "published.yaml").read_bytes(), OLD)

    def test_invalid_upstream_commit_leaves_pin_and_snapshot_intact(self) -> None:
        initial = self.lock.read_bytes()
        with patch.object(update, "fetch", side_effect=[self.latest("not-a-commit")]) as fetch:
            with self.assertRaisesRegex(ValueError, "Invalid upstream OpenAPI commit"):
                update.main()
        self.assertEqual(fetch.call_count, 1)
        self.assertEqual(self.lock.read_bytes(), initial)


if __name__ == "__main__":
    unittest.main()
