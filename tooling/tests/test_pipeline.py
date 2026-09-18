import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import build as codegen
import check_openapi
import openapi as update_spec
import preprocess
from ruamel.yaml import YAML


class ProvenanceTests(unittest.TestCase):
    def test_vendored_spec_matches_lock(self):
        lock = json.loads((codegen.ROOT / "tooling/sources/lock.json").read_text())
        codegen.verify_spec((codegen.ROOT / "tooling/sources/openapi/openapi.yaml").read_bytes(), lock)
        with self.assertRaises(ValueError):
            codegen.verify_spec(b"corrupted", lock)

    def test_file_set_and_byte_drift_are_detected(self):
        self.assertEqual(codegen.differences({"a": b"x", "deleted": b""},
                                            {"a": b"y", "added": b""}),
                         ["a", "added", "deleted"])

    def test_preprocessing_removes_only_undefined_required_data(self):
        prefix, suffix = b"openapi: 3.1.0\n", b"    NextSchema: {}\n"
        source = prefix + preprocess.CHAT_RESPONSE_REQUIRED + suffix
        result = preprocess.preprocess(source)
        self.assertEqual(
            result,
            prefix + preprocess.CHAT_RESPONSE_REQUIRED_FIXED + suffix,
        )
        self.assertNotIn(b"        - data\n", result)

    def test_preprocessing_fails_closed_when_upstream_shape_changes(self):
        with self.assertRaises(ValueError):
            preprocess.preprocess(b"openapi: 3.1.0\n")

    def test_overlay_assumptions_match_pinned_published_spec(self):
        path = codegen.ROOT / "tooling/sources/openapi/openapi.yaml"
        spec = YAML(typ="safe", pure=True).load(path.read_bytes())
        check_openapi.validate_published(spec)

    def test_overlay_assumptions_fail_closed_on_source_drift(self):
        path = codegen.ROOT / "tooling/sources/openapi/openapi.yaml"
        spec = YAML(typ="safe", pure=True).load(path.read_bytes())
        required = spec["components"]["schemas"]["ChatCompletionResponse"]["allOf"][1]["required"]
        required.remove("data")
        with self.assertRaisesRegex(ValueError, "ChatCompletionResponse"):
            check_openapi.validate_published(spec)


class UpdateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "tooling/sources/openapi").mkdir(parents=True)
        self.old_spec = b"openapi: 3.1.0\ninfo: {}\n"
        self.lock = {
            "upstream_repository": "mistralai/platform-docs-public",
            "upstream_spec_path": "openapi.yaml",
            "published_spec_url": "https://docs.mistral.ai/openapi.yaml",
            "upstream_commit": "a" * 40,
            "spec_sha256": hashlib.sha256(self.old_spec).hexdigest(),
            "generator": "openapi-to-rust", "generator_version": "0.16.0",
            "rust_toolchain": "1.94.0",
        }
        (self.root / "tooling/sources/lock.json").write_text(json.dumps(self.lock) + "\n")
        (self.root / "tooling/sources/openapi/openapi.yaml").write_bytes(self.old_spec)
        self.addCleanup(patch.stopall)
        patch.object(update_spec, "ROOT", self.root).start()

    def test_new_commit_with_identical_spec_is_strict_noop_and_checks_publication(self):
        before = codegen.snapshot(self.root)
        with patch.object(update_spec, "fetch", side_effect=[
            json.dumps([{"sha": "b" * 40}]).encode(), self.old_spec, self.old_spec,
        ]):
            update_spec.main()
        self.assertEqual(before, codegen.snapshot(self.root))

    def test_published_spec_divergence_fails_without_mutating_files(self):
        before = codegen.snapshot(self.root)
        published = self.old_spec + b"paths: {}\n"
        with patch.object(update_spec, "fetch", side_effect=[
            json.dumps([{"sha": "b" * 40}]).encode(), self.old_spec, published,
        ]), self.assertRaisesRegex(ValueError, "Published OpenAPI specification diverges"):
            update_spec.main()
        self.assertEqual(before, codegen.snapshot(self.root))

    def test_changed_spec_updates_provenance_and_second_run_is_noop(self):
        new_spec = self.old_spec + b"paths: {}\n"
        with patch.object(update_spec, "fetch", side_effect=[
            json.dumps([{"sha": "b" * 40}]).encode(), new_spec, new_spec, b"license",
        ]), patch.object(update_spec, "optional_notice", return_value=None):
            update_spec.main()
        lock = json.loads((self.root / "tooling/sources/lock.json").read_text())
        self.assertEqual(lock["upstream_commit"], "b" * 40)
        codegen.verify_spec(new_spec, lock)
        report = (self.root / "update-report.md").read_text()
        self.assertIn(self.lock["spec_sha256"], report)
        self.assertIn(self.lock["published_spec_url"], report)
        self.assertIsNone(update_spec.updated_lock(lock, "c" * 40, new_spec))

    def test_failed_acquisition_leaves_pinned_files_unchanged(self):
        before = codegen.snapshot(self.root)
        with patch.object(update_spec, "fetch", side_effect=[
            json.dumps([{"sha": "b" * 40}]).encode(), OSError("network failed"),
        ]), self.assertRaises(OSError):
            update_spec.main()
        self.assertEqual(before, codegen.snapshot(self.root))


if __name__ == "__main__":
    unittest.main()
