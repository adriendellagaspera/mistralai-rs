import copy
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "quality"))
import coverage


def spec():
    return {
        "paths": {
            "/v1/test": {
                "post": {
                    "operationId": "create_test",
                    "tags": ["tests"],
                    "responses": {
                        "200": {
                            "content": {
                                "application/json": {},
                                "text/event-stream": {},
                            }
                        }
                    },
                }
            }
        }
    }


def binding(
    method,
    *,
    kind="call_shape",
    representation=None,
    source_path="/v1/test",
    source_method="POST",
    source_operation="create_test",
):
    return {
        "kind": kind,
        "source_operation": {
            "operation_id": source_operation,
            "method": source_method,
            "path": source_path,
        },
        "rust_method_name": method,
        "representation": representation or {
            "kind": "json",
            "schema_name": "TestResponse",
            "media_type": "application/json",
        },
        "success_statuses": ["200"],
    }


def manifest(*operations):
    return {
        "schema": coverage.MANIFEST_SCHEMA,
        "schema_version": coverage.MANIFEST_VERSION,
        "operations": list(operations),
    }


class CoverageTests(unittest.TestCase):
    def test_inventory_tracks_generator_owned_call_shapes(self):
        document = spec()
        bindings = manifest(
            binding("create_test"),
            binding(
                "create_test_stream",
                representation={
                    "kind": "event_stream",
                    "media_type": "text/event-stream",
                },
            ),
        )
        client = """
        pub async fn create_test(&self) {}
        pub async fn create_test_stream(&self) {}
        """
        report = coverage.inventory(document, document, client, bindings)
        self.assertEqual(report["upstream_operations"], 1)
        self.assertEqual(report["source_operations"], 1)
        self.assertEqual(report["generated_methods"], 2)
        self.assertEqual(
            [row["rust_method"] for row in report["operations"]],
            ["create_test", "create_test_stream"],
        )
        self.assertTrue(all(row["upstream"] for row in report["operations"]))

    def test_inventory_rejects_generated_method_drift_and_runtime_stubs(self):
        document = spec()
        bindings = manifest(binding("create_test"))
        good = "pub async fn create_test(&self) {}"
        coverage.inventory(document, document, good, bindings)
        for broken in [
            "",
            "pub async fn other(&self) {}",
            good + " pub async fn unexpected(&self) {}",
            good + ' HttpError::Config("unsupported".into())',
        ]:
            with self.assertRaises(ValueError):
                coverage.inventory(document, document, broken, bindings)

    def test_inventory_rejects_source_operation_identity_drift(self):
        document = spec()
        bindings = manifest(
            binding("create_test", source_path="/v1/renamed"),
        )
        with self.assertRaisesRegex(ValueError, "source-operation drift"):
            coverage.inventory(
                document,
                document,
                "pub async fn create_test(&self) {}",
                bindings,
            )

    def test_every_overlaid_operation_requires_a_call_shape(self):
        document = spec()
        prepared = copy.deepcopy(document)
        prepared["paths"]["/v1/other"] = {
            "get": {
                "operationId": "get_other",
                "responses": {"204": {}},
            }
        }
        bindings = manifest(binding("create_test"))
        with self.assertRaisesRegex(ValueError, "source-operation drift"):
            coverage.inventory(
                document,
                prepared,
                "pub async fn create_test(&self) {}",
                bindings,
            )

    def test_additive_multipart_helpers_are_manifest_owned(self):
        document = spec()
        bindings = manifest(
            binding("create_test"),
            binding("create_test_with_multipart_filenames", kind="multipart_filenames"),
        )
        client = """
        pub async fn create_test(&self) {}
        pub async fn create_test_with_multipart_filenames(&self) {}
        """
        report = coverage.inventory(document, document, client, bindings)
        self.assertEqual(report["generated_methods"], 2)
        self.assertEqual(
            [row["operation_kind"] for row in report["operations"]],
            ["call_shape", "multipart_filenames"],
        )

    def test_manifest_schema_and_version_are_fail_closed(self):
        document = spec()
        client = "pub async fn create_test(&self) {}"
        for broken in [
            {"schema": "other", "schema_version": 1, "operations": [binding("create_test")]},
            {
                "schema": coverage.MANIFEST_SCHEMA,
                "schema_version": 2,
                "operations": [binding("create_test")],
            },
        ]:
            with self.assertRaises(ValueError):
                coverage.inventory(document, document, client, broken)


if __name__ == "__main__":
    unittest.main()
