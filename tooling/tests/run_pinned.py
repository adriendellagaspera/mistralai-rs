#!/usr/bin/env python3
"""Run repository tooling tests in the exact pinned pipeline environment."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "pipeline"))

import build  # noqa: E402


def main() -> None:
    lock = json.loads((ROOT / "tooling" / "sources" / "lock.json").read_text())
    python = build.tooling_python(lock)
    build.run(python, ROOT / "tooling" / "tests" / "run.py")


if __name__ == "__main__":
    main()
