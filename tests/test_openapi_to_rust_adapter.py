import json
from pathlib import Path
import unittest

from openapi_rust_facade import RustBindingsIr
from openapi_rust_facade.adapters.openapi_to_rust import OpenApiToRustAdapter

FIXTURES = Path(__file__).resolve().parent / "fixtures"


class OpenApiToRustAdapterTests(unittest.TestCase):
    def test_adapter_is_exactly_a_source_to_bindings_normalizer(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                root = FIXTURES / name
                expected = RustBindingsIr.from_dict(
                    json.loads((root / "rust-bindings.json").read_text())
                )
                actual = OpenApiToRustAdapter.parse(
                    (root / "types.rs").read_bytes(),
                    (root / "client.rs").read_bytes(),
                )
                self.assertEqual(actual, expected)

    def test_adapter_owns_openapi_to_rust_layout_conventions(self):
        bindings = OpenApiToRustAdapter.parse(
            (FIXTURES / "menagerie" / "types.rs").read_bytes(),
            (FIXTURES / "menagerie" / "client.rs").read_bytes(),
        )
        self.assertEqual(
            bindings.binding.client.type_path,
            "crate::generated::client::HttpClient",
        )
        self.assertEqual(
            bindings.binding.type_preludes,
            ("crate::generated::types::*",),
        )
        self.assertEqual(
            bindings.symbol_paths["AnimalRequest"],
            "crate::generated::types::AnimalRequest",
        )


if __name__ == "__main__":
    unittest.main()
