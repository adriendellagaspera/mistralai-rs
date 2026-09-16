"""Validate the extracted package and its temporary source-parity invariant."""

from pathlib import Path
import subprocess
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "tools/openapi-to-rust-facade"
PACKAGE_SOURCE = PACKAGE / "src/openapi_to_rust_facade"
CORE_FILES = (
    "rust_symbols.py",
    "rust_types.py",
    "sdk_ir.py",
    "sdk_raw_ir.py",
    "sdk_openapi_to_rust.py",
    "sdk_frontend.py",
    "sdk_model_lowering.py",
    "sdk_operation_lowering.py",
    "sdk_emit.py",
    "sdk_compiler.py",
    "sdk-semantics.schema.json",
    "raw-ir.schema.json",
)


class ExtractedFacadePackageTests(unittest.TestCase):
    def test_standalone_suite(self):
        subprocess.run(
            [
                sys.executable,
                "-m",
                "unittest",
                "discover",
                "-s",
                str(PACKAGE / "tests"),
                "-p",
                "test_*.py",
            ],
            cwd=PACKAGE,
            check=True,
        )

    def test_extracted_core_matches_repository_copy_until_migration(self):
        for name in CORE_FILES:
            with self.subTest(name=name):
                self.assertEqual(
                    (PACKAGE_SOURCE / name).read_bytes(),
                    (ROOT / "codegen" / name).read_bytes(),
                )


if __name__ == "__main__":
    unittest.main()
