"""Fail-closed assertions for Mistral OpenAPI assumptions owned by this consumer."""

from pathlib import Path
import sys

from ruamel.yaml import YAML


LOCAL_ALTERNATE_PATHS = (
    "/v1/chat/completions",
    "/v1/fim/completions",
    "/v1/audio/speech",
)

STREAM_OPERATIONS = (
    (
        "/v1/conversations",
        "post",
        "agents_api_v1_conversations_start",
        "ConversationRequest",
        {"application/json"},
    ),
    (
        "/v1/conversations#stream",
        "post",
        "agents_api_v1_conversations_start_stream",
        "ConversationStreamRequest",
        {"text/event-stream"},
    ),
    (
        "/v1/conversations/{conversation_id}",
        "post",
        "agents_api_v1_conversations_append",
        "ConversationAppendRequest",
        {"application/json"},
    ),
    (
        "/v1/conversations/{conversation_id}#stream",
        "post",
        "agents_api_v1_conversations_append_stream",
        "ConversationAppendStreamRequest",
        {"text/event-stream"},
    ),
    (
        "/v1/conversations/{conversation_id}/restart",
        "post",
        "agents_api_v1_conversations_restart",
        "ConversationRestartRequest",
        {"application/json"},
    ),
    (
        "/v1/conversations/{conversation_id}/restart#stream",
        "post",
        "agents_api_v1_conversations_restart_stream",
        "ConversationRestartStreamRequest",
        {"text/event-stream"},
    ),
    (
        "/v1/audio/transcriptions",
        "post",
        "audio_api_v1_transcriptions_post",
        "AudioTranscriptionRequest",
        {"application/json"},
    ),
    (
        "/v1/audio/transcriptions#stream",
        "post",
        "audio_api_v1_transcriptions_post_stream",
        "AudioTranscriptionRequestStream",
        {"text/event-stream"},
    ),
    (
        "/v1/chat/completions",
        "post",
        "chat_completion_v1_chat_completions_post",
        "ChatCompletionRequest",
        {"application/json", "text/event-stream"},
    ),
    (
        "/v1/fim/completions",
        "post",
        "fim_completion_v1_fim_completions_post",
        "FIMCompletionRequest",
        {"application/json", "text/event-stream"},
    ),
    (
        "/v1/audio/speech",
        "post",
        "speech_v1_audio_speech_post",
        "SpeechRequest",
        {"application/json", "text/event-stream"},
    ),
    (
        "/v1/agents/completions",
        "post",
        "agents_completion_v1_agents_completions_post",
        "AgentsCompletionRequest",
        {"application/json"},
    ),
)


def _stream_property(schema: dict) -> dict:
    matches = []
    properties = schema.get("properties")
    if isinstance(properties, dict) and "stream" in properties:
        matches.append(properties["stream"])
    for branch in schema.get("allOf", []):
        if not isinstance(branch, dict):
            continue
        properties = branch.get("properties")
        if isinstance(properties, dict) and "stream" in properties:
            matches.append(properties["stream"])
    if len(matches) != 1:
        raise ValueError(f"Expected exactly one stream property; found {len(matches)}")
    return matches[0]


def _validate_stream_operation(paths: dict, schemas: dict, rule: tuple) -> None:
    path, method, operation_id, schema_name, response_media = rule
    try:
        operation = paths[path][method]
    except KeyError as error:
        raise ValueError(f"Review stream operation {operation_id}: missing {path} {method}") from error

    if operation.get("operationId") != operation_id:
        raise ValueError(f"Review stream operation identity for {path} {method}")

    response_content = operation["responses"]["200"].get("content", {})
    if set(response_content) != response_media:
        raise ValueError(f"Review response representations for {operation_id}")

    request_content = operation.get("requestBody", {}).get("content", {})
    if len(request_content) != 1:
        raise ValueError(f"Review request media for {operation_id}")
    request_schema = next(iter(request_content.values())).get("schema", {})
    expected_ref = f"#/components/schemas/{schema_name}"
    if request_schema.get("$ref") != expected_ref:
        raise ValueError(f"Review request schema for {operation_id}")

    stream = _stream_property(schemas[schema_name])
    if stream.get("type") != "boolean":
        raise ValueError(f"Review {schema_name}.stream discriminator type")


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
    if (
        sharing.get("required", []).count("level") != 1
        or "level" in sharing.get("properties", {})
    ):
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
            raise ValueError(
                f"Published OpenAPI now supplies {path}#stream; review local migration"
            )

    voice_path = "/v1/audio/voices/{voice_id}/sample"
    voice_content = paths[voice_path]["get"]["responses"]["200"]["content"]
    if set(voice_content) != {"application/json", "audio/wav"}:
        raise ValueError(f"Review response representations for {voice_path}")
    if voice_path + "#wav" in paths:
        raise ValueError(
            f"Published OpenAPI now supplies {voice_path}#wav; review local migration"
        )

    for rule in STREAM_OPERATIONS:
        _validate_stream_operation(paths, schemas, rule)


def load_and_validate(path: Path) -> None:
    spec = YAML(typ="safe", pure=True).load(path.read_bytes())
    validate_published(spec)


if __name__ == "__main__":
    load_and_validate(Path(sys.argv[1]))
