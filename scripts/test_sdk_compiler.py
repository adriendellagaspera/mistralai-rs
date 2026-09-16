import json
from pathlib import Path
import sys
import unittest

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen as compatibility
import sdk_compiler
import sdk_frontend
from sdk_openapi_to_rust import OpenApiToRustAdapter
from sdk_raw_ir import RawIr
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class CompilerBoundaryTests(unittest.TestCase):
    def test_menagerie_compiles_from_explicit_policy_without_repository_tooling(self):
        openapi = sdk_compiler.OpenApiIndex(openapi_document())
        raw = OpenApiToRustAdapter.parse(TYPES.encode(), CLIENT.encode())

        ir, files = sdk_compiler.compile_facade(openapi, raw, manifest())

        self.assertEqual(ir.client_name, "Menagerie")
        self.assertEqual(set(files), {"facade_types.rs", "zoo.rs", "mod.rs"})
        self.assertIn("pub enum Animal", files["facade_types.rs"])
        self.assertIn("pub async fn adopt(&self, request: Adoption)", files["zoo.rs"])
        self.assertNotIn("coverage.json", files)
        self.assertNotIn("api-surface.json", files)

    def test_legacy_raw_index_exists_only_on_compatibility_surface(self):
        self.assertIs(compatibility.build_ir, sdk_frontend.build_ir)
        self.assertIs(compatibility.OpenApiIndex, sdk_frontend.OpenApiIndex)
        self.assertFalse(hasattr(sdk_frontend, "RustIndex"))
        self.assertIsInstance(compatibility.RustIndex(TYPES.encode(), CLIENT.encode()), RawIr)

    def test_compiler_dependency_graph_excludes_projection_and_audit_tooling(self):
        sources = [
            (ROOT / "codegen/sdk_compiler.py").read_text(),
            (ROOT / "codegen/sdk_frontend.py").read_text(),
        ]
        for source in sources:
            for forbidden in (
                "import sdk_autoproject",
                "from sdk_autoproject",
                "import sdk_contracts",
                "from sdk_contracts",
                "import sdk_pipeline",
                "from sdk_pipeline",
                "import sdk_codegen",
                "from sdk_codegen",
                "coverage_inventory(",
                "public_surface(",
            ):
                self.assertNotIn(forbidden, source)

    def test_raw_source_parser_is_localized_to_adapter(self):
        frontend = (ROOT / "codegen/sdk_frontend.py").read_text()
        adapter = (ROOT / "codegen/sdk_openapi_to_rust.py").read_text()
        for forbidden in ("tree_sitter", "tree_sitter_rust", "Language(", "Parser(",
                          "OpenApiToRustAdapter"):
            self.assertNotIn(forbidden, frontend)
        self.assertIn("tree_sitter", adapter)
        self.assertIn("Parser(", adapter)


class OpenApiToRustAdapterTests(unittest.TestCase):
    def fixture(self):
        types = TYPES + "\npub type MetadataDict = std::collections::BTreeMap<String, serde_json::Value>;\n"
        client = CLIENT + """
impl HttpClient {
    pub async fn delete_animal(&self, animal_id: impl AsRef<str>) -> Result<(), Error> { todo!() }
    pub async fn download_animal(&self) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
    pub async fn stream_animals(&self, request: AnimalRequest) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
}
"""
        return OpenApiToRustAdapter.parse(types.encode(), client.encode())

    def test_adapter_normalizes_supported_raw_shapes(self):
        raw = self.fixture()

        self.assertEqual([field.name for field in raw.fields("AnimalRequest")], ["animals", "model", "energy"])
        self.assertEqual([(variant.name, variant.payload) for variant in raw.variants("AnimalUnion")],
                         [("Cat", "Cat"), ("Dog", "Dog")])
        self.assertEqual(raw.aliases["MetadataDict"].constructor, "std::collections::BTreeMap")
        self.assertEqual([parameter.type for parameter in raw.operation("adopt").parameters], ["AnimalRequest"])
        self.assertEqual(raw.operation("adopt").success_type, "AnimalResponse")
        self.assertEqual(raw.operation("delete_animal").success_type, "()")
        self.assertEqual(
            raw.operation("download_animal").success_type,
            "futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>",
        )
        self.assertEqual(
            raw.operation("stream_animals").success_type,
            "futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>",
        )
        with self.assertRaises(TypeError):
            raw.structs["Injected"] = ()

    def test_raw_ir_sidecar_round_trip_matches_schema(self):
        raw = self.fixture()
        sidecar = raw.to_dict()
        schema = json.loads((ROOT / "codegen/raw-ir.schema.json").read_text())
        Draft202012Validator(schema).validate(sidecar)
        self.assertEqual(raw, RawIr.from_dict(sidecar))

    def test_raw_ir_sidecar_rejects_unknown_version(self):
        sidecar = self.fixture().to_dict()
        sidecar["schema_version"] = 2
        with self.assertRaisesRegex(ValueError, "sidecar version"):
            RawIr.from_dict(sidecar)


if __name__ == "__main__":
    unittest.main()
