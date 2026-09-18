"""Fail-closed checks for published OpenAPI assumptions owned by this consumer."""

from pathlib import Path


def load_published(source: bytes):
    """Parse and validate the authenticated published document without mutating it."""
    from ruamel.yaml import YAML

    spec = YAML(typ="safe", pure=True).load(source)
    assert_overlay_assumptions(spec)
    return spec


def assert_overlay_assumptions(spec) -> None:
    """Fail closed if contradictions targeted by the repository Overlay drift."""
    schemas = spec["components"]["schemas"]

    chat = schemas["ChatCompletionResponse"]
    all_of = chat.get("allOf")
    if not isinstance(all_of, list) or len(all_of) < 2:
        raise ValueError("Review Overlay target for ChatCompletionResponse")
    chat_extension = all_of[1]
    chat_required = chat_extension.get("required", [])
    chat_properties = chat_extension.get("properties", {})
    if "data" not in chat_required or "data" in chat_properties:
        raise ValueError("Review Overlay repair for ChatCompletionResponse.data")

    sharing = schemas["SharingDelete"]
    if "level" not in sharing.get("required", []) or "level" in sharing.get("properties", {}):
        raise ValueError("Review Overlay repair for SharingDelete.level")

    workflows = schemas["WorkflowListResponse"]
    required = workflows.get("required", [])
    properties = workflows.get("properties", {})
    if (
        "beta.workflows" not in required
        or "beta.workflows" in properties
        or "workflows" not in properties
        or "workflows" in required
    ):
        raise ValueError("Review Overlay repair for WorkflowListResponse.workflows")

    paths = spec["paths"]
    for path in (
        "/v1/chat/completions",
        "/v1/fim/completions",
        "/v1/audio/speech",
    ):
        content = paths[path]["post"]["responses"]["200"]["content"]
        if set(content) != {"application/json", "text/event-stream"}:
            raise ValueError(f"Review response representations for {path}")
        if path + "#stream" in paths:
            raise ValueError(
                f"Published OpenAPI now supplies {path}#stream; review synthetic-path retirement"
            )

    voice_path = "/v1/audio/voices/{voice_id}/sample"
    voice_content = paths[voice_path]["get"]["responses"]["200"]["content"]
    if set(voice_content) != {"application/json", "audio/wav"}:
        raise ValueError(f"Review response representations for {voice_path}")
    if voice_path + "#wav" in paths:
        raise ValueError(
            f"Published OpenAPI now supplies {voice_path}#wav; review synthetic-path retirement"
        )


def preprocess(source: bytes) -> bytes:
    """Validate the authenticated published document without mutating it."""
    load_published(source)
    return source


def path_inventory(source: bytes) -> list[str]:
    """Return the published path keys after the same fail-closed validation."""
    spec = load_published(source)
    return sorted(spec["paths"])


def main(source: str, *, print_paths: bool = False) -> None:
    import json

    data = Path(source).read_bytes()
    if print_paths:
        print(json.dumps(path_inventory(data), separators=(",", ":")))
    else:
        preprocess(data)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source")
    parser.add_argument("--print-paths", action="store_true")
    args = parser.parse_args()
    main(args.source, print_paths=args.print_paths)
