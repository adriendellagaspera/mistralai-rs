#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from copy import deepcopy
from pathlib import Path

import yaml

FERN_ALL_OF_ISSUE = "https://github.com/fern-api/fern/issues/17930"
FERN_ENUM_DEFAULT_ISSUE = "https://github.com/fern-api/fern/issues/17931"


def main() -> int:
    if len(sys.argv) != 4:
        raise SystemExit(
            "usage: prepare_fern_source.py INPUT OUTPUT REPORT"
        )
    src, dst, report_path = map(Path, sys.argv[1:])
    doc = yaml.safe_load(src.read_text())
    schemas = doc["components"]["schemas"]
    applied: list[dict[str, str]] = []

    # Presentation metadata for the generated Rust crate. This does not alter
    # the API wire contract; it prevents Fern's generic "official SDK" fallback.
    doc.setdefault("info", {})["description"] = (
        "Unofficial asynchronous Rust SDK for Mistral AI. "
        "This project is not affiliated with Mistral AI."
    )

    # Temporary Fern importer compatibility only. Remove this rewrite as soon
    # as the pinned Fern CLI contains the fix tracked by #17930.
    for name in (
        "ConversationRequest",
        "ConversationStreamRequest",
        "ConversationAppendRequest",
        "ConversationRestartRequest",
        "ConversationAppendStreamRequest",
        "ConversationRestartStreamRequest",
    ):
        schema = schemas.get(name)
        if not isinstance(schema, dict) or not isinstance(
            schema.get("allOf"), list
        ):
            continue
        merged: dict = {}
        properties: dict = {}
        required: list[str] = []
        for branch in schema["allOf"]:
            if "$ref" in branch:
                parent = deepcopy(
                    schemas[branch["$ref"].rsplit("/", 1)[-1]]
                )
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
        applied.append(
            {
                "kind": "temporary_fern_workaround",
                "source": f"#/components/schemas/{name}",
                "upstream": FERN_ALL_OF_ISSUE,
                "remove_when": (
                    "the pinned Fern CLI accepts compatible allOf property "
                    "narrowing without flattening"
                ),
            }
        )

    # These are supported Fern source annotations that disambiguate source
    # parameter identities after Rust identifier normalization. They are not
    # transport/runtime patches and do not alter wire names.
    parameter_annotations = 0
    for path_item in doc.get("paths", {}).values():
        if not isinstance(path_item, dict):
            continue
        for operation in path_item.values():
            if not isinstance(operation, dict):
                continue
            op_id = operation.get("operationId")
            for parameter in operation.get("parameters", []):
                if not isinstance(parameter, dict):
                    continue
                wire = parameter.get("name")
                if (
                    op_id in {"prompts_list", "skills_list"}
                    and wire == "sort.direction"
                ):
                    parameter["x-fern-parameter-name"] = (
                        "sort_direction_legacy"
                    )
                    parameter_annotations += 1
                if (
                    op_id
                    in {
                        "stream_deployment_logs",
                        "stream_workflow_execution_logs",
                    }
                    and wire == "Last-Event-ID"
                ):
                    parameter["x-fern-parameter-name"] = (
                        "last_event_id_header"
                    )
                    parameter_annotations += 1

    # Temporary importer compatibility for #17931. Remove only the generated
    # client default; the accepted enum wire values remain unchanged.
    for path_item in doc.get("paths", {}).values():
        if not isinstance(path_item, dict):
            continue
        for operation in path_item.values():
            if (
                not isinstance(operation, dict)
                or operation.get("operationId")
                != "jobs_api_routes_batch_get_batch_jobs"
            ):
                continue
            for parameter in operation.get("parameters", []):
                if (
                    isinstance(parameter, dict)
                    and parameter.get("name") == "order_by"
                    and isinstance(parameter.get("schema"), dict)
                    and parameter["schema"].get("default") == "-created"
                ):
                    parameter["schema"].pop("default")
                    applied.append(
                        {
                            "kind": "temporary_fern_workaround",
                            "source": (
                                "operationId:"
                                "jobs_api_routes_batch_get_batch_jobs/"
                                "parameter:order_by"
                            ),
                            "upstream": FERN_ENUM_DEFAULT_ISSUE,
                            "remove_when": (
                                "the pinned Fern CLI accepts '-created' as "
                                "a string-enum default"
                            ),
                        }
                    )

    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(yaml.safe_dump(doc, sort_keys=False))
    report = {
        "schema_version": 1,
        "temporary_workarounds": applied,
        "supported_parameter_annotations": parameter_annotations,
    }
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
