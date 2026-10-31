"""Compatibility regressions for new Mistral public OpenAPI features."""
from __future__ import annotations

import copy
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

import yaml

MODULE = Path(__file__).resolve().parent / "fern" / "prepare_fern_source.py"
SPEC = importlib.util.spec_from_file_location("prepare_fern_source", MODULE)
assert SPEC is not None and SPEC.loader is not None
prepare = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(prepare)

TAGS = {
    "And": "and",
    "Or": "or",
    "Not": "not",
    "Equal": "equal",
    "Range": "range",
    "In": "in",
}


def fixture() -> dict:
    mapping = {tag: f"#/components/schemas/{name}" for name, tag in TAGS.items()}
    components = {}
    for name, tag in TAGS.items():
        schema = {
            "type": "object",
            "properties": {"type": {"const": tag, "type": "string"}},
        }
        if name in ("And", "Or", "Not"):
            union = {
                "discriminator": {"propertyName": "type", "mapping": dict(mapping)},
                "oneOf": [{"$ref": value} for value in mapping.values()],
            }
            if name == "Not":
                schema["properties"]["match"] = union
            else:
                schema["properties"]["matches"] = {"items": union, "type": "array"}
        components[name] = schema

    # New Responses* members take precedence over legacy Response* aliases.
    events = {
        "ResponsesErrorEvent": "error",
        "ResponsesCompletedEvent": "response.completed",
    }
    for name, tag in events.items():
        components[name] = {
            "properties": {"type": {"type": "string", "const": tag}},
            "type": "object",
        }
    components["ResponsesStreamFrame"] = {
        "properties": {
            "data": {
                "oneOf": [
                    {"$ref": f"#/components/schemas/{name}"} for name in events
                ],
                "discriminator": {
                    "propertyName": "type",
                    "mapping": {
                        "error": "#/components/schemas/ResponseErrorEvent",
                        "response.completed": "#/components/schemas/ResponsesCompletedEvent",
                    },
                },
            }
        }
    }
    components["PhonemeHolder"] = {
        "type": "object",
        "properties": {
            "phoneme": {"type": "string", "enum": ["aɪ", "tʃ", "θ", "plain"]},
        },
    }
    return {
        "openapi": "3.1.0",
        "info": {"title": "fixture", "version": "1.0"},
        "paths": {},
        "components": {"schemas": components},
    }


def transform(document: dict) -> tuple[dict, dict]:
    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        source, target, report = (
            root / "source.yaml",
            root / "compatible.yaml",
            root / "report.json",
        )
        source.write_text(yaml.safe_dump(document, sort_keys=False))
        with patch.object(
            sys, "argv", ["prepare_fern_source.py", str(source), str(target), str(report)]
        ):
            prepare.main()
        return yaml.safe_load(target.read_text()), json.loads(report.read_text())


class FernCatalogCompatibilityTests(unittest.TestCase):
    def test_wire_contract_and_all_union_variants_are_preserved(self) -> None:
        original = fixture()
        snapshot = copy.deepcopy(original)
        generated, report = transform(original)
        self.assertEqual(original, snapshot)
        original_schemas = original["components"]["schemas"]
        generated_schemas = generated["components"]["schemas"]
        for name in ("And", "Or", "Not"):
            key = "match" if name == "Not" else "matches"
            before = original_schemas[name]["properties"][key]
            after = generated_schemas[name]["properties"][key]
            if name != "Not":
                before, after = before["items"], after["items"]
            self.assertNotIn("discriminator", after)
            self.assertEqual(before["oneOf"], after["oneOf"])
        for name, tag in TAGS.items():
            self.assertEqual(
                generated_schemas[name]["properties"]["type"]["const"], tag
            )

        frame = generated_schemas["ResponsesStreamFrame"]["properties"]["data"]
        self.assertEqual(
            frame["discriminator"]["mapping"],
            {
                "error": "#/components/schemas/ResponsesErrorEvent",
                "response.completed": "#/components/schemas/ResponsesCompletedEvent",
            },
        )
        self.assertEqual(
            frame["oneOf"],
            original_schemas["ResponsesStreamFrame"]["properties"]["data"]["oneOf"],
        )

        ipa = generated_schemas["PhonemeHolder"]["properties"]["phoneme"]
        self.assertEqual(ipa["type"], "string")
        self.assertNotIn("enum", ipa)
        workarounds = report["temporary_workarounds"]
        ipa_entries = [w for w in workarounds if "wire_values" in w]
        self.assertEqual(len(ipa_entries), 1)
        self.assertEqual(ipa_entries[0]["wire_values"], ["aɪ", "tʃ", "θ", "plain"])

    def test_changed_filter_mapping_fails_closed(self) -> None:
        original = fixture()
        original["components"]["schemas"]["Not"]["properties"]["match"][
            "discriminator"
        ]["propertyName"] = "kind"
        with self.assertRaisesRegex(ValueError, "Unexpected recursive filter"):
            transform(original)

    def test_duplicate_responses_tags_fail_closed(self) -> None:
        original = fixture()
        schema = original["components"]["schemas"]["ResponsesCompletedEvent"]
        schema["properties"]["type"]["const"] = "error"
        with self.assertRaisesRegex(ValueError, "Ambiguous Responses event tag"):
            transform(original)


if __name__ == "__main__":
    unittest.main()
