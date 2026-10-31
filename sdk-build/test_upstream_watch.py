"""Offline regression tests for independent Mistral upstream monitoring."""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("upstream_watch", HERE / "monitoring" / "watch.py")
assert SPEC is not None and SPEC.loader is not None
watch = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(watch)

BASE = b"openapi: 3.1.0\npaths:\n  /v1/hello:\n    get:\n      operationId: hello\n"
CHANGED = b"openapi: 3.1.0\npaths:\n  /v1/hello:\n    get:\n      operationId: hello\n  /v1/new:\n    post:\n      operationId: new\n"
OLD_PY, NEW_PY = "a" * 40, "b" * 40
OLD_TS, NEW_TS = "c" * 40, "d" * 40


class UpstreamWatchTests(unittest.TestCase):
    def setUp(self) -> None:
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        (root / "sdk-build/openapi").mkdir(parents=True)
        (root / "sdk-build/monitoring").mkdir(parents=True)
        self.source = root / "sdk-build/openapi/published.yaml"
        self.source.write_bytes(BASE)
        self.lock = root / "sdk-build/provenance.lock.json"
        self.catalog = root / "sdk-build/openapi/public-catalog.lock.json"
        self.report = root / "sdk-build/monitoring/STATUS.md"
        self.state = root / "status.json"
        self.outputs = root / "outputs.txt"
        self.original = {
            "openapi": {
                "source_kind": "published_catalog",
                "url": watch.PUBLIC_URL,
                "sha256": watch.sha256(BASE),
            },
            "official_sdks": {
                "python": {"repository": watch.SOURCES["python"], "commit": OLD_PY},
                "typescript": {"repository": watch.SOURCES["typescript"], "commit": OLD_TS},
            },
        }
        self.lock.write_text(json.dumps(self.original))
        self.catalog.write_text(json.dumps({
            "url": watch.PUBLIC_URL,
            "sha256": watch.sha256(BASE),
            "reviewed_at": "2026-09-01",
        }))
        for name, value in (
            ("LOCK", self.lock), ("CATALOG_LOCK", self.catalog),
            ("SOURCE", self.source), ("STATE", self.state), ("REPORT", self.report)
        ):
            active = patch.object(watch, name, value)
            active.start()
            self.addCleanup(active.stop)
        env = patch.dict(os.environ, {"GITHUB_OUTPUT": str(self.outputs)})
        env.start()
        self.addCleanup(env.stop)

    def test_no_drift_no_candidate(self) -> None:
        with patch.object(watch, "download", return_value=BASE), patch.object(
            watch, "latest_commit", side_effect=[OLD_PY, OLD_TS]
        ):
            watch.prepare()
        result = json.loads(self.state.read_text())
        self.assertFalse(result["changed"])
        self.assertFalse(self.report.exists())
        self.assertIn("changed=false", self.outputs.read_text())

    def test_upstream_drift_prepares_explicit_reviewed_candidate(self) -> None:
        with patch.object(watch, "download", return_value=CHANGED), patch.object(
            watch, "latest_commit", side_effect=[NEW_PY, NEW_TS]
        ):
            watch.prepare()
        watch.finish("failure")
        locked = json.loads(self.lock.read_text())
        self.assertEqual(self.source.read_bytes(), CHANGED)
        self.assertEqual(locked["openapi"]["source_kind"], "published_catalog")
        self.assertEqual(locked["openapi"]["sha256"], watch.sha256(CHANGED))
        self.assertEqual(locked["official_sdks"]["python"]["commit"], NEW_PY)
        self.assertEqual(locked["official_sdks"]["typescript"]["commit"], NEW_TS)
        self.assertEqual(json.loads(self.catalog.read_text())["reviewed_at"], "PENDING_PR_REVIEW")
        report = self.report.read_text()
        self.assertIn("POST /v1/new", report)
        self.assertIn("FAILURE", report)
        self.assertIn("changed=true", self.outputs.read_text())

    def test_source_failure_does_not_suppress_sdk_updates(self) -> None:
        with patch.object(watch, "download", side_effect=OSError("catalog unavailable")), patch.object(
            watch, "latest_commit", side_effect=[NEW_PY, OLD_TS]
        ):
            watch.prepare()
        watch.finish("failure")
        result = json.loads(self.state.read_text())
        self.assertIn("openapi", result["errors"])
        self.assertTrue(result["changes"]["python"])
        self.assertFalse(result["changes"]["typescript"])
        self.assertEqual(self.source.read_bytes(), BASE)
        self.assertIn("catalog unavailable", self.report.read_text())

    def test_python_sdk_failure_does_not_suppress_typescript(self) -> None:
        def sdk(repository: str) -> str:
            if repository.endswith("client-python"):
                raise OSError("python source missing")
            return NEW_TS
        with patch.object(watch, "download", return_value=BASE), patch.object(
            watch, "latest_commit", side_effect=sdk
        ):
            watch.prepare()
        result = json.loads(self.state.read_text())
        self.assertIn("python", result["errors"])
        self.assertTrue(result["changes"]["typescript"])

    def test_semantic_diff_detects_shape_changes(self) -> None:
        changes = watch.diff(watch.surface(BASE), watch.surface(CHANGED))
        self.assertEqual(changes["operations"]["added"], ["POST /v1/new"])
        self.assertFalse(changes["operations"]["removed"])


class FernInventoryAdapterTests(unittest.TestCase):
    def test_fern_inventory_is_the_only_taxonomy_source(self) -> None:
        spec = importlib.util.spec_from_file_location(
            "official_sdk_harvest", HERE / "official-sdks" / "harvest.py"
        )
        assert spec is not None and spec.loader is not None
        harvest = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(harvest)
        with tempfile.TemporaryDirectory() as temp:
            fixture = Path(temp) / "inventory.json"
            fixture.write_text(json.dumps({
                "operations": [{
                    "operation_id": "hello",
                    "http_method": "GET",
                    "http_path": "/v1/hello",
                    "outcome": "published",
                }],
            }))
            with patch.object(harvest, "FERN_INVENTORY_PATH", fixture):
                coverage = harvest.fern_coverage()
        self.assertEqual(coverage["operations"], [{
            "operation_id": "hello",
            "method": "GET",
            "path": "/v1/hello",
            "upstream": True,
        }])

    def test_incomplete_inventory_fails_closed(self) -> None:
        spec = importlib.util.spec_from_file_location(
            "official_sdk_harvest", HERE / "official-sdks" / "harvest.py"
        )
        assert spec is not None and spec.loader is not None
        harvest = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(harvest)
        with tempfile.TemporaryDirectory() as temp:
            fixture = Path(temp) / "inventory.json"
            fixture.write_text(json.dumps({"operations": [{"http_method": "GET"}]}))
            with patch.object(harvest, "FERN_INVENTORY_PATH", fixture):
                with self.assertRaisesRegex(ValueError, "Incomplete Fern"):
                    harvest.fern_coverage()


if __name__ == "__main__":
    unittest.main()
