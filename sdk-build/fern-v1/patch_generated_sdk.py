#!/usr/bin/env python3
"""Apply strict, reproducible workarounds for known Fern Rust v1 defects.

This operates only on freshly generated Rust sources. Every expected edit is
counted and asserted so generator output drift fails loudly rather than being
silently rewritten incorrectly.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

FIXED_STRING_TYPES = {
    "GetDeploymentSummariesResponseVespaDeploymentType": ("Vespa", "vespa"),
    "AssistantMessageRole": ("Assistant", "assistant"),
    "OAuth2ClientCredentialsAuthMethodGrantType": ("ClientCredentials", "client_credentials"),
    "FunctionToolType": ("Function", "function"),
    "OcrCodeBlockType": ("Code", "code"),
}
MISSING_TYPE_NAMES = tuple(FIXED_STRING_TYPES)


def replace_exact(source: str, old: str, new: str, expected: int, label: str) -> str:
    actual = source.count(old)
    if actual != expected:
        raise ValueError(f"{label}: expected {expected} occurrence(s), found {actual}")
    return source.replace(old, new)


def patch_generated_sdk(root: Path) -> dict[str, int]:
    if not root.is_dir():
        raise ValueError(f"Generated Rust source directory does not exist: {root}")
    rust_files = sorted(root.rglob("*.rs"))
    if not rust_files:
        raise ValueError(f"No generated Rust files found under {root}")

    references = {name: 0 for name in MISSING_TYPE_NAMES}
    for path in rust_files:
        source = path.read_text()
        for name in MISSING_TYPE_NAMES:
            references[name] += source.count(name)
    expected_references = {
        "GetDeploymentSummariesResponseVespaDeploymentType": 3,
        "AssistantMessageRole": 9,
        "OAuth2ClientCredentialsAuthMethodGrantType": 3,
        "FunctionToolType": 9,
        "OcrCodeBlockType": 6,
    }
    if references != expected_references:
        raise ValueError(
            f"Unexpected missing-type references: expected {expected_references}, got {references}"
        )
    types_mod = root / "api" / "types" / "mod.rs"
    types_source = types_mod.read_text()
    marker = "// Fern v1 compatibility enums for fixed-string schema types.\n"
    if marker in types_source:
        raise ValueError("Fern v1 compatibility enums are already present")
    enums = marker
    for name, (variant, wire_value) in FIXED_STRING_TYPES.items():
        enums += (
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\n"
            f"pub enum {name} {{\n"
            f'    #[serde(rename = "{wire_value}")]\n'
            f"    {variant},\n"
            "}\n\n"
        )
    types_mod.write_text(types_source.rstrip() + "\n\n" + enums.rstrip() + "\n")

    counts: dict[str, int] = {
        f"fixed-enum:{name}": count for name, count in references.items()
    }

    # Multipart scalar collections/maps are JSON values, not Display strings.
    for filename in ("audio_transcription_request.rs", "stream_request.rs"):
        path = root / "api" / "types" / filename
        source = path.read_text()
        for field in ("context_bias", "speaker_references", "timestamp_granularities"):
            old = f'form = form.text("{field}", value.to_string());'
            new = (
                f'form = form.text("{field}", '
                'serde_json::to_string(&value).expect("multipart JSON field serialization"));'
            )
            before = source.count(old)
            if before != 1:
                raise ValueError(f"{filename}:{field}: expected one multipart field, found {before}")
            source = source.replace(old, new)
            counts[f"multipart-json:{filename}:{field}"] = 1
        old_file = 'form = form.text("file", value.to_string());'
        if source.count(old_file) != 1:
            raise ValueError(f"{filename}: expected one file text part")
        source = source.replace(
            old_file,
            'form = form.part("file", reqwest::multipart::Part::bytes(value.0));',
        )
        counts[f"multipart-file:{filename}"] = 1
        path.write_text(source)
        subprocess.run(["rustfmt", "--edition", "2021", str(path)], check=True)

    for filename in ("document_upload.rs", "multi_part_body_params.rs"):
        path = root / "api" / "types" / filename
        source = path.read_text()
        source = replace_exact(
            source,
            'form = form.text("file", self.file.to_string());',
            'form = form.part("file", reqwest::multipart::Part::bytes(self.file.0));',
            1,
            f"{filename}:file multipart part",
        )
        path.write_text(source)
        subprocess.run(["rustfmt", "--edition", "2021", str(path)], check=True)
        counts[f"multipart-file:{filename}"] = 1

    # messages is required in the OpenAPI schema. Fern incorrectly emits
    # serde(default) for this enum field, imposing a nonexistent Default impl.
    path = root / "api" / "types" / "conversation_messages.rs"
    source = path.read_text()
    source = replace_exact(
        source,
        "    #[serde(default)]\n    pub messages: MessageEntries,",
        "    pub messages: MessageEntries,",
        1,
        "ConversationMessages.messages serde(default)",
    )
    path.write_text(source)
    subprocess.run(["rustfmt", "--edition", "2021", str(path)], check=True)
    counts["serde-default:ConversationMessages.messages"] = 1

    # The existing public config calls the bearer credential `token`; Fern's
    # generated auth helper reads `access_token`. Normalize both builder and
    # direct-struct construction so existing client configuration remains valid.
    config_path = root / "config.rs"
    config_source = replace_exact(
        config_path.read_text(),
        "            access_token: self.access_token,",
        "            access_token: self.access_token.or_else(|| self.token.clone()),",
        1,
        "ClientConfigBuilder token normalization",
    )
    config_path.write_text(config_source)
    subprocess.run(["rustfmt", "--edition", "2021", str(config_path)], check=True)
    counts["auth:builder-token-normalization"] = 1

    client_path = root / "client.rs"
    client_source = replace_exact(
        client_path.read_text(),
        "    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {\n        let http_client = Arc::new(",
        "    pub fn new(mut config: ClientConfig) -> Result<Self, crate::error::ApiError> {\n        if config.access_token.is_none() {\n            config.access_token = config.token.clone();\n        }\n        let http_client = Arc::new(",
        1,
        "Mistral::new direct-config token normalization",
    )
    client_path.write_text(client_source)
    subprocess.run(["rustfmt", "--edition", "2021", str(client_path)], check=True)
    counts["auth:direct-config-token-normalization"] = 1

    resource_root = root / "api" / "resources"
    old_override = (
        "                if let Some(token) = &opts.token {\n"
        "                    overridden.access_token = Some(token.clone());\n"
        "                }"
    )
    api_key_override = (
        "                if let Some(api_key) = &opts.api_key {\n"
        "                    overridden.api_key = Some(api_key.clone());\n"
        "                }"
    )
    override_pattern = re.compile(
        re.escape(old_override)
        + r"(?!\n"
        + re.escape(api_key_override)
        + r")"
    )
    token_sites = 0
    existing_api_key_sites = 0
    inserted_api_key_sites = 0
    for resource_path in sorted(resource_root.rglob("*.rs")):
        source = resource_path.read_text()
        token_sites += source.count(old_override)
        existing_api_key_sites += source.count(api_key_override)
        patched, inserted = override_pattern.subn(
            lambda match: match.group(0) + "\n" + api_key_override,
            source,
        )
        if inserted:
            inserted_api_key_sites += inserted
            resource_path.write_text(patched)
            subprocess.run(["rustfmt", "--edition", "2021", str(resource_path)], check=True)
    if (token_sites, existing_api_key_sites, inserted_api_key_sites) != (349, 6, 343):
        raise ValueError(
            "RequestOptions auth override drift: expected 349 token sites, "
            "6 existing API-key overrides, and 343 insertions; got "
            f"{token_sites}, {existing_api_key_sites}, and {inserted_api_key_sites}"
        )
    counts["auth:per-call-api-key-overrides"] = inserted_api_key_sites

    final_types = types_mod.read_text()
    for name, (variant, wire_value) in FIXED_STRING_TYPES.items():
        if (
            f"pub enum {name}" not in final_types
            or f'#[serde(rename = "{wire_value}")]' not in final_types
            or f"    {variant}," not in final_types
        ):
            raise ValueError(f"Missing fixed-string compatibility enum: {name}")
    return counts


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: patch_generated_sdk.py GENERATED_SRC_DIR")
    counts = patch_generated_sdk(Path(sys.argv[1]))
    print("Fern v1 generated-source compatibility patches:")
    for label, count in sorted(counts.items()):
        print(f"  {label}: {count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
