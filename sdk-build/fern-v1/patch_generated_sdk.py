#!/usr/bin/env python3
"""Apply strict, reproducible workarounds for known Fern Rust v1 defects.

This operates only on freshly generated Rust sources. Every expected edit is
counted and asserted so generator output drift fails loudly rather than being
silently rewritten incorrectly.
"""
from __future__ import annotations

import sys
from pathlib import Path

MISSING_TYPE_NAMES = (
    "GetDeploymentSummariesResponseVespaDeploymentType",
    "AssistantMessageRole",
    "OAuth2ClientCredentialsAuthMethodGrantType",
    "FunctionToolType",
    "OcrCodeBlockType",
)


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
    marker = "// Fern v1 compatibility aliases for fixed-string schema types.\n"
    if marker in types_source:
        raise ValueError("Fern v1 compatibility aliases are already present")
    aliases = marker + "".join(f"pub type {name} = String;\n" for name in MISSING_TYPE_NAMES)
    types_mod.write_text(types_source.rstrip() + "\n\n" + aliases)

    counts: dict[str, int] = {
        f"type-alias:{name}": count for name, count in references.items()
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
    counts["serde-default:ConversationMessages.messages"] = 1

    for name in MISSING_TYPE_NAMES:
        if f"pub type {name} = String;" not in types_mod.read_text():
            raise ValueError(f"Missing generated compatibility alias: {name}")
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
