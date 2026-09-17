import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import build as codegen
import openapi as update_spec
import preprocess


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

    @staticmethod
    def published_shape_fixture():
        def operation(operation_id, media):
            return {
                "operationId": operation_id,
                "responses": {"200": {"content": {name: {} for name in media}}},
            }

        return {
            "openapi": "3.1.0",
            "components": {
                "schemas": {
                    "ChatCompletionResponse": {
                        "allOf": [
                            {"$ref": "#/components/schemas/ChatCompletionResponseBase"},
                            {
                                "type": "object",
                                "properties": {"choices": {}},
                                "required": ["id", "data", "choices"],
                            },
                        ]
                    },
                    "SharingDelete": {"properties": {}, "required": ["level"]},
                    "WorkflowListResponse": {
                        "properties": {"workflows": {}},
                        "required": ["beta.workflows"],
                    },
                }
            },
            "paths": {
                "/v1/chat/completions": {
                    "post": operation("chat", ["application/json", "text/event-stream"])
                },
                "/v1/fim/completions": {
                    "post": operation("fim", ["application/json", "text/event-stream"])
                },
                "/v1/audio/speech": {
                    "post": operation("speech", ["application/json", "text/event-stream"])
                },
                "/v1/audio/voices/{voice_id}/sample": {
                    "get": operation("voice_sample", ["application/json", "audio/wav"])
                },
            },
        }

    def test_preprocessing_keeps_contract_repairs_declarative(self):
        source = self.published_shape_fixture()
        result = json.loads(preprocess.preprocess(json.dumps(source).encode()))
        schemas = result["components"]["schemas"]
        self.assertIn("data", schemas["ChatCompletionResponse"]["allOf"][1]["required"])
        self.assertIn("level", schemas["SharingDelete"]["required"])
        self.assertIn("beta.workflows", schemas["WorkflowListResponse"]["required"])
        self.assertNotIn("workflows", schemas["WorkflowListResponse"]["required"])
        self.assertIn("/v1/chat/completions#stream", result["paths"])
        self.assertIn("/v1/audio/voices/{voice_id}/sample#wav", result["paths"])

    def test_preprocessing_fails_closed_when_overlay_target_changes(self):
        source = self.published_shape_fixture()
        source["components"]["schemas"]["SharingDelete"]["required"].remove("level")
        with self.assertRaisesRegex(ValueError, "SharingDelete.level"):
            preprocess.preprocess(json.dumps(source).encode())


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
