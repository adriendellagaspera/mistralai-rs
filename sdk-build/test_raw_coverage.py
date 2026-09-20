"""Raw coverage is regenerated from emitted bindings, not retained from tooling/."""

import json
from pathlib import Path
import tempfile
import unittest

from build import raw_coverage, verify_raw_coverage


class RawCoverageTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.generated = self.root / "generated"
        self.generated.mkdir()
        self.overlaid = self.root / "overlaid.json"

    def fixture(self, *, bound=True):
        self.overlaid.write_text(json.dumps({
            "paths": {"/v1/echo#stream": {"post": {
                "operationId": "echo_stream",
                "tags": ["test"],
                "responses": {"200": {"content": {"text/event-stream": {}}}},
            }}}
        }))
        self.generated.joinpath("binding-manifest.json").write_text(json.dumps({
            "operations": [{
                "kind": "call_shape",
                "source_operation": {
                    "operation_id": "echo_stream", "method": "POST", "path": "/v1/echo"
                },
                "rust_method_name": "echo_stream",
            }] if bound else []
        }))

    def test_synthetic_stream_route_is_normalized_and_covered(self):
        self.fixture()
        result = raw_coverage(self.generated, self.overlaid)
        self.assertEqual(result["generated_methods"], 1)
        self.assertEqual(result["upstream_operations"], 1)
        self.assertEqual(result["operations"], [{
            "method": "POST", "operation_id": "echo_stream", "path": "/v1/echo",
            "success_media": ["text/event-stream"], "tags": ["test"],
            "upstream": True, "rust_method": "echo_stream",
        }])

    def test_missing_generated_method_fails_closed(self):
        self.fixture(bound=False)
        with self.assertRaisesRegex(ValueError, "missing=\\['echo_stream'\\]"):
            raw_coverage(self.generated, self.overlaid)

    def test_committed_coverage_requires_matching_methods_and_paths(self):
        actual = {"operations": [{"operation_id": "echo_stream", "method": "POST", "path": "/v1/echo"}]}
        verify_raw_coverage(actual, actual)
        with self.assertRaisesRegex(ValueError, "operation identities"):
            verify_raw_coverage(actual, {
                "operations": [{"operation_id": "echo_stream", "method": "GET", "path": "/v1/echo"}]
            })
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            verify_raw_coverage({"operations": actual["operations"] * 2}, actual)


if __name__ == "__main__":
    unittest.main()
