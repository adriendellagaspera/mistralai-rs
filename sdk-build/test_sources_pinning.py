"""Pinning regressions: a failed or unchanged source must not mask the other SDK."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "official_sdk_harvest", HERE / "official-sdks" / "harvest.py"
)
assert SPEC is not None and SPEC.loader is not None
harvest = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(harvest)


class OfficialSourcePinTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.lock_path = Path(self.temp.name) / "provenance.lock.json"
        self.original = {
            "official_sdks": {
                "python": {"repository": "example/python", "commit": "old-python"},
                "typescript": {"repository": "example/typescript", "commit": "old-typescript"},
            }
        }
        self.lock_path.write_text(json.dumps(self.original, indent=4) + "\n")
        replacement = patch.object(harvest, "LOCK_PATH", self.lock_path)
        replacement.start()
        self.addCleanup(replacement.stop)

    def test_unchanged_source_does_not_reformat_lock(self) -> None:
        with patch.object(harvest, "latest_commit", return_value="old-python"):
            harvest.pin_latest(self.original.copy(), source="python")
        self.assertEqual(self.lock_path.read_text(), json.dumps(self.original, indent=4) + "\n")

    def test_one_sdk_can_advance_independently(self) -> None:
        with patch.object(harvest, "latest_commit", return_value="new-python"):
            harvest.pin_latest(self.original.copy(), source="python")
        changed = json.loads(self.lock_path.read_text())["official_sdks"]
        self.assertEqual(changed["python"]["commit"], "new-python")
        self.assertEqual(changed["typescript"]["commit"], "old-typescript")

    def test_unknown_source_is_rejected(self) -> None:
        with self.assertRaises(ValueError):
            harvest.pin_latest(self.original, source="unknown")


if __name__ == "__main__":
    unittest.main()
