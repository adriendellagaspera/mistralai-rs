"""Run the extracted package tests in a fresh interpreter."""

from pathlib import Path
import subprocess
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "tools/openapi-to-rust-facade"


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


if __name__ == "__main__":
    unittest.main()
