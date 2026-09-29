#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

HTTP_METHODS = {"get", "put", "post", "delete", "options", "head", "patch", "trace"}


def operations(doc: dict[str, Any]):
    for path, item in (doc.get("paths") or {}).items():
        if not isinstance(item, dict):
            continue
        for method, op in item.items():
            if method.lower() in HTTP_METHODS and isinstance(op, dict):
                yield path, method.lower(), op


def apply_product_policy(doc: dict[str, Any], policy_path: Path) -> None:
    policy = json.loads(policy_path.read_text())
    methods = policy["method_names"]
    param_names = policy["parameter_names"]
    seen_ids: set[str] = set()

    for _path, _method, op in operations(doc):
        opid = op.get("operationId")
        if not isinstance(opid, str):
            raise ValueError("Every production operation must have an operationId")
        seen_ids.add(opid)

        tags = op.get("tags") or []
        if policy.get("group_from_first_tag"):
            if tags and isinstance(tags[0], str):
                op["x-fern-sdk-group-name"] = tags[0].split(".")
            else:
                op["x-fern-sdk-group-name"] = ["default"]

        if opid in methods:
            op["x-fern-sdk-method-name"] = methods[opid]

        for parameter in op.get("parameters", []):
            if not isinstance(parameter, dict):
                continue
            override = param_names.get(opid, {}).get(parameter.get("name"))
            if override:
                parameter["x-fern-parameter-name"] = override

    unknown_methods = sorted(set(methods) - seen_ids)
    unknown_params = sorted(set(param_names) - seen_ids)
    if unknown_methods or unknown_params:
        raise ValueError(
            f"Product policy references missing operations: methods={unknown_methods}, parameters={unknown_params}"
        )

    for path, stream in policy["streaming"].items():
        op = doc["paths"][path]["post"]
        op["x-fern-sdk-method-name"] = "complete"
        op["x-fern-streaming"] = {
            "format": "sse",
            "stream-condition": "$request.stream",
            "response": {"$ref": f"#/components/schemas/{stream['response']}"},
            "response-stream": {"$ref": f"#/components/schemas/{stream['response_stream']}"},
        }
