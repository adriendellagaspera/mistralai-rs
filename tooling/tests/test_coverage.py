import copy
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "quality"))
sys.path.insert(0, str(ROOT / "tooling" / "pipeline"))
import coverage
import preprocess


def fixture():
    paths = {}
    for path in ["/v1/chat/completions", "/v1/fim/completions", "/v1/audio/speech"]:
        paths[path] = {"post": {"operationId": path.split("/")[2], "responses": {
            "200": {"content": {"application/json": {}, "text/event-stream": {}}}}}}
    paths["/v1/audio/voices/{voice_id}/sample"] = {"get": {
        "operationId": "sample", "responses": {"200": {"content": {
            "application/json": {}, "audio/wav": {}}}}}}
    return {"paths": paths, "components": {"schemas": {
        "ChatCompletionResponse": {"allOf": [
            {"$ref": "#/components/schemas/ChatCompletionResponseBase"},
            {"type": "object", "properties": {"choices": {}}, "required": ["id", "data", "choices"]},
        ]},
        "SharingDelete": {"properties": {"share_with_uuid": {}}, "required": ["share_with_uuid", "level"]},
        "WorkflowListResponse": {"properties": {"workflows": {}, "next_cursor": {}}, "required": ["beta.workflows", "next_cursor"]},
    }}}


class CoverageTests(unittest.TestCase):
    def test_published_validation_is_narrow_and_preserves_input(self):
        original = fixture()
        published = copy.deepcopy(original)
        preprocess.assert_overlay_assumptions(published)
        self.assertEqual(published, original)
        self.assertEqual(len(published["paths"]), len(original["paths"]))
        self.assertFalse(any("#" in path for path in published["paths"]))

    def test_overlay_assumption_drift_requires_review(self):
        for mutation in [
            lambda s: s["components"]["schemas"]["SharingDelete"]["properties"].update(level={}),
            lambda s: s["components"]["schemas"]["SharingDelete"]["required"].remove("level"),
            lambda s: s["components"]["schemas"]["WorkflowListResponse"]["required"].remove("beta.workflows"),
            lambda s: s["components"]["schemas"]["ChatCompletionResponse"]["allOf"][1]["required"].remove("data"),
        ]:
            spec = fixture()
            mutation(spec)
            with self.assertRaises(ValueError):
                preprocess.assert_overlay_assumptions(spec)

    def test_inventory_rejects_missing_operations_and_runtime_stubs(self):
        spec = {"paths": {"/v1/test": {"get": {"operationId": "get_test"}}}}
        client = "pub async fn get_test(&self) {}"
        self.assertEqual(coverage.inventory(spec, spec, client)["upstream_operations"], 1)
        for broken in ["", "pub async fn other(&self) {}", client + ' HttpError::Config("unsupported".into())']:
            with self.assertRaises(ValueError):
                coverage.inventory(spec, spec, broken)

    def test_inventory_uses_exact_generator_manifest_method_set(self):
        spec = {"paths": {"/v1/test": {"get": {"operationId": "get_test"}}}}
        client = """
        pub async fn get_test(&self) {}
        pub async fn generated_helper(&self) {}
        """
        expected = {"get_test", "generated_helper"}
        report = coverage.inventory(spec, spec, client, expected)
        self.assertEqual(2, report["generated_methods"])
        with self.assertRaises(ValueError):
            coverage.inventory(spec, spec, client)
        with self.assertRaises(ValueError):
            coverage.inventory(spec, spec, client, expected | {"missing_helper"})

    def test_manifest_method_inventory_is_fail_closed(self):
        manifest = {
            "operations": [
                {"rust_method_name": "get_test"},
                {"rust_method_name": "generated_helper"},
            ]
        }
        self.assertEqual(
            coverage.manifest_methods(manifest),
            {"get_test", "generated_helper"},
        )
        for invalid in [
            {},
            {"operations": []},
            {"operations": [None]},
            {"operations": [{}]},
            {
                "operations": [
                    {"rust_method_name": "duplicate"},
                    {"rust_method_name": "duplicate"},
                ]
            },
        ]:
            with self.assertRaises(ValueError):
                coverage.manifest_methods(invalid)

    def test_manifest_binary_stream_identity_is_explicit_and_fail_closed(self):
        manifest = {
            "operations": [
                {
                    "kind": "call_shape",
                    "rust_method_name": "download_file_stream",
                    "source_operation": {
                        "method": "GET",
                        "path": "/v1/files/{file_id}/content",
                        "operation_id": "download_file",
                    },
                    "representation": {
                        "kind": "binary_stream",
                        "media_type": "application/octet-stream",
                        "wildcard": False,
                    },
                },
                {
                    "kind": "call_shape",
                    "rust_method_name": "download_file",
                    "source_operation": {
                        "method": "GET",
                        "path": "/v1/files/{file_id}/content",
                        "operation_id": "download_file",
                    },
                    "representation": {
                        "kind": "binary_buffered",
                        "media_type": "application/octet-stream",
                        "wildcard": False,
                    },
                },
            ]
        }
        key = ("GET", "/v1/files/{file_id}/content", "download_file")
        self.assertEqual(
            coverage.manifest_binary_streams(manifest),
            {key: "download_file_stream"},
        )
        duplicate = copy.deepcopy(manifest)
        duplicate["operations"].append(copy.deepcopy(duplicate["operations"][0]))
        with self.assertRaisesRegex(ValueError, "multiple binary streams"):
            coverage.manifest_binary_streams(duplicate)

    def test_generator_is_pinned_without_local_patch(self):
        lock = json.loads((ROOT / "tooling/sources/lock.json").read_text())
        self.assertNotIn("generator_patch_sha256", lock)
        self.assertFalse((ROOT / "tooling/pipeline/openapi-to-rust.patch").exists())
        self.assertEqual(lock["generator_repository"], "adriendellagaspera/openapi-to-rust")
        self.assertRegex(lock["generator_commit"], r"^[0-9a-f]{40}$")

    def test_inventory_tracks_manifest_owned_binary_stream_companions(self):
        spec = {"paths": {"/v1/files/{file_id}/content": {"get": {
            "operationId": "download_file",
            "responses": {"200": {"content": {"application/octet-stream": {
                "schema": {"type": "string", "format": "binary"}
            }}}},
        }}}}
        client = """
        pub async fn download_file(&self) {}
        pub async fn download_file_stream(&self) {}
        """
        expected = {"download_file", "download_file_stream"}
        streams = {
            ("GET", "/v1/files/{file_id}/content", "download_file"):
                "download_file_stream"
        }
        report = coverage.inventory(spec, spec, client, expected, streams)
        self.assertEqual(2, report["generated_methods"])
        self.assertEqual("download_file_stream", report["operations"][0]["binary_stream_method"])
        for broken in [
            "pub async fn download_file(&self) {}",
            client + "pub async fn unexpected(&self) {}",
        ]:
            with self.assertRaises(ValueError):
                coverage.inventory(spec, spec, broken, expected, streams)


if __name__ == "__main__":
    unittest.main()
