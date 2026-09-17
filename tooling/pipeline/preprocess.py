"""Fail-closed checks for published OpenAPI assumptions owned by this consumer."""

from pathlib import Path


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


def preprocess(source: bytes) -> bytes:
    """Validate the authenticated published document without mutating it."""
    from ruamel.yaml import YAML

    spec = YAML(typ="safe", pure=True).load(source)
    assert_overlay_assumptions(spec)
    return source


def main(source: str) -> None:
    preprocess(Path(source).read_bytes())


if __name__ == "__main__":
    import sys

    main(*sys.argv[1:])
