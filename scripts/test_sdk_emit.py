import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen
import sdk_compiler
import sdk_emit
import sdk_pipeline
from sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, ModelSpec, NoRequest,
                    OperationCall, OperationSpec, RawSignature, ResourceSpec,
                    TypeAliasPolicy)
from sdk_raw_ir import RawIr
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class ResolvedEmissionTests(unittest.TestCase):
    def test_compatibility_module_routes_generation_to_pipeline(self):
        self.assertIs(sdk_codegen.generate, sdk_pipeline.generate)
        self.assertIs(sdk_codegen.GenerationError, sdk_compiler.GenerationError)

    def test_pipeline_passes_raw_ir_to_automatic_projection(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            raw = root / "raw"
            raw.mkdir()
            (raw / "types.rs").write_text(TYPES)
            (raw / "client.rs").write_text(CLIENT)
            (raw / "coverage.json").write_text(json.dumps({"operations": []}))
            openapi = root / "openapi.json"
            overlay = root / "overlay.json"
            taxonomy = root / "taxonomy.json"
            openapi.write_text(json.dumps(openapi_document()))
            overlay.write_text(json.dumps(manifest()))
            taxonomy.write_text(json.dumps({"operations": {}}))
            seen = {}

            def project(openapi_index, current, taxonomy_ir, raw_coverage, raw_ir):
                seen["raw"] = raw_ir
                return current, None

            with patch.object(sdk_pipeline, "expand_manifest", side_effect=project):
                sdk_pipeline.generate(raw, root / "sdk", overlay, openapi, taxonomy)
            self.assertIsInstance(seen["raw"], RawIr)
            self.assertIn("adopt", seen["raw"].operations)

    def test_renderer_emits_hand_built_ir_without_source_contracts(self):
        model = ModelSpec(
            "Cursor", "RawCursor", TypeAliasPolicy(), AliasModelSpec("String"),
        )
        operation = OperationSpec(
            "ping", "ping", "ping", RawSignature((), "Result<(), Error>", "()"),
            NoRequest(), EmptyResponse(), OperationCall("", "", None), None,
        )
        resource = ResourceSpec(("health",), "health", "Health", (operation,))
        files = sdk_emit.emit(FacadeIr("Client", (model,), (resource,)))
        self.assertEqual(set(files), {"facade_types.rs", "health.rs", "mod.rs"})
        self.assertIn("pub type Cursor = String;", files["facade_types.rs"])
        self.assertIn("pub async fn ping(&self) -> Result<(), SdkError>", files["health.rs"])
        self.assertIn("self.raw.ping().await.map_err(Into::into)", files["health.rs"])

    def test_renderer_has_no_source_inspection_dependencies(self):
        source = (ROOT / "codegen/sdk_emit.py").read_text()
        for forbidden in ("OpenApiIndex", "RustIndex", "tree_sitter", "rust_types",
                          "sdk_codegen", "sdk_model_lowering", "sdk_operation_lowering"):
            self.assertNotIn(forbidden, source)

    def test_repository_pipeline_delegates_compilation_to_core(self):
        source = (ROOT / "codegen/sdk_pipeline.py").read_text()
        self.assertIn("sdk_compiler.compile_facade", source)
        self.assertNotIn("sdk_emit", source)
        self.assertNotIn("resolve_models", source)
        self.assertNotIn("resolve_operations", source)
        self.assertNotIn("frontend._emit_model", source)


if __name__ == "__main__":
    unittest.main()
