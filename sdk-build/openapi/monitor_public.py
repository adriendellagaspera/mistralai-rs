"""Monitor the deployed public API catalog independently of SDK source pinning.

A changed catalog requires review, but cannot block the pinned SDK source updater.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from urllib.request import urlopen


LOCK = Path(__file__).with_name("public-catalog.lock.json")
PUBLIC_URL = "https://docs.mistral.ai/openapi.yaml"


def verify_public(document: bytes, reviewed_sha256: str) -> str:
    """Return the observed fingerprint, rejecting unreviewed or missing content."""
    if not document:
        raise ValueError("Published OpenAPI document is empty")
    observed = hashlib.sha256(document).hexdigest()
    if observed != reviewed_sha256:
        raise ValueError(
            "Published OpenAPI changed independently of the SDK's pinned source; "
            "review the catalog before updating public-catalog.lock.json: "
            f"reviewed_sha256={reviewed_sha256}, observed_sha256={observed}, "
            f"observed_bytes={len(document)}"
        )
    return observed


def main() -> None:
    reviewed = json.loads(LOCK.read_text())
    if reviewed["url"] != PUBLIC_URL:
        raise ValueError("Unexpected public OpenAPI URL in monitoring lock")
    with urlopen(PUBLIC_URL, timeout=30) as response:
        document = response.read()
    observed = verify_public(document, reviewed["sha256"])
    print(
        f"Independently published OpenAPI unchanged: sha256={observed}, "
        f"bytes={len(document)}, reviewed_at={reviewed['reviewed_at']}"
    )


if __name__ == "__main__":
    main()
