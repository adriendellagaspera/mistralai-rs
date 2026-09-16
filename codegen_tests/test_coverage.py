import copy
import hashlib
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
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
        "SharingDelete": {"properties": {"share_with_uuid": {}}, "required": ["share_with_uuid", "level"]},
        "WorkflowListResponse": {"properties": {"workflows": {}, "next_cursor": {}}, "required": ["beta.workflows", "next_cursor"]},
    }}}


class CoverageTests(unittest.TestCase):
    def test_preprocessing_is_narrow_and_preserves_original_media(self):
        original = fixture()
        prepared = copy.deepcopy(original)
        preprocess.transform(prepared)
        self.assertEqual(len(prepared["paths"]), len(original["paths"]) + 4)
        for path, operation in original["paths"].items():
            self.assertEqual(prepared["paths"][path], operation)
        self.assertEqual(prepared["paths"]["/v1/chat/completions#stream"]["post"]["responses"]["200"]["content"],
                         {"text/event-stream": {}})
        self.assertEqual(prepared["components"]["schemas"]["WorkflowListResponse"]["required"],
                         ["next_cursor", "workflows"])

    def test_upstream_repairs_and_new_aliases_require_review(self):
        for mutation in [
            lambda s: s["components"]["schemas"]["SharingDelete"]["properties"].update(level={}),
            lambda s: s["paths"].update({"/v1/chat/completions#stream": {}}),
            lambda s: s["paths"]["/v1/chat/completions"]["post"]["responses"]["200"]["content"].pop("text/event-stream"),
        ]:
            spec = fixture()
            mutation(spec)
            with self.assertRaises(ValueError):
                preprocess.transform(spec)

    def test_inventory_rejects_missing_operations_and_runtime_stubs(self):
        spec = {"paths": {"/v1/test": {"get": {"operationId": "get_test"}}}}
        client = "pub async fn get_test(&self) {}"
        self.assertEqual(coverage.inventory(spec, spec, client)["upstream_operations"], 1)
        for broken in ["", "pub async fn other(&self) {}", client + ' HttpError::Config("unsupported".into())']:
            with self.assertRaises(ValueError):
                coverage.inventory(spec, spec, broken)

    def test_generator_patch_is_authenticated(self):
        lock = json.loads((ROOT / "tooling/sources/lock.json").read_text())
        digest = hashlib.sha256((ROOT / "tooling/pipeline/openapi-to-rust.patch").read_bytes()).hexdigest()
        self.assertEqual(digest, lock["generator_patch_sha256"])

    def test_inventory_tracks_exact_binary_stream_companions(self):
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
        report = coverage.inventory(spec, spec, client)
        self.assertEqual(2, report["generated_methods"])
        self.assertEqual("download_file_stream", report["operations"][0]["binary_stream_method"])
        for broken in [
            "pub async fn download_file(&self) {}",
            client + "pub async fn unexpected(&self) {}",
        ]:
            with self.assertRaises(ValueError):
                coverage.inventory(spec, spec, broken)


if __name__ == "__main__":
    unittest.main()
