"""Fail-closed assertions for Mistral OpenAPI assumptions owned by this consumer."""

from pathlib import Path
import sys

from ruamel.yaml import YAML


LOCAL_ALTERNATE_PATHS = (
    "/v1/chat/completions",
    "/v1/fim/completions",
    "/v1/audio/speech",
)


def validate_published(spec: dict) -> None:
    """Reject source drift before repository-owned overlays are applied."""
    schemas = spec["components"]["schemas"]

    chat = schemas["ChatCompletionResponse"]
    branches = chat.get("allOf")
    if not isinstance(branches, list) or len(branches) < 2:
        raise ValueError("Review ChatCompletionResponse overlay assumption")
    chat_object = branches[1]
    required = chat_object.get("required", [])
    properties = chat_object.get("properties", {})
    if required.count("data") != 1 or "data" in properties:
        raise ValueError("Review ChatCompletionResponse.required/data overlay assumption")

    sharing = schemas["SharingDelete"]
    if sharing.get("required", []).count("level") != 1 or "level" in sharing.get("properties", {}):
        raise ValueError("Review SharingDelete.required/level overlay assumption")

    workflows = schemas["WorkflowListResponse"]
    workflow_required = workflows.get("required", [])
    workflow_properties = workflows.get("properties", {})
    if (
        workflow_required.count("beta.workflows") != 1
        or "beta.workflows" in workflow_properties
        or "workflows" not in workflow_properties
        or "workflows" in workflow_required
    ):
        raise ValueError("Review WorkflowListResponse required-field overlay assumption")

    paths = spec["paths"]
    for path in LOCAL_ALTERNATE_PATHS:
        content = paths[path]["post"]["responses"]["200"]["content"]
        if set(content) != {"application/json", "text/event-stream"}:
            raise ValueError(f"Review response representations for {path}")
        if path + "#stream" in paths:
            raise ValueError(f"Published OpenAPI now supplies {path}#stream; review local migration")

    voice_path = "/v1/audio/voices/{voice_id}/sample"
    voice_content = paths[voice_path]["get"]["responses"]["200"]["content"]
    if set(voice_content) != {"application/json", "audio/wav"}:
        raise ValueError(f"Review response representations for {voice_path}")
    if voice_path + "#wav" in paths:
        raise ValueError(f"Published OpenAPI now supplies {voice_path}#wav; review local migration")


def load_and_validate(path: Path) -> None:
    spec = YAML(typ="safe", pure=True).load(path.read_bytes())
    validate_published(spec)


if __name__ == "__main__":
    load_and_validate(Path(sys.argv[1]))
