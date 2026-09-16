from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_compiler
from test_sdk_facade import CLIENT, TYPES, manifest, openapi_document


class CompilerBoundaryTests(unittest.TestCase):
    def test_menagerie_compiles_from_explicit_policy_without_repository_tooling(self):
        openapi = sdk_compiler.OpenApiIndex(openapi_document())
        rust = sdk_compiler.RustIndex(TYPES.encode(), CLIENT.encode())

        ir, files = sdk_compiler.compile_facade(openapi, rust, manifest())

        self.assertEqual(ir.client_name, "Menagerie")
        self.assertEqual(set(files), {"facade_types.rs", "zoo.rs", "mod.rs"})
        self.assertIn("pub enum Animal", files["facade_types.rs"])
        self.assertIn("pub async fn adopt(&self, request: Adoption)", files["zoo.rs"])
        self.assertNotIn("coverage.json", files)
        self.assertNotIn("api-surface.json", files)

    def test_compiler_module_does_not_depend_on_projection_or_audit_tooling(self):
        source = (ROOT / "codegen/sdk_compiler.py").read_text()
        for forbidden in (
            "sdk_autoproject",
            "sdk_contracts",
            "sdk_pipeline",
            "coverage_inventory",
            "public_surface",
            "taxonomy",
        ):
            self.assertNotIn(forbidden, source)


if __name__ == "__main__":
    unittest.main()
