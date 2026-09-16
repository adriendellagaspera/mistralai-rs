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
import sdk_model_lowering
import sdk_pipeline
from sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, MapModelSpec, MapPolicy, ModelSpec, NoRequest,
                    OperationCall, OperationSpec, RawSignature, ResourceSpec,
                    TypeAliasPolicy)
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class ResolvedEmissionTests(unittest.TestCase):
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

    def test_pipeline_passes_raw_index_to_automatic_projection(self):
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

            def project(openapi_index, current, taxonomy_ir, raw_coverage, rust_index):
                seen["rust"] = rust_index
                return current, None

            with patch.object(sdk_pipeline, "expand_manifest", side_effect=project):
                sdk_pipeline.generate(raw, root / "sdk", overlay, openapi, taxonomy)
            self.assertIsInstance(seen["rust"], sdk_compiler.RustIndex)
            self.assertIn("adopt", seen["rust"].operations)

    def test_map_policy_lowers_to_source_agnostic_render_spec(self):
        rust = sdk_codegen.RustIndex(
            b"pub struct RawMetadata { pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>, }",
            b"impl HttpClient {}",
        )
        openapi = sdk_codegen.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"metadata": {"type": "object", "additionalProperties": True}},
            }}},
        })
        model = ModelSpec("Metadata", "RawMetadata", MapPolicy("Root", ("metadata",)))
        resolved = sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)
        spec = resolved.models[0].render
        self.assertIsInstance(spec, MapModelSpec)
        self.assertEqual(
            "std::collections::BTreeMap<String, serde_json::Value>", spec.public_type
        )
        source = sdk_emit.emit_model(resolved.models[0])
        self.assertIn("pub struct Metadata { values: std::collections::BTreeMap<String, serde_json::Value> }", source)
        self.assertIn("value.additional_properties", source)

    def test_map_policy_fails_closed_on_raw_shape_drift(self):
        rust = sdk_codegen.RustIndex(
            b"pub struct RawMetadata { pub values: std::collections::BTreeMap<String, serde_json::Value>, }",
            b"impl HttpClient {}",
        )
        openapi = sdk_codegen.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"metadata": {"type": "object", "additionalProperties": True}},
            }}},
        })
        model = ModelSpec("Metadata", "RawMetadata", MapPolicy("Root", ("metadata",)))
        with self.assertRaisesRegex(
            sdk_model_lowering.ModelLoweringError, "must contain only additional_properties"
        ):
            sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)

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
