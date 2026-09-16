"""Run repository-owned code generation tests in the pinned tooling environment."""

from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "pipeline" / "mistral"))
sys.path.insert(0, str(ROOT / "tooling" / "pipeline"))
sys.path.insert(0, str(ROOT / "tooling" / "sources"))
sys.path.insert(0, str(ROOT / "tooling" / "quality"))

suite = unittest.defaultTestLoader.discover(
    str(ROOT / "tooling/tests"), pattern="test_*.py"
)
result = unittest.TextTestRunner(verbosity=1).run(suite)
raise SystemExit(0 if result.wasSuccessful() else 1)
