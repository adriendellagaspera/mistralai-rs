"""Offline checks for the independent published OpenAPI catalog monitor."""

from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import unittest


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "public_openapi_monitor", HERE / "openapi" / "monitor_public.py"
)
assert SPEC is not None and SPEC.loader is not None
monitor = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(monitor)


class PublicCatalogMonitorTests(unittest.TestCase):
    def test_reviewed_snapshot_is_accepted(self) -> None:
        document = b"openapi: 3.1.0\n"
        digest = hashlib.sha256(document).hexdigest()
        self.assertEqual(monitor.verify_public(document, digest), digest)

    def test_unreviewed_catalog_change_is_rejected(self) -> None:
        document = b"openapi: 3.1.0\n"
        digest = hashlib.sha256(document).hexdigest()
        with self.assertRaisesRegex(ValueError, "Published OpenAPI changed independently"):
            monitor.verify_public(document + b"# changed\n", digest)

    def test_empty_catalog_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "empty"):
            monitor.verify_public(b"", hashlib.sha256(b"").hexdigest())


if __name__ == "__main__":
    unittest.main()
