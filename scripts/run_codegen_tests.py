"""Run repository integration tests with the pinned facade package bootstrapped."""

from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

# Registers the historical compiler module names for compatibility tests while
# the implementation itself lives only in the pinned external package.
import openapi_to_rust_facade  # noqa: F401,E402

suite = unittest.defaultTestLoader.discover(str(ROOT / "scripts"), pattern="test_*.py")
result = unittest.TextTestRunner(verbosity=1).run(suite)
raise SystemExit(0 if result.wasSuccessful() else 1)
