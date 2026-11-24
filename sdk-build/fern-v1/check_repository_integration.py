#!/usr/bin/env python3
"""Assemble the Fern v1 candidate with this repository's tests and manifest."""

from __future__ import annotations

import re
import shutil
import sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()
generated = Path(sys.argv[2]).resolve()
workspace = Path(sys.argv[3]).resolve()

if workspace.exists():
    shutil.rmtree(workspace)
workspace.mkdir(parents=True)

shutil.copytree(generated / "src", workspace / "src")
shutil.copytree(root / "tests", workspace / "tests")

manifest = (generated / "Cargo.toml").read_text()
manifest = re.sub(r'(?m)^name = "mistralai_sdk"$', 'name = "mistralai-sdk"', manifest, count=1)
manifest = manifest.replace(
    'description = "Unofficial asynchronous Rust SDK for Mistral AI"',
    'description = "Unofficial asynchronous Rust SDK for Mistral AI generated reproducibly with Fern"',
)
manifest = manifest.replace(
    'repository = "https://github.com/adriendellagaspera/mistralai-rs"',
    'repository = "https://github.com/adriendellagaspera/mistralai-rs"\\nlicense = "MIT OR Apache-2.0"\\npublish = ["crates-io"]',
)
manifest += """

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["multipart"] }
futures = "0.3"
"""
(workspace / "Cargo.toml").write_text(manifest)
print(f"Prepared Fern v1 repository integration workspace at {workspace}")
