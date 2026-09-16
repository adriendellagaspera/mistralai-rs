"""Run repository integration tests with pinned compiler internals isolated locally."""

import importlib
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
sys.path.insert(0, str(ROOT / "scripts"))

# Historical repository tests still import extracted engine modules by their old
# top-level names. Keep that compatibility here, never in rust-sdk-compiler's
# public package API.
for name in (
    "rust_symbols",
    "rust_types",
    "sdk_emit",
    "sdk_frontend",
    "sdk_ir",
    "sdk_model_lowering",
    "sdk_operation_lowering",
    "sdk_raw_ir",
    "sdk_runtime",
):
    sys.modules.setdefault(name, importlib.import_module(f"rust_sdk_compiler.{name}"))

suite = unittest.defaultTestLoader.discover(str(ROOT / "scripts"), pattern="test_*.py")
result = unittest.TextTestRunner(verbosity=1).run(suite)
raise SystemExit(0 if result.wasSuccessful() else 1)
