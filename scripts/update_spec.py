"""Fetch upstream at an immutable commit; no write when spec bytes are unchanged."""

import hashlib
import json
import os
from pathlib import Path
import re
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def fetch(url, *, api=False):
    headers = {"User-Agent": "mistralai-rs-codegen", "Accept": "application/vnd.github+json"}
    # Never forward the GitHub token to raw.githubusercontent.com.
    if api and os.environ.get("GH_TOKEN"):
        headers["Authorization"] = "Bearer " + os.environ["GH_TOKEN"]
    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def updated_lock(old, commit, spec):
    digest = hashlib.sha256(spec).hexdigest()
    if digest == old["spec_sha256"]:
        return None
    return {**old, "upstream_commit": commit, "spec_sha256": digest}


def optional_notice(repository, commit):
    try:
        return fetch(f"https://raw.githubusercontent.com/{repository}/{commit}/NOTICE")
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise
        return None


def main():
    lock_path = ROOT / "codegen.lock"
    old = json.loads(lock_path.read_text())
    repo, path = old["upstream_repository"], old["upstream_spec_path"]
    commits = json.loads(fetch(
        f"https://api.github.com/repos/{repo}/commits?path={path}&per_page=1", api=True))
    commit = commits[0]["sha"]
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError("Invalid upstream commit")
    spec = fetch(f"https://raw.githubusercontent.com/{repo}/{commit}/{path}")
    new = updated_lock(old, commit, spec)
    report = ROOT / "update-report.md"
    report.unlink(missing_ok=True)
    if new is None:
        print("Upstream spec is unchanged; no files updated.")
        return
    if not spec.startswith(b"openapi: 3.1."):
        raise ValueError("Upstream changed OpenAPI dialect; review generator compatibility")
    # Download attribution before changing the checkout. Fail on network errors.
    license_text = fetch(f"https://raw.githubusercontent.com/{repo}/{commit}/LICENSE")
    notice = optional_notice(repo, commit)
    (ROOT / "spec/openapi.yaml").write_bytes(spec)
    (ROOT / "spec/LICENSE").write_bytes(license_text)
    notice_path = ROOT / "spec/NOTICE"
    if notice is None:
        notice_path.unlink(missing_ok=True)
    else:
        notice_path.write_bytes(notice)
    lock_path.write_text(json.dumps(new, indent=2) + "\n")
    report.write_text(
        "Regenerate the unofficial Rust SDK from the official Mistral OpenAPI specification.\n\n"
        f"- Previous commit: `{old['upstream_commit']}`\n"
        f"- New commit: `{commit}`\n"
        f"- Previous spec SHA-256: `{old['spec_sha256']}`\n"
        f"- New spec SHA-256: `{new['spec_sha256']}`\n"
        f"- Generator: `{old['generator']} {old['generator_version']}`\n"
        f"- Rust/rustfmt: `{old['rust_toolchain']}`\n"
        f"- [Upstream changes](https://github.com/{repo}/compare/{old['upstream_commit']}...{commit})\n\n"
        "Review the spec and generated API diff for breaking changes before merging.\n")
    print(f"Updated spec to {commit} ({new['spec_sha256']}).")


if __name__ == "__main__":
    main()
