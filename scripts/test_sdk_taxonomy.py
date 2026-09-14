import json
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import sdk_taxonomy


class TaxonomyTests(unittest.TestCase):
    def test_reconcile_uses_operation_id_and_http_stream_fallback(self):
        raw = [
            {"operation_id": "chat_complete", "method": "POST", "path": "/v1/chat", "upstream": True},
            {"operation_id": "chat_complete_stream", "method": "POST", "path": "/v1/chat", "upstream": False},
        ]
        records = [
            {
                "operation_id": "chat_complete",
                "http_method": "POST",
                "http_path": "/v1/chat",
                "transport_variant": "default",
                "public_path": "chat.complete",
                "normalized_public_path": "chat.complete",
            },
            {
                "operation_id": "stream_chat",
                "http_method": "POST",
                "http_path": "/v1/chat",
                "transport_variant": "stream",
                "public_path": "chat.stream",
                "normalized_public_path": "chat.stream",
            },
        ]
        matched, unresolved = sdk_taxonomy.reconcile("test", records, raw)
        self.assertEqual([], unresolved)
        self.assertEqual("operation_id", matched["chat_complete"][0]["match_kind"])
        self.assertEqual("http", matched["chat_complete_stream"][0]["match_kind"])

    def test_reconcile_fails_on_operation_id_route_disagreement(self):
        raw = [{"operation_id": "list", "method": "GET", "path": "/v1/files"}]
        record = {
            "operation_id": "list",
            "http_method": "POST",
            "http_path": "/v1/files",
            "transport_variant": "default",
            "public_path": "files.list",
            "normalized_public_path": "files.list",
        }
        with self.assertRaisesRegex(ValueError, "disagrees"):
            sdk_taxonomy.reconcile("test", [record], raw)

    def test_unmatched_and_ambiguous_methods_are_explicit(self):
        raw = [
            {"operation_id": "one", "method": "POST", "path": "/v1/shared"},
            {"operation_id": "two", "method": "POST", "path": "/v1/shared"},
        ]
        records = [{
            "operation_id": "unknown",
            "http_method": "POST",
            "http_path": "/v1/shared",
            "transport_variant": "default",
            "public_path": "shared.call",
            "normalized_public_path": "shared.call",
        }]
        matched, unresolved = sdk_taxonomy.reconcile("test", records, raw)
        self.assertEqual({}, matched)
        self.assertEqual("ambiguous_http_match", unresolved[0]["reason"])
        self.assertEqual(["one", "two"], unresolved[0]["candidate_operation_ids"])

    def test_cross_sdk_naming_drift_is_reported(self):
        lock = {
            "upstream_repository": "mistralai/spec",
            "upstream_commit": "a" * 40,
            "official_typescript_sdk_repository": "mistralai/client-ts",
            "official_typescript_sdk_commit": "b" * 40,
            "official_python_sdk_repository": "mistralai/client-python",
            "official_python_sdk_commit": "c" * 40,
        }
        coverage = {"operations": [{
            "operation_id": "files_list",
            "method": "GET",
            "path": "/v1/files",
            "upstream": True,
        }]}
        common = {
            "operation_id": "files_list",
            "http_method": "GET",
            "http_path": "/v1/files",
            "transport_variant": "default",
        }
        inventory = sdk_taxonomy.build_inventory(
            lock,
            coverage,
            [{**common, "public_path": "files.list", "normalized_public_path": "files.list"}],
            [{**common, "public_path": "files.all", "normalized_public_path": "files.all"}],
        )
        self.assertEqual(1, inventory["summary"]["cross_sdk_divergences"])
        self.assertEqual("public_path_mismatch", inventory["cross_sdk_divergences"][0]["reason"])

    def test_typescript_extraction_follows_resource_graph(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "src/sdk").mkdir(parents=True)
            (root / "src/funcs").mkdir()
            (root / "src/sdk/sdk.ts").write_text(
                'import { Files } from "./files.js";\n'
                'export class Mistral extends ClientSDK {\n'
                '  get files(): Files { return new Files(); }\n'
                '}\n'
            )
            (root / "src/sdk/files.ts").write_text(
                'import { filesList } from "../funcs/filesList.js";\n'
                'export class Files extends ClientSDK {\n'
                '  async list(): Promise<void> { return filesList(this); }\n'
                '}\n'
            )
            (root / "src/funcs/filesList.ts").write_text(
                'export function filesList() {\n'
                ' const path = pathToFunc("/v1/files")();\n'
                ' const context = { operationID: "files_list" };\n'
                ' return request({ method: "GET", path });\n'
                '}\n'
            )
            records = sdk_taxonomy.parse_typescript(root)
            self.assertEqual("files.list", records[0]["public_path"])
            self.assertEqual("files_list", records[0]["operation_id"])


if __name__ == "__main__":
    unittest.main()
