"""Pin the repository OpenAPI and independently verify the reviewed public-spec fingerprint."""

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
    mirror = fetch(source["published_url"])
    digest = hashlib.sha256(spec).hexdigest()
    mirror_digest = hashlib.sha256(mirror).hexdigest()
    reviewed_mirror_digest = source.get("published_sha256")
    if not reviewed_mirror_digest:
        raise ValueError("Missing reviewed published OpenAPI SHA-256 fingerprint")
    if mirror_digest != reviewed_mirror_digest:
        raise ValueError(
            "Published OpenAPI changed independently of the pinned repository source; "
            "review the public-spec change before updating published_sha256: "
            f"reviewed_sha256={reviewed_mirror_digest}, actual_sha256={mirror_digest}, "
            f"upstream_commit={commit}, upstream_sha256={digest}, "
            f"upstream_bytes={len(spec)}, published_bytes={len(mirror)}"
        )
    if spec != mirror:
        print(
            "Known public-spec divergence (reviewed fingerprint): "
            f"repository_sha256={digest}, published_sha256={mirror_digest}, "
            f"upstream_commit={commit}; the GitHub repository remains the SDK source."
        )
    if digest == source["sha256"]:
        if (HERE / "published.yaml").read_bytes() != spec:
            raise ValueError("Published OpenAPI checkout differs from its locked SHA-256")
        print("OpenAPI unchanged")
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
    print(f"Pinned OpenAPI {commit} ({digest}); published mirror verified")


if __name__ == "__main__":
    main()
