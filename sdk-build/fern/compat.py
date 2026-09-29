#!/usr/bin/env python3
"""Temporary OpenAPI-input compatibility workarounds for pinned Fern importer bugs."""
from __future__ import annotations

from copy import deepcopy
from typing import Any

from policy import operations

FERN_ISSUES = {
    "type_collision": "https://github.com/fern-api/fern/issues/17929",
    "allof_narrowing": "https://github.com/fern-api/fern/issues/17930",
    "enum_default": "https://github.com/fern-api/fern/issues/17931",
    "group_without_method": "https://github.com/fern-api/fern/issues/17932",
}


def apply_input_compat(doc: dict[str, Any]) -> list[str]:
    applied: list[str] = []
    schemas = doc["components"]["schemas"]

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
            raise ValueError(f"Fern allOf workaround target changed: {name}")
        merged: dict[str, Any] = {}
        properties: dict[str, Any] = {}
        required: list[str] = []
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
    applied.append(FERN_ISSUES["allof_narrowing"])

    schemas["Judge"]["properties"]["output"]["x-fern-type-name"] = "JudgeOutputConfig"
    applied.append(FERN_ISSUES["type_collision"])

    enum_fixed = False
    group_fallbacks = 0
    for _path, _method, op in operations(doc):
        opid = op.get("operationId")
        if opid == "jobs_api_routes_batch_get_batch_jobs":
            for parameter in op.get("parameters", []):
                if parameter.get("name") == "order_by":
                    schema = parameter.get("schema", {})
                    if schema.get("default") != "-created":
                        raise ValueError("Fern enum-default workaround target changed")
                    schema.pop("default")
                    enum_fixed = True

        if op.get("x-fern-sdk-group-name") and not op.get("x-fern-sdk-method-name"):
            if not isinstance(opid, str):
                raise ValueError("Fern group workaround requires operationId")
            op["x-fern-sdk-method-name"] = opid
            group_fallbacks += 1

    if not enum_fixed:
        raise ValueError("Fern enum-default workaround target was not found")
    applied.append(FERN_ISSUES["enum_default"])
    if group_fallbacks:
        applied.append(FERN_ISSUES["group_without_method"])
    return applied
