import json
from pathlib import Path
import sys
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
FIXTURES = Path(__file__).resolve().parent / "fixtures"
sys.path.insert(0, str(SRC))

from openapi_to_rust_facade import (  # noqa: E402
    OpenApiIndex,
    OpenApiToRustAdapter,
    RawIr,
    __version__,
    compile_facade,
)


def compile_fixture(name: str):
    root = FIXTURES / name
    document = json.loads((root / "openapi.json").read_text())
    policy = json.loads((root / "policy.json").read_text())
    raw = OpenApiToRustAdapter.parse(
        (root / "types.rs").read_bytes(),
        (root / "client.rs").read_bytes(),
    )
    ir, files = compile_facade(OpenApiIndex(document), raw, policy)
    return ir, files, raw


class StandalonePackageTests(unittest.TestCase):
    def test_menagerie_is_an_independent_fixture(self):
        ir, files, raw = compile_fixture("menagerie")
        self.assertEqual(ir.client_name, "Menagerie")
        self.assertEqual(set(files), {"facade_types.rs", "zoo.rs", "mod.rs"})
        self.assertIn("pub enum Animal", files["facade_types.rs"])
        self.assertIn("pub fn cat(content: impl Into<String>)", files["facade_types.rs"])
        self.assertIn("pub async fn adopt(&self, request: Adoption)", files["zoo.rs"])
        self.assertEqual(RawIr.from_dict(raw.to_dict()), raw)

    def test_library_exercises_hierarchy_and_multiple_transport_primitives(self):
        ir, files, _ = compile_fixture("library")
        self.assertEqual(ir.client_name, "LibraryClient")
        self.assertEqual(
            set(files),
            {"facade_types.rs", "catalog.rs", "catalog_books.rs", "mod.rs"},
        )
        self.assertIn("pub fn catalog(&self) -> Catalog<'_>", files["mod.rs"])
        self.assertIn("pub fn books(&self) -> CatalogBooks<'a>", files["catalog.rs"])
        resource = files["catalog_books.rs"]
        self.assertIn("pub async fn create(&self, request: NewBook)", resource)
        self.assertIn("pub async fn list(&self)", resource)
        self.assertIn(
            "pub async fn list_with(&self, request: ListCatalogBooksRequest)", resource
        )
        self.assertIn("pub fn limit(mut self, limit: i64)", resource)
        self.assertIn(
            "pub async fn delete(&self, book_id: impl AsRef<str>) -> Result<(), SdkError>",
            resource,
        )
        self.assertIn(
            "pub async fn download(&self, book_id: impl AsRef<str>) -> Result<BinaryStream, SdkError>",
            resource,
        )

    def test_generation_is_deterministic_for_both_fixtures(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                _, first, _ = compile_fixture(name)
                _, second, _ = compile_fixture(name)
                self.assertEqual(first, second)

    def test_package_metadata_pins_runtime_and_backend_contracts(self):
        project = tomllib.loads((ROOT / "pyproject.toml").read_text())["project"]
        self.assertEqual(project["version"], __version__)
        self.assertEqual(
            set(project["dependencies"]),
            {
                "ruamel.yaml==0.18.6",
                "tree-sitter==0.25.2",
                "tree-sitter-rust==0.24.0",
                "jsonschema==4.23.0",
            },
        )
        compatibility = json.loads((ROOT / "COMPATIBILITY.json").read_text())
        self.assertEqual(compatibility["package_version"], __version__)
        self.assertEqual(compatibility["raw_ir_schema_version"], 1)
        self.assertEqual(compatibility["backend"]["name"], "openapi-to-rust")
        self.assertEqual(compatibility["backend"]["version"], "0.16.0")
        self.assertEqual(
            compatibility["backend"]["commit"],
            "2af34b86ca9f38c35787f13ec5841989efcf4b99",
        )

    def test_generic_package_and_fixtures_contain_no_product_backend(self):
        forbidden = ("mistralai", "ChatCompletionRequest", "OCRRequest")
        for root in (SRC, FIXTURES):
            for path in root.rglob("*"):
                if not path.is_file() or path.suffix not in {".py", ".json", ".rs"}:
                    continue
                source = path.read_text()
                for needle in forbidden:
                    self.assertNotIn(needle, source, f"{needle} leaked into {path}")


if __name__ == "__main__":
    unittest.main()
