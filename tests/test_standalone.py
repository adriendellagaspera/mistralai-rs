import importlib.resources
import json
from pathlib import Path
import sys
import tomllib
import unittest

from jsonschema import Draft202012Validator

import rust_sdk_compiler as package
from rust_sdk_compiler import Bindings, OpenApi, Policy, Runtime, compile, lower
from rust_sdk_compiler.rust_types import parse_type

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
FIXTURES = Path(__file__).resolve().parent / "fixtures"


def load_bindings(name: str) -> Bindings:
    value = json.loads((FIXTURES / name / "rust-bindings.json").read_text())
    schema = json.loads(
        importlib.resources.files(package).joinpath("rust-bindings.schema.json").read_text()
    )
    Draft202012Validator(schema).validate(value)
    bindings = Bindings.from_dict(value)
    assert isinstance(bindings, Bindings)
    return bindings


def compile_fixture(name: str, *, runtime: Runtime | None = None):
    root = FIXTURES / name
    openapi = OpenApi(json.loads((root / "openapi.json").read_text()))
    policy = Policy.from_dict(json.loads((root / "policy.json").read_text()))
    bindings = load_bindings(name)
    kwargs = {} if runtime is None else {"runtime": runtime}
    return compile(openapi, bindings, policy, **kwargs), bindings


class StandaloneCompilerTests(unittest.TestCase):
    def test_public_api_is_small(self):
        self.assertEqual(
            set(package.__all__),
            {
                "Bindings",
                "Compilation",
                "GenerationError",
                "OpenApi",
                "Policy",
                "Runtime",
                "compile",
                "lower",
            },
        )

    def test_tests_use_installed_package_not_source_tree(self):
        self.assertNotEqual(
            Path(package.__file__).resolve().parent,
            SRC / "rust_sdk_compiler",
        )

    def test_core_has_no_backend_parser_dependency(self):
        self.assertNotIn("tree_sitter", sys.modules)
        self.assertNotIn("tree_sitter_rust", sys.modules)
        root = SRC / "rust_sdk_compiler"
        forbidden = ("crate::generated", "HttpClient", "tree_sitter", "openapi_to_rust")
        for path in root.glob("*.py"):
            source = path.read_text()
            for needle in forbidden:
                self.assertNotIn(needle, source, f"{needle} leaked into {path.name}")
        self.assertFalse((root / "adapters").exists())

    def test_structural_type_parser_rejects_statement_suffixes(self):
        parsed = parse_type("Option < Vec < Result<String, Error> > >")
        self.assertEqual(parsed.unary("Option").unary("Vec").constructor, "Result")
        self.assertEqual(parse_type("[u8; 32]").spelling, "[u8; 32]")
        with self.assertRaises(ValueError):
            parse_type("Option<String> ; fn injected() {}")

    def test_menagerie_compiles_through_public_api(self):
        compilation, bindings = compile_fixture("menagerie")
        self.assertEqual(compilation.ir.client_name, "Menagerie")
        self.assertEqual(set(compilation.files), {"facade_types.rs", "zoo.rs", "mod.rs"})
        self.assertIn("pub enum Animal", compilation.files["facade_types.rs"])
        self.assertIn("pub async fn adopt(&self, request: Adoption)", compilation.files["zoo.rs"])
        self.assertEqual(Bindings.from_dict(bindings.to_dict()), bindings)

    def test_library_exercises_transport_primitives(self):
        compilation, _ = compile_fixture("library")
        self.assertEqual(compilation.ir.client_name, "LibraryClient")
        resource = compilation.files["catalog_books.rs"]
        self.assertIn("pub async fn create(&self, request: NewBook)", resource)
        self.assertIn("pub async fn delete(&self, book_id: impl AsRef<str>)", resource)
        self.assertIn("pub async fn download(&self, book_id: impl AsRef<str>)", resource)
        self.assertIn("chunk.map_err(Into::into)", resource)

    def test_runtime_is_explicit(self):
        runtime = Runtime(
            error_type="FacadeError",
            error_module="support",
            error_exports=("FacadeError",),
            sse_module="crate::events",
            sse_function="decode_json",
            generated_marker="// generated fixture\n",
        )
        compilation, _ = compile_fixture("menagerie", runtime=runtime)
        self.assertTrue(compilation.files["mod.rs"].startswith("// generated fixture\n"))
        self.assertIn("pub mod support;", compilation.files["mod.rs"])

    def test_lower_stops_before_emission(self):
        root = FIXTURES / "menagerie"
        openapi = OpenApi(json.loads((root / "openapi.json").read_text()))
        policy = Policy.from_dict(json.loads((root / "policy.json").read_text()))
        ir = lower(openapi, load_bindings("menagerie"), policy)
        self.assertEqual(ir.client_name, "Menagerie")

    def test_generation_is_deterministic(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                first, _ = compile_fixture(name)
                second, _ = compile_fixture(name)
                self.assertEqual(dict(first.files), dict(second.files))

    def test_package_metadata_has_only_core_dependencies(self):
        project = tomllib.loads((ROOT / "pyproject.toml").read_text())["project"]
        self.assertEqual(project["name"], "rust-sdk-compiler")
        self.assertEqual(project["version"], package.__version__)
        self.assertEqual(
            set(project["dependencies"]),
            {"ruamel.yaml==0.18.6", "jsonschema==4.23.0"},
        )
        self.assertNotIn("optional-dependencies", project)
        compatibility = json.loads((ROOT / "COMPATIBILITY.json").read_text())
        self.assertEqual(compatibility["package_name"], "rust-sdk-compiler")
        self.assertEqual(compatibility["bindings_schema_version"], 2)


if __name__ == "__main__":
    unittest.main()
