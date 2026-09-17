"""Temporary call-shape aliases that are still blocked on mistralai-rs#31."""

from pathlib import Path


def assert_overlay_assumptions(spec) -> None:
    """Fail closed if the published contradictions targeted by our Overlay drift."""
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


def transform(spec) -> None:
    """Add only the legacy media aliases that #31 has not replaced yet."""
    import copy

    assert_overlay_assumptions(spec)

    for path in ["/v1/chat/completions", "/v1/fim/completions", "/v1/audio/speech"]:
        operation = spec["paths"][path]["post"]
        content = operation["responses"]["200"]["content"]
        if set(content) != {"application/json", "text/event-stream"}:
            raise ValueError(f"Review upstream streaming media for {path}")
        stream = copy.deepcopy(operation)
        stream["operationId"] += "_stream"
        stream["responses"]["200"]["content"] = {
            "text/event-stream": content["text/event-stream"]
        }
        # Temporary: the generator strips URL fragments. Delete these aliases
        # as soon as #31 owns request discriminators and binding metadata.
        alias = path + "#stream"
        if alias in spec["paths"]:
            raise ValueError(f"Upstream now supplies {alias}; remove local alias")
        spec["paths"][alias] = {"post": stream}

    path = "/v1/audio/voices/{voice_id}/sample"
    operation = copy.deepcopy(spec["paths"][path]["get"])
    content = operation["responses"]["200"]["content"]
    if set(content) != {"application/json", "audio/wav"}:
        raise ValueError("Review voice sample response media")
    operation["operationId"] += "_wav"
    operation["responses"]["200"]["content"] = {
        "audio/wav": {"schema": {"type": "string", "format": "binary"}}
    }
    alias = path + "#wav"
    if alias in spec["paths"]:
        raise ValueError(f"Upstream now supplies {alias}; remove local alias")
    spec["paths"][alias] = {"get": operation}


def preprocess(source: bytes) -> bytes:
    """Return temporary generator input without mutating contract corrections."""
    import json
    from ruamel.yaml import YAML

    spec = YAML(typ="safe", pure=True).load(source)
    transform(spec)
    # JSON is valid YAML and avoids emitter/version-dependent YAML formatting.
    return (json.dumps(spec, sort_keys=True, indent=2) + "\n").encode()


def main(source: str, destination: str) -> None:
    Path(destination).write_bytes(preprocess(Path(source).read_bytes()))


if __name__ == "__main__":
    import sys

    main(*sys.argv[1:])
