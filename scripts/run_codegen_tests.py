"""Run repository-owned code generation tests in the pinned tooling environment."""

from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen" / "mistral"))
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

suite = unittest.defaultTestLoader.discover(
    str(ROOT / "codegen_tests"), pattern="test_*.py"
)
result = unittest.TextTestRunner(verbosity=1).run(suite)
raise SystemExit(0 if result.wasSuccessful() else 1)
