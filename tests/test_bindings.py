import json
from pathlib import Path
import tomllib
import unittest

import openapi_to_rust_bindings as package
from openapi_to_rust_bindings import parse_bindings, read_bindings
from rust_sdk_compiler import Bindings

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
FIXTURES = Path(__file__).resolve().parent / "fixtures"


class BindingsPackageTests(unittest.TestCase):
    def test_public_api_is_functional_and_small(self):
        self.assertEqual(
            set(package.__all__),
            {"ParseError", "parse_bindings", "read_bindings"},
        )

    def test_tests_use_installed_package_not_source_tree(self):
        self.assertNotEqual(
            Path(package.__file__).resolve().parent,
            SRC / "openapi_to_rust_bindings",
        )

    def test_fixtures_match_checked_in_bindings_contract(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                root = FIXTURES / name
                expected = Bindings.from_dict(
                    json.loads((root / "rust-bindings.json").read_text())
                )
                actual = parse_bindings(
                    (root / "types.rs").read_bytes(),
                    (root / "client.rs").read_bytes(),
                )
                self.assertIsInstance(actual, Bindings)
                self.assertEqual(actual, expected)
                self.assertEqual(read_bindings(root), expected)

    def test_package_does_not_import_compiler_internals(self):
        source = (SRC / "openapi_to_rust_bindings" / "parser.py").read_text()
        self.assertNotIn("rust_sdk_compiler.sdk_", source)
        self.assertNotIn("RawIr", source)
        self.assertNotIn("Adapter", source)

    def test_metadata_pins_compiler_and_backend_contracts(self):
        project = tomllib.loads((ROOT / "pyproject.toml").read_text())["project"]
        self.assertEqual(project["name"], "openapi-to-rust-bindings")
        self.assertIn("rust-sdk-compiler @ git+https://github.com/adriendellagaspera/mistralai-rs.git@5bda3f6b704edf5b6a9d8788010c4117321ceb5e", project["dependencies"])
        compatibility = json.loads((ROOT / "COMPATIBILITY.json").read_text())
        self.assertEqual(compatibility["bindings_schema_version"], 2)
        self.assertEqual(
            compatibility["compiler"]["commit"],
            "5bda3f6b704edf5b6a9d8788010c4117321ceb5e",
        )
        self.assertEqual(
            compatibility["backend"]["commit"],
            "2af34b86ca9f38c35787f13ec5841989efcf4b99",
        )


if __name__ == "__main__":
    unittest.main()
