import hashlib
import json
from pathlib import Path
import tempfile
import tomllib
import unittest
from unittest.mock import patch

import build as codegen
import check_openapi
import openapi as update_spec
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

    def test_stream_discriminator_assumptions_fail_closed_on_source_drift(self):
        path = codegen.ROOT / "tooling/sources/openapi/openapi.yaml"
        spec = YAML(typ="safe", pure=True).load(path.read_bytes())
        spec["components"]["schemas"]["ChatCompletionRequest"]["properties"]["stream"]["type"] = "string"
        with self.assertRaisesRegex(ValueError, "ChatCompletionRequest.stream"):
            check_openapi.validate_published(spec)

    def test_generator_config_owns_overlay_manifest_and_stream_discriminators(self):
        config = tomllib.loads(
            (codegen.ROOT / "tooling/pipeline/openapi-to-rust.toml").read_text()
        )
        generator = config["generator"]
        self.assertEqual(generator["spec_path"], "../sources/openapi/openapi.yaml")
        self.assertEqual(generator["overlays"], ["mistral.overlay.yaml"])
        self.assertEqual(
            generator["overlay_output"],
            "../sources/openapi/openapi.codegen.json",
        )
        self.assertTrue(generator["binding_manifest"])

        rules = config["client"]["request_discriminators"]
        self.assertEqual(len(rules), 15)
        self.assertEqual(
            {
                (
                    rule["operation"],
                    rule["transport"],
                    rule["media_type"],
                    rule["field"],
                    rule["value"],
                )
                for rule in rules
            },
            {
                (
                    "agents_api_v1_conversations_start",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "agents_api_v1_conversations_start_stream",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "agents_api_v1_conversations_append",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "agents_api_v1_conversations_append_stream",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "agents_api_v1_conversations_restart",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "agents_api_v1_conversations_restart_stream",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "audio_api_v1_transcriptions_post",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "audio_api_v1_transcriptions_post_stream",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "chat_completion_v1_chat_completions_post",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "chat_completion_v1_chat_completions_post",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "fim_completion_v1_fim_completions_post",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "fim_completion_v1_fim_completions_post",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "speech_v1_audio_speech_post",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
                (
                    "speech_v1_audio_speech_post",
                    "event_stream",
                    "text/event-stream",
                    "stream",
                    True,
                ),
                (
                    "agents_completion_v1_agents_completions_post",
                    "buffered",
                    "application/json",
                    "stream",
                    False,
                ),
            },
        )

    def test_generator_pin_uses_unpatched_generic_fork(self):
        lock = json.loads((codegen.ROOT / "tooling/sources/lock.json").read_text())
        self.assertEqual(
            lock["generator_repository"],
            "adriendellagaspera/openapi-to-rust",
        )
        self.assertEqual(lock["generator_version"], "0.17.0")
        self.assertNotIn("generator_patch_sha256", lock)


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
