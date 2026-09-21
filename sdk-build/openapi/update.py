"""Pin the versioned GitHub OpenAPI source independently of the docs-site catalog."""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import re
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
LOCK = HERE.parent / "provenance.lock.json"


def fetch(url: str, *, github_api: bool = False) -> bytes:
    headers = {
        "User-Agent": "mistralai-rs-sdk-build",
        "Accept": "application/vnd.github+json",
    }
    if github_api and os.environ.get("GH_TOKEN"):
        headers["Authorization"] = "Bearer " + os.environ["GH_TOKEN"]
    with urllib.request.urlopen(
        urllib.request.Request(url, headers=headers), timeout=60
    ) as response:
        return response.read()


def optional_notice(repository: str, commit: str) -> bytes | None:
    try:
        return fetch(f"https://raw.githubusercontent.com/{repository}/{commit}/NOTICE")
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise
        return None


def main() -> None:
    lock = json.loads(LOCK.read_text())
    source = lock["openapi"]
    repository, path = source["repository"], source["path"]
    latest = json.loads(
        fetch(
            f"https://api.github.com/repos/{repository}/commits?path={path}&per_page=1",
            github_api=True,
        )
    )
    commit = latest[0]["sha"]
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError("Invalid upstream OpenAPI commit")
    spec = fetch(f"https://raw.githubusercontent.com/{repository}/{commit}/{path}")
    # docs.mistral.ai/openapi.yaml serves a distinct, broader catalog, not
    # a mirror of platform-docs-public/openapi.yaml (see issue #127).
    # Do not compare their bytes or silently switch the SDK source to it.
    digest = hashlib.sha256(spec).hexdigest()
    if digest == source["sha256"]:
        if (HERE / "published.yaml").read_bytes() != spec:
            raise ValueError("Published OpenAPI checkout differs from its locked SHA-256")
        print("Pinned GitHub OpenAPI unchanged")
        return
    if not spec.startswith(b"openapi: 3.1."):
        raise ValueError("OpenAPI dialect changed; review required")

    license_text = fetch(
        f"https://raw.githubusercontent.com/{repository}/{commit}/LICENSE"
    )
    notice = optional_notice(repository, commit)
    (HERE / "published.yaml").write_bytes(spec)
    (HERE / "LICENSE").write_bytes(license_text)
    notice_path = HERE / "NOTICE"
    if notice is None:
        notice_path.unlink(missing_ok=True)
    else:
        notice_path.write_bytes(notice)
    source.update(commit=commit, sha256=digest)
    LOCK.write_text(json.dumps(lock, indent=2) + "\n")
    print(f"Pinned GitHub OpenAPI {commit} ({digest}); docs-site catalog is independent")


if __name__ == "__main__":
    main()
