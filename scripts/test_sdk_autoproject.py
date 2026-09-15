import sys
from pathlib import Path
import unittest
from types import SimpleNamespace

from rust_types import parse_type

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "codegen"))
import sdk_autoproject


class FakeOpenApi:
    def __init__(self):
        self.schemas = {
            "Thing": {
                "type": "object",
                "properties": {
                    "id": {"type": "string"},
                    "count": {"type": "integer"},
                },
            },
            "CreateThing": {
                "type": "object",
                "required": ["name"],
                "properties": {
                    "name": {"type": "string"},
                    "enabled": {"type": "boolean"},
                },
            },
        }
        self.operations = {
            "list_things": {
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
            "create_thing": {
                "requestBody": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/CreateThing"}}}},
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
        }


def manifest():
    return {"schema_version": 2, "client": {"name": "Client"}, "models": {}, "resources": {}}


def raw_coverage(*operation_ids: str):
    return {
        "operations": [
            {"operation_id": operation_id, "rust_method": operation_id, "upstream": True}
            for operation_id in operation_ids
        ]
    }


class AutoProjectionTests(unittest.TestCase):
    def test_projects_official_nested_taxonomy(self):
        expanded, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {
                "list_things": ["beta.things.list"],
                "create_thing": ["beta.things.create"],
            }},
            raw_coverage("list_things", "create_thing"),
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn("beta", expanded["resources"])
        self.assertEqual(["beta", "things"], expanded["resources"]["beta_things"]["path"])
        self.assertIn("list", expanded["resources"]["beta_things"]["operations"])
        self.assertIn("create", expanded["resources"]["beta_things"]["operations"])
        self.assertEqual("CreateThingParams", expanded["resources"]["beta_things"]["operations"]["create"]["request"])
        self.assertEqual("ThingView", expanded["resources"]["beta_things"]["operations"]["list"]["response"])

    def test_projects_nested_request_objects_with_generated_adapters(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"]["properties"]["tool"] = {"$ref": "#/components/schemas/Tool"}
        api.schemas["Tool"] = {
            "type": "object", "required": ["name"],
            "properties": {"name": {"type": "string"}},
        }
        fields = {
            "CreateThing": (
                SimpleNamespace(name="name", type="String"),
                SimpleNamespace(name="enabled", type="Option<bool>"),
                SimpleNamespace(name="tool", type="Option<Tool>"),
            ),
            "Tool": (SimpleNamespace(name="name", type="String"),),
        }
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, symbol_modules={name: "types" for name in fields},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        self.assertEqual({"tool": "ToolParams"}, expanded["models"]["CreateThingParams"]["adapters"])
        self.assertEqual(["name"], expanded["models"]["ToolParams"]["constructor"])

    def test_projects_safe_map_alias_without_leaking_generated_alias_name(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"]["properties"]["metadata"] = {
            "type": "object", "additionalProperties": True,
        }
        fields = {
            "CreateThing": (
                SimpleNamespace(name="name", type="String"),
                SimpleNamespace(name="enabled", type="Option<bool>"),
                SimpleNamespace(name="metadata", type="Option<MetadataDict>"),
            ),
        }
        rust = SimpleNamespace(
            structs=set(fields),
            aliases={"MetadataDict": parse_type("std::collections::BTreeMap<String, serde_json::Value>")},
            symbol_modules={"CreateThing": "types", "MetadataDict": "types"},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        self.assertEqual({"metadata": "MetadataDictValue"}, expanded["models"]["CreateThingParams"]["adapters"])
        self.assertTrue(expanded["models"]["MetadataDictValue"]["type_alias"])

    def test_projects_empty_success_without_fake_response_model(self):
        api = FakeOpenApi()
        api.operations["delete_thing"] = {
            "responses": {"204": {"description": "No Content"}},
            "parameters": [],
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"delete_thing": ["things.delete"]}},
            raw_coverage("delete_thing"),
        )
        self.assertEqual(1, report["added_count"])
        operation = expanded["resources"]["things"]["operations"]["delete"]
        self.assertTrue(operation["empty_response"])
        self.assertNotIn("response", operation)

    def test_projects_binary_success_without_fake_response_model(self):
        api = FakeOpenApi()
        api.operations["download_thing"] = {
            "responses": {"200": {"content": {"application/octet-stream": {"schema": {"type": "string", "format": "binary"}}}}},
            "parameters": [],
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"download_thing": ["things.download"]}},
            {"operations": [{
                "operation_id": "download_thing",
                "rust_method": "download_thing",
                "binary_stream_method": "download_thing_stream",
                "upstream": True,
            }]},
        )
        self.assertEqual(1, report["added_count"])
        operation = expanded["resources"]["things"]["operations"]["download"]
        self.assertTrue(operation["binary_response"])
        self.assertNotIn("response", operation)

    def test_equal_depth_aliases_require_review(self):
        _, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {"list_things": ["things.list", "items.list"]}},
            raw_coverage("list_things"),
        )
        self.assertEqual("taxonomy_alias_ambiguity", report["rejected"]["list_things"])

    def test_nullable_response_field_is_not_flattened(self):
        api = FakeOpenApi()
        api.schemas["Thing"]["properties"]["description"] = {
            "anyOf": [{"type": "string"}, {"type": "null"}]
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"list_things": ["things.list"]}},
            raw_coverage("list_things"),
        )
        self.assertEqual(1, report["added_count"])
        accessors = expanded["models"]["ThingView"]["accessors"]
        self.assertNotIn("description", accessors)
        self.assertIn("id", accessors)

    def test_flattened_resource_module_collision_fails_closed(self):
        api = FakeOpenApi()
        api.operations["create_thing_alias"] = api.operations["list_things"]
        taxonomy = {
            "operations": {
                "list_things": ["a_b.c.list"],
                "create_thing_alias": ["a.b_c.create"],
            }
        }
        with self.assertRaisesRegex(ValueError, "resource module collision"):
            sdk_autoproject.expand_manifest(
                api,
                manifest(),
                taxonomy,
                raw_coverage("list_things", "create_thing_alias"),
            )


    def test_official_binary_transport_selects_mixed_media_binary_variant(self):
        api = FakeOpenApi()
        api.operations["sample_audio"] = {
            "x-sdk-path": "/v1/audio/voices/{voice_id}/sample",
            "x-sdk-method": "get",
            "responses": {"200": {"content": {
                "application/json": {"schema": {"$ref": "#/components/schemas/Thing"}},
                "audio/wav": {"schema": {"type": "string", "format": "binary"}},
            }}},
            "parameters": [],
        }
        coverage = {"operations": [
            {"operation_id": "sample_audio", "rust_method": "sample_audio", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["application/json", "audio/wav"], "upstream": True},
            {"operation_id": "sample_audio_wav", "rust_method": "sample_audio_wav",
             "binary_stream_method": "sample_audio_wav_stream", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["audio/wav"], "upstream": False},
        ]}
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"sample_audio": ["audio.voices.get_sample_audio"]},
             "operation_transports": {"sample_audio": "binary_stream"}},
            coverage,
        )
        self.assertEqual(1, report["added_count"])
        operation = expanded["resources"]["audio_voices"]["operations"]["get_sample_audio"]
        self.assertTrue(operation["binary_response"])
        self.assertEqual("sample_audio_wav_stream", operation["raw_method"])

if __name__ == "__main__":
    unittest.main()
