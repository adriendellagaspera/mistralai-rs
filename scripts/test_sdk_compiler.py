from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen as compatibility
import sdk_compiler
import sdk_frontend
from sdk_openapi_to_rust import OpenApiToRustAdapter
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

    def test_compatibility_frontend_reexports_extracted_frontend(self):
        self.assertIs(compatibility.build_ir, sdk_frontend.build_ir)
        self.assertIs(compatibility.OpenApiIndex, sdk_frontend.OpenApiIndex)
        self.assertIs(compatibility.RustIndex, sdk_frontend.RustIndex)

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


class OpenApiToRustAdapterTests(unittest.TestCase):
    def test_adapter_normalizes_supported_raw_shapes(self):
        types = TYPES + "\npub type MetadataDict = std::collections::BTreeMap<String, serde_json::Value>;\n"
        client = CLIENT + """
impl HttpClient {
    pub async fn delete_animal(&self, animal_id: impl AsRef<str>) -> Result<(), Error> { todo!() }
    pub async fn download_animal(&self) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
    pub async fn stream_animals(&self, request: AnimalRequest) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
}
"""
        raw = OpenApiToRustAdapter.parse(types.encode(), client.encode())

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


if __name__ == "__main__":
    unittest.main()
