import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
FIXTURES = Path(__file__).resolve().parent / "fixtures"
sys.path.insert(0, str(SRC))

from openapi_to_rust_facade import RawIr  # noqa: E402
from openapi_to_rust_facade.adapters.openapi_to_rust import (  # noqa: E402
    OpenApiToRustAdapter,
)


class OpenApiToRustAdapterTests(unittest.TestCase):
    def test_adapter_is_exactly_a_source_to_bindings_normalizer(self):
        for name in ("menagerie", "library"):
            with self.subTest(name=name):
                root = FIXTURES / name
                expected = RawIr.from_dict(json.loads((root / "raw-ir.json").read_text()))
                actual = OpenApiToRustAdapter.parse(
                    (root / "types.rs").read_bytes(),
                    (root / "client.rs").read_bytes(),
                )
                self.assertEqual(actual, expected)

    def test_adapter_owns_openapi_to_rust_layout_conventions(self):
        raw = OpenApiToRustAdapter.parse(
            (FIXTURES / "menagerie" / "types.rs").read_bytes(),
            (FIXTURES / "menagerie" / "client.rs").read_bytes(),
        )
        self.assertEqual(raw.binding.client.type_path, "crate::generated::client::HttpClient")
        self.assertEqual(raw.binding.type_preludes, ("crate::generated::types::*",))
        self.assertEqual(
            raw.symbol_paths["AnimalRequest"],
            "crate::generated::types::AnimalRequest",
        )


if __name__ == "__main__":
    unittest.main()
