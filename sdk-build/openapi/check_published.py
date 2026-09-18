"""Fail-closed validation of the pinned, unmodified Mistral OpenAPI snapshot."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PUBLISHED = HERE / "published.yaml"
LOCK = HERE.parent / "provenance.lock.json"


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
    for path in (
        "/v1/chat/completions#stream:",
        "/v1/fim/completions#stream:",
        "/v1/audio/speech#stream:",
        "/v1/audio/voices/{voice_id}/sample#wav:",
    ):
        if f"  {path}" in text:
            raise SystemExit(f"published OpenAPI now contains retired local path {path[:-1]}")
    print(f"published OpenAPI verified: {actual}")


if __name__ == "__main__":
    main()
