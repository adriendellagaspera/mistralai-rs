#!/usr/bin/env python3
from __future__ import annotations

import sys
from copy import deepcopy
from pathlib import Path

import yaml

src = Path(sys.argv[1])
dst = Path(sys.argv[2])
doc = yaml.safe_load(src.read_text())
schemas = doc["components"]["schemas"]

# Fern currently rejects an allOf child that intentionally narrows an inherited
# property (the conversation stream=true/false request variants). Flatten only
# these known request variants by copying the parent and overlaying the child.
for name in (
    "ConversationRequest",
    "ConversationStreamRequest",
    "ConversationAppendRequest",
    "ConversationRestartRequest",
    "ConversationAppendStreamRequest",
    "ConversationRestartStreamRequest",
):
    schema = schemas.get(name)
    if not isinstance(schema, dict) or not isinstance(schema.get("allOf"), list):
        continue
    merged = {}
    properties = {}
    required = []
    for branch in schema["allOf"]:
        if "$ref" in branch:
            parent = deepcopy(schemas[branch["$ref"].rsplit("/", 1)[-1]])
            properties.update(parent.pop("properties", {}))
            required.extend(parent.pop("required", []))
            merged.update(parent)
        else:
            child = deepcopy(branch)
            properties.update(child.pop("properties", {}))
            required.extend(child.pop("required", []))
            merged.update(child)
    merged["properties"] = properties
    if required:
        merged["required"] = list(dict.fromkeys(required))
    schemas[name] = merged

# SDK-name-only disambiguation: wire names remain unchanged.
for item in doc["paths"].values():
    for op in item.values():
        if not isinstance(op, dict):
            continue
        opid = op.get("operationId")
        for parameter in op.get("parameters", []):
            wire = parameter.get("name")
            if opid in {"prompts_list", "skills_list"} and wire == "sort.direction":
                parameter["x-fern-parameter-name"] = "sort_direction_legacy"
            if opid in {"stream_deployment_logs", "stream_workflow_execution_logs"} and wire == "Last-Event-ID":
                parameter["x-fern-parameter-name"] = "last_event_id_header"

# Fern 5.112 rejects this valid leading-hyphen enum default. Keep the accepted
# enum values but omit the generated-client default in the experimental input.
for item in doc["paths"].values():
    for op in item.values():
        if not isinstance(op, dict) or op.get("operationId") != "jobs_api_routes_batch_get_batch_jobs":
            continue
        for parameter in op.get("parameters", []):
            if parameter.get("name") == "order_by":
                parameter.get("schema", {}).pop("default", None)

# Fern 5.112 does not propagate generator-level disable-examples into the
# OpenAPI importer. The importer therefore synthesizes endpoint examples before
# the generator-level flag can clear them. Its synthesized examples for these
# three live-judging endpoints are invalid (the request contains a discriminated
# union and the generated response is validated against the wrong union).
# Supplying complete x-fern-examples prevents that importer autogeneration.
judge_examples = {
    "judge_chat_completion_event_v1_observability_chat_completion_events__event_id__live_judging_post": {
        "path-parameters": {"event_id": "00000000-0000-0000-0000-000000000000"},
        "request": {
            "judge_definition": {
                "name": "judge",
                "description": "judge",
                "model_name": "model",
                "output": {
                    "type": "CLASSIFICATION",
                    "options": [{"value": "ok", "description": "ok"}],
                },
                "instructions": "judge",
                "tools": [],
            }
        },
        "response": {"body": {"analysis": "ok", "answer": "ok"}},
    },
    "judge_conversation_v1_observability_judges__judge_id__live_judging_post": {
        "path-parameters": {"judge_id": "00000000-0000-0000-0000-000000000000"},
        "request": {"messages": []},
        "response": {"body": {"analysis": "ok", "answer": "ok"}},
    },
    "judge_dataset_record_v1_observability_dataset_records__dataset_record_id__live_judging_post": {
        "path-parameters": {"dataset_record_id": "00000000-0000-0000-0000-000000000000"},
        "request": {
            "judge_definition": {
                "name": "judge",
                "description": "judge",
                "model_name": "model",
                "output": {
                    "type": "CLASSIFICATION",
                    "options": [{"value": "ok", "description": "ok"}],
                },
                "instructions": "judge",
                "tools": [],
            }
        },
        "response": {"body": {"analysis": "ok", "answer": "ok"}},
    },
}
for item in doc["paths"].values():
    for op in item.values():
        if not isinstance(op, dict):
            continue
        example = judge_examples.get(op.get("operationId"))
        if example is not None:
            op["x-fern-examples"] = [example]

dst.write_text(yaml.safe_dump(doc, sort_keys=False))
