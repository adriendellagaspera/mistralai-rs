"""Minimal deterministic fixes for contradictions in the upstream spec."""

from pathlib import Path


CHAT_RESPONSE_REQUIRED = b"""    ChatCompletionResponse:
      allOf:
      - $ref: '#/components/schemas/ChatCompletionResponseBase'
      - type: object
        title: ChatCompletionResponse
        properties:
          choices:
            type: array
            items:
              $ref: '#/components/schemas/ChatCompletionChoice'
        required:
        - id
        - object
        - data
        - model
        - usage
        - created
        - choices
"""

CHAT_RESPONSE_REQUIRED_FIXED = CHAT_RESPONSE_REQUIRED.replace(b"        - data\n", b"")


def preprocess(source: bytes) -> bytes:
    """Return generator input without modifying the authenticated source spec."""
    count = source.count(CHAT_RESPONSE_REQUIRED)
    if count != 1:
        raise ValueError(
            "Expected exactly one known ChatCompletionResponse contradiction; "
            f"found {count}. Review the upstream schema before regenerating."
        )
    return source.replace(CHAT_RESPONSE_REQUIRED, CHAT_RESPONSE_REQUIRED_FIXED)


def main(source: str, destination: str) -> None:
    import json
    from ruamel.yaml import YAML

    spec = YAML(typ="safe", pure=True).load(preprocess(Path(source).read_bytes()))
    transform(spec)
    # JSON is valid YAML and avoids emitter/version-dependent YAML formatting.
    Path(destination).write_text(json.dumps(spec, sort_keys=True, indent=2) + "\n")


def transform(spec):
    """Explicit repairs and alternate media methods; original document stays intact."""
    import copy

    schemas = spec["components"]["schemas"]
    for name, invalid, replacement in [
        ("SharingDelete", "level", None),
        ("WorkflowListResponse", "beta.workflows", "workflows"),
    ]:
        schema = schemas[name]
        if invalid not in schema["required"] or invalid in schema["properties"]:
            raise ValueError(f"Review upstream required-field repair for {name}")
        schema["required"].remove(invalid)
        if replacement:
            if replacement not in schema["properties"]:
                raise ValueError(f"Missing replacement {name}.{replacement}")
            schema["required"].append(replacement)

    for path in ["/v1/chat/completions", "/v1/fim/completions", "/v1/audio/speech"]:
        operation = spec["paths"][path]["post"]
        content = operation["responses"]["200"]["content"]
        if set(content) != {"application/json", "text/event-stream"}:
            raise ValueError(f"Review upstream streaming media for {path}")
        stream = copy.deepcopy(operation)
        stream["operationId"] += "_stream"
        stream["responses"]["200"]["content"] = {
            "text/event-stream": content["text/event-stream"]}
        # Match upstream's #stream aliases: the generator strips URL fragments.
        alias = path + "#stream"
        if alias in spec["paths"]:
            raise ValueError(f"Upstream now supplies {alias}; remove local alias")
        spec["paths"][alias] = {"post": stream}

    # This endpoint serves WAV. Keep the upstream JSON variant and add a binary
    # method instead of trying to JSON-decode audio or dropping a declared media.
    path = "/v1/audio/voices/{voice_id}/sample"
    operation = copy.deepcopy(spec["paths"][path]["get"])
    content = operation["responses"]["200"]["content"]
    if set(content) != {"application/json", "audio/wav"}:
        raise ValueError("Review voice sample response media")
    operation["operationId"] += "_wav"
    operation["responses"]["200"]["content"] = {
        "audio/wav": {"schema": {"type": "string", "format": "binary"}}}
    spec["paths"][path + "#wav"] = {"get": operation}


if __name__ == "__main__":
    import sys
    main(*sys.argv[1:])
