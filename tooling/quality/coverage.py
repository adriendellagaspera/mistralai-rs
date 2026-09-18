"""Fail generation on missing operations or runtime stubs; emit an API inventory."""

import json
from pathlib import Path
import re
import sys

METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}

# Temporary consumer tripwire for additive methods emitted by the pinned raw generator.
# Replace this exact allowlist with generator-owned binding metadata once #31 lands.
MISTRAL_ADDITIVE_METHODS = frozenset({
    "audio_api_v1_transcriptions_post_stream_with_multipart_filenames",
    "audio_api_v1_transcriptions_post_with_multipart_filenames",
    "chat_completion_v1_chat_completions_post_stream",
    "files_api_routes_upload_file_with_multipart_filenames",
    "fim_completion_v1_fim_completions_post_stream",
    "get_voice_sample_audio_v1_audio_voices_voice_id_sample_get_wav",
    "get_voice_sample_audio_v1_audio_voices_voice_id_sample_get_wav_stream",
    "libraries_documents_upload_v1_with_multipart_filenames",
    "speech_v1_audio_speech_post_stream",
})


def operations(spec):
    return [(path, method, op) for path, item in spec["paths"].items()
            for method, op in item.items() if method in METHODS]


def rust_method(operation_id):
    return re.sub(r"_+", "_", operation_id.replace("-", "_")).lower()


def binary_stream_method(op):
    success = [response for status, response in op.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None
    content = success[0].get("content", {})
    if len(content) != 1:
        return None
    payload = next(iter(content.values()))
    schema = payload.get("schema", {})
    if schema.get("type") == "string" and schema.get("format") == "binary":
        return rust_method(op["operationId"]) + "_stream"
    return None


def inventory(original, spec, client, additive_methods=()):
    methods = re.findall(r"pub async fn (\w+)\s*\(", client)
    if len(methods) != len(set(methods)):
        raise ValueError("Generated client contains duplicate async method names")
    expected = {rust_method(op["operationId"]) for _, _, op in operations(spec)}
    expected |= {method for _, _, op in operations(spec)
                 if (method := binary_stream_method(op)) is not None}
    expected |= set(additive_methods)
    actual = set(methods)
    if actual != expected:
        missing = sorted(expected - actual)
        extra = sorted(actual - expected)
        raise ValueError(f"Generated method drift: missing={missing}, extra={extra}")
    if re.search(r'HttpError::Config\s*\(\s*"', client) or "unimplemented!" in client or "todo!" in client:
        raise ValueError("Generated client contains a runtime configuration stub")
    inventory = []
    upstream = {op["operationId"] for _, _, op in operations(original)}
    for path, method, op in operations(spec):
        # Error enum docs retain exact upstream IDs, independent of Rust casing.
        # Untyped-error endpoints still carry their IDs in the generated method.
        snake = rust_method(op["operationId"])
        if snake not in methods:
            raise ValueError(f"Missing generated operation: {op['operationId']} ({snake})")
        entry = {"operation_id": op["operationId"], "method": method.upper(),
                 "path": path.split("#")[0], "rust_method": snake,
                 "upstream": op["operationId"] in upstream,
                 "tags": op.get("tags", []),
                 "success_media": sorted({media for status, response in op.get("responses", {}).items()
                     if str(status).startswith("2") for media in response.get("content", {})})}
        if stream_method := binary_stream_method(op):
            entry["binary_stream_method"] = stream_method
        inventory.append(entry)
    return {
        "upstream_operations": len(upstream), "generated_methods": len(methods),
        "operations": sorted(inventory, key=lambda op: op["operation_id"])
    }


def main(source, prepared, directory):
    from ruamel.yaml import YAML
    original = YAML(typ="safe", pure=True).load(Path(source).read_text())
    spec = json.loads(Path(prepared).read_text())
    client = (Path(directory) / "client.rs").read_text()
    report = inventory(original, spec, client)
    (Path(directory) / "coverage.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main(*sys.argv[1:])
