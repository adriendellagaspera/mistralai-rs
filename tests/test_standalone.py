import importlib.resources
import json
from pathlib import Path
import sys
import tomllib
import unittest

from jsonschema import Draft202012Validator

import openapi_rust_facade as package
from openapi_rust_facade import (
    OpenApiIndex,
    RustBindingsIr,
    RustFacadeRuntime,
    __version__,
    compile_facade,
)

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
FIXTURES = Path(__file__).resolve().parent / "fixtures"


def load_bindings(name: str) -> RustBindingsIr:
    value = json.loads((FIXTURES / name / "rust-bindings.json").read_text())
    schema = json.loads(
        importlib.resources.files(package).joinpath("rust-bindings.schema.json").read_text()
    )
    Draft202012Validator(schema).validate(value)
    return RustBindingsIr.from_dict(value)


def compile_fixture(name: str):
    root = FIXTURES / name
    document = json.loads((root / "openapi.json").read_text())
    policy = json.loads((root / "policy.json").read_text())
    bindings = load_bindings(name)
    ir, files = compile_facade(OpenApiIndex(document), bindings, policy)
    return ir, files, bindings


class StandaloneCompilerTests(unittest.TestCase):
    def test_tests_use_installed_package_not_source_tree(self):
        package_path = Path(package.__file__).resolve()
        self.assertNotEqual(package_path.parent, SRC / "openapi_rust_facade")

    def test_importing_core_does_not_import_adapter_runtime(self):
        self.assertNotIn("tree_sitter", sys.modules)
        self.assertNotIn("tree_sitter_rust", sys.modules)

    def test_menagerie_is_an_independent_fixture(self):
        ir, files, bindings = compile_fixture("menagerie")
        self.assertEqual(ir.client_name, "Menagerie")
        self.assertEqual(set(files), {"facade_types.rs", "zoo.rs", "mod.rs"})
        self.assertIn("pub enum Animal", files["facade_types.rs"])
        self.assertIn("pub fn cat(content: impl Into<String>)", files["facade_types.rs"])
        self.assertIn("pub async fn adopt(&self, request: Adoption)", files["zoo.rs"])
        self.assertEqual(RustBindingsIr.from_dict(bindings.to_dict()), bindings)

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
        self.assertIn("chunk.map_err(Into::into)", resource)

    def test_binding_paths_are_data_not_compiler_constants(self):
        bindings = load_bindings("menagerie")
        self.assertEqual(
            bindings.qualified_type("Vec<AnimalUnion>"),
            "Vec<crate::generated::types::AnimalUnion>",
        )
        custom = bindings.to_dict()
        custom["binding"]["client"]["type_path"] = "crate::wire::Client"
        custom["binding"]["type_preludes"] = ["crate::wire::models::*"]
        custom["symbol_paths"] = {
            name: path.replace("crate::generated::types", "crate::wire::models")
            for name, path in custom["symbol_paths"].items()
        }
        document = json.loads((FIXTURES / "menagerie" / "openapi.json").read_text())
        policy = json.loads((FIXTURES / "menagerie" / "policy.json").read_text())
        _, files = compile_facade(
            OpenApiIndex(document), RustBindingsIr.from_dict(custom), policy
        )
        self.assertIn("use crate::wire::Client;", files["mod.rs"])
        self.assertIn("use crate::wire::models::*;", files["facade_types.rs"])
        self.assertNotIn("crate::generated", "".join(files.values()))

    def test_runtime_support_paths_are_explicit(self):
        bindings = load_bindings("menagerie")
        document = json.loads((FIXTURES / "menagerie" / "openapi.json").read_text())
        policy = json.loads((FIXTURES / "menagerie" / "policy.json").read_text())
        runtime = RustFacadeRuntime(
            error_type="FacadeError",
            error_module="support",
            error_exports=("FacadeError",),
            sse_module="crate::events",
            sse_function="decode_json",
            generated_marker="// generated fixture\n",
        )
        _, files = compile_facade(
            OpenApiIndex(document), bindings, policy, runtime=runtime
        )
        self.assertTrue(files["mod.rs"].startswith("// generated fixture\n"))
        self.assertIn("pub mod support;", files["mod.rs"])
        self.assertIn("pub use support::{FacadeError};", files["mod.rs"])

    def test_scalar_enum_is_lowered_from_wire_provenance(self):
        document = {
            "openapi": "3.1.0",
            "info": {"title": "Enum fixture", "version": "1"},
            "paths": {},
            "components": {
                "schemas": {
                    "ResourceVisibility": {
                        "type": "string",
                        "enum": ["shared_global", "private"],
                    }
                }
            },
        }
        bindings = RustBindingsIr.from_dict(
            {
                "schema_version": 2,
                "structs": {},
                "enums": {
                    "ResourceVisibility": [
                        {
                            "name": "SharedGlobal",
                            "payload": None,
                            "wire_name": "shared_global",
                        },
                        {
                            "name": "Private",
                            "payload": None,
                            "wire_name": "private",
                        },
                    ]
                },
                "aliases": {},
                "operations": {},
                "symbol_paths": {
                    "ResourceVisibility": "crate::raw::ResourceVisibility",
                },
                "binding": {
                    "client": {
                        "type_path": "crate::raw::Client",
                        "constructor": "new",
                        "api_key_builder": "with_api_key",
                        "base_url_builder": "with_base_url",
                    },
                    "type_preludes": ["crate::raw::*"],
                },
            }
        )
        policy = {
            "schema_version": 2,
            "client": {"name": "EnumClient"},
            "models": {
                "ResourceVisibilityValue": {
                    "raw": "ResourceVisibility",
                    "scalar_enum": {"root": "ResourceVisibility", "path": []},
                }
            },
            "resources": {},
        }
        _, files = compile_facade(OpenApiIndex(document), bindings, policy)
        facade = files["facade_types.rs"]
        self.assertIn("pub enum ResourceVisibilityValue", facade)
        self.assertIn("SharedGlobal", facade)
        self.assertIn("Private", facade)

    def test_generation_is_deterministic_for_both_fixtures(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                _, first, _ = compile_fixture(name)
                _, second, _ = compile_fixture(name)
                self.assertEqual(first, second)

    def test_package_metadata_separates_core_and_adapter_dependencies(self):
        project = tomllib.loads((ROOT / "pyproject.toml").read_text())["project"]
        self.assertEqual(project["name"], "openapi-rust-facade")
        self.assertEqual(project["version"], __version__)
        self.assertEqual(
            set(project["dependencies"]),
            {"ruamel.yaml==0.18.6", "jsonschema==4.23.0"},
        )
        self.assertEqual(
            set(project["optional-dependencies"]["openapi-to-rust"]),
            {"tree-sitter==0.25.2", "tree-sitter-rust==0.24.0"},
        )
        compatibility = json.loads((ROOT / "COMPATIBILITY.json").read_text())
        self.assertEqual(compatibility["package_name"], "openapi-rust-facade")
        self.assertEqual(compatibility["package_version"], __version__)
        self.assertEqual(compatibility["rust_bindings_schema_version"], 2)
        self.assertEqual(compatibility["adapter"]["name"], "openapi-to-rust")

    def test_compiler_core_contains_no_backend_layout_or_parser_dependency(self):
        root = SRC / "openapi_rust_facade"
        forbidden = ("crate::generated", "HttpClient", "tree_sitter", "openapi_to_rust")
        for path in root.glob("*.py"):
            source = path.read_text()
            for needle in forbidden:
                self.assertNotIn(needle, source, f"{needle} leaked into {path.name}")

    def test_public_api_uses_rust_bindings_terminology(self):
        self.assertIn("RustBindingsIr", package.__all__)
        self.assertNotIn("RawIr", package.__all__)

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
