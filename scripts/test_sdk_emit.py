import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen
import sdk_emit
import sdk_pipeline
from sdk_ir import (EmptyResponse, NoRequest, OperationCall, OperationSpec,
                    RawSignature, ResourceSpec)
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class ResolvedOperationEmissionTests(unittest.TestCase):
    def test_pipeline_matches_legacy_output_byte_for_byte(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            raw = root / "raw"
            raw.mkdir()
            (raw / "types.rs").write_text(TYPES)
            (raw / "client.rs").write_text(CLIENT)
            openapi = root / "openapi.json"
            overlay = root / "overlay.json"
            openapi.write_text(json.dumps(openapi_document()))
            overlay.write_text(json.dumps(manifest()))
            legacy, resolved = root / "legacy", root / "resolved"
            sdk_codegen.generate(raw, legacy, overlay, openapi)
            sdk_pipeline.generate(raw, resolved, overlay, openapi)
            legacy_files = {path.name: path.read_bytes() for path in legacy.iterdir()}
            resolved_files = {path.name: path.read_bytes() for path in resolved.iterdir()}
            self.assertEqual(legacy_files, resolved_files)

    def test_renderer_emits_hand_built_ir_without_source_contracts(self):
        operation = OperationSpec(
            "ping", "ping", "ping", RawSignature((), "Result<(), Error>", "()"),
            NoRequest(), EmptyResponse(), OperationCall("", "", None), None,
        )
        resource = ResourceSpec(("health",), "health", "Health", (operation,))
        source = sdk_emit.emit_resource(resource, (resource,))
        self.assertIn("pub async fn ping(&self) -> Result<(), SdkError>", source)
        self.assertIn("self.raw.ping().await.map_err(Into::into)", source)

    def test_renderer_has_no_source_inspection_dependencies(self):
        source = (ROOT / "codegen/sdk_emit.py").read_text()
        for forbidden in ("OpenApiIndex", "RustIndex", "tree_sitter", "rust_types", "sdk_codegen"):
            self.assertNotIn(forbidden, source)


if __name__ == "__main__":
    unittest.main()
