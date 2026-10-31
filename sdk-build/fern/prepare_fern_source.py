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

    # The published catalog contains a recursively discriminated And/Or/Not
    # filter tree. Fern's importer's discriminator recursion overflows on the
    # recursive oneOf. The six oneOf branches and their distinct wire tags are
    # sufficient: omit ONLY the discriminator hint on these three nodes.
    filter_tags = {
        "And": "and", "Or": "or", "Not": "not",
        "Equal": "equal", "Range": "range", "In": "in",
    }
    for name in ("And", "Or", "Not"):
        schema = schemas.get(name)
        if schema is None:
            continue
        prop = "match" if name == "Not" else "matches"
        node = schema["properties"][prop]
        if name != "Not":
            node = node["items"]
        mapping = {tag: f"#/components/schemas/{variant}"
                   for variant, tag in filter_tags.items()}
        references = [x.get("$ref") for x in node.get("oneOf", [])]
        if (
            node.get("discriminator") != {"propertyName": "type", "mapping": mapping}
            or set(references) != set(mapping.values())
            or len(references) != len(mapping)
            or schema["properties"]["type"].get("const") != filter_tags[name]
        ):
            raise ValueError(f"Unexpected recursive filter union: {name}")
        for variant, tag in filter_tags.items():
            if schemas[variant]["properties"]["type"].get("const") != tag:
                raise ValueError(f"Non-unique filter tag: {variant}")
        del node["discriminator"]
        applied.append({
            "kind": "temporary_fern_workaround",
            "source": f"#/components/schemas/{name}/properties/{prop}",
            "upstream": "https://github.com/fern-api/fern",
            "remove_when": "Fern parses recursively discriminated oneOf filters",
        })

    # Give Unicode IPA values explicit, stable Rust identifiers through
    # Fern's supported x-fern-enum extension. Preserve every original enum
    # member, enforcing the exact set of accepted serialized wire values.
    # This is normal public naming policy, not an importer workaround.
    enum_annotations = 0

    def annotate_unicode_enum(node: object, pointer: str) -> None:
        nonlocal enum_annotations
        if isinstance(node, list):
            for index, value in enumerate(node):
                annotate_unicode_enum(value, f"{pointer}/{index}")
            return
        if not isinstance(node, dict):
            return
        variants = node.get("enum")
        if isinstance(variants, list) and any(
            isinstance(variant, str) and not variant.isascii()
            for variant in variants
        ):
            if (
                node.get("type") != "string"
                or not variants
                or not all(isinstance(variant, str) for variant in variants)
                or len(variants) != len(set(variants))
            ):
                raise ValueError(f"Unexpected Unicode enum: {pointer}")
            names = {
                value: {
                    "name": "Ipa" + "".join(f"U{ord(char):04X}" for char in value)
                }
                for value in variants
                if not value.isascii()
            }
            if "x-fern-enum" in node and node["x-fern-enum"] != names:
                raise ValueError(f"Upstream enum annotations changed: {pointer}")
            node["x-fern-enum"] = names
            enum_annotations += 1
            return
        for key, value in node.items():
            annotate_unicode_enum(value, f"{pointer}/{key}")

    annotate_unicode_enum(doc, "#")

    # The published ResponsesStreamFrame union's discriminator still points
    # to deprecated Response* aliases, whereas oneOf lists actual Responses*
    # events. Build the mapping from the event type consts rather than
    # silently losing the new stream event variants.
    def normalize_responses(node: object) -> None:
        if isinstance(node, list):
            for value in node:
                normalize_responses(value)
            return
        if not isinstance(node, dict):
            return
        variants = node.get("oneOf")
        if isinstance(variants, list) and any(
            isinstance(value, dict)
            and value.get("$ref") == "#/components/schemas/ResponsesErrorEvent"
            for value in variants
        ):
            discriminator = node.get("discriminator")
            if not isinstance(discriminator, dict):
                raise ValueError("Responses union discriminator is missing")
            key = discriminator.get("propertyName")
            mapping = discriminator.get("mapping")
            if key != "type" or not isinstance(mapping, dict):
                raise ValueError("Responses discriminator changed")
            expected = {}
            for value in variants:
                ref = value.get("$ref") if isinstance(value, dict) else None
                if not isinstance(ref, str) or not ref.startswith("#/components/schemas/"):
                    raise ValueError("Unexpected Responses variant")
                target = schemas.get(ref.rsplit("/", 1)[-1])
                if not isinstance(target, dict):
                    raise ValueError(f"Unknown Responses variant {ref}")
                tag_schema = target.get("properties", {}).get(key, {})
                tag = tag_schema.get("const")
                if tag is None and len(tag_schema.get("enum", [])) == 1:
                    tag = tag_schema["enum"][0]
                if not isinstance(tag, str) or tag in expected:
                    raise ValueError(f"Ambiguous Responses event tag: {ref}")
                expected[tag] = ref
            if set(mapping) != set(expected):
                raise ValueError("Responses event tag set changed")
            if mapping != expected:
                discriminator["mapping"] = expected
                applied.append({
                    "kind": "temporary_fern_workaround",
                    "source": "#/components/schemas/ResponsesStreamFrame",
                    "upstream": "https://github.com/fern-api/fern",
                    "remove_when": "published Responses discriminator matches its oneOf members",
                })
            return
        for value in node.values():
            normalize_responses(value)

    normalize_responses(doc)

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
        "supported_enum_annotations": enum_annotations,
    }
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
