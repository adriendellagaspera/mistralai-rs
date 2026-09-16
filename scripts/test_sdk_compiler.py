from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

import sdk_codegen as legacy_frontend
import sdk_compiler
import sdk_frontend
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

    def test_extracted_frontend_matches_legacy_frontend_ir(self):
        document = openapi_document()
        policy = manifest()
        extracted = sdk_frontend.build_ir(
            sdk_frontend.OpenApiIndex(document),
            sdk_frontend.RustIndex(TYPES.encode(), CLIENT.encode()),
            policy,
        )
        legacy = legacy_frontend.build_ir(
            legacy_frontend.OpenApiIndex(document),
            legacy_frontend.RustIndex(TYPES.encode(), CLIENT.encode()),
            policy,
        )
        self.assertEqual(extracted, legacy)

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
                "coverage_inventory(",
                "public_surface(",
            ):
                self.assertNotIn(forbidden, source)


if __name__ == "__main__":
    unittest.main()
