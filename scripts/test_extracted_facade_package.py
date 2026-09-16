"""Run the extracted package's own tests inside the pinned codegen environment."""

import importlib.util
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / "tools/openapi-to-rust-facade"
sys.path.insert(0, str(PACKAGE / "src"))

spec = importlib.util.spec_from_file_location(
    "openapi_to_rust_facade_standalone_tests",
    PACKAGE / "tests/test_standalone.py",
)
if spec is None or spec.loader is None:
    raise RuntimeError("cannot load standalone facade package tests")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

StandalonePackageTests = module.StandalonePackageTests
