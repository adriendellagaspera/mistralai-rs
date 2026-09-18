"""Fail-closed validation of the pinned, unmodified Mistral OpenAPI snapshot."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
PUBLISHED = HERE / "published.yaml"
LOCK = ROOT / "provenance.lock.json"


def main() -> None:
    lock = json.loads(LOCK.read_text())
    source = PUBLISHED.read_bytes()
    actual = hashlib.sha256(source).hexdigest()
    expected = lock["openapi"]["sha256"]
    if actual != expected:
        raise SystemExit(
            f"published OpenAPI SHA-256 mismatch: expected {expected}, got {actual}"
        )
    text = source.decode()
    if not text.startswith("openapi: 3.1."):
        raise SystemExit("published OpenAPI dialect changed")
    assumptions = (
        ("ChatCompletionResponse still requires undeclared data", "- data\n"),
        ("SharingDelete still requires undeclared level", "    SharingDelete:\n"),
        ("WorkflowListResponse still carries beta.workflows typo", "- beta.workflows\n"),
    )
    for description, needle in assumptions:
        if needle not in text:
            raise SystemExit(f"review Overlay: assumption changed: {description}")
    if "#stream:" in text or "#wav:" in text:
        raise SystemExit("published OpenAPI unexpectedly contains synthetic generator-only paths")
    print(f"published OpenAPI verified: {actual}")


if __name__ == "__main__":
    main()
