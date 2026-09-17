import sys
from pathlib import Path
import unittest
from types import SimpleNamespace

from rust_sdk_compiler import OpenApi, parse_type

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tooling" / "pipeline" / "mistral"))
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

    def test_projects_composed_request_and_omits_optional_singleton_scalar(self):
        api = OpenApi({
            "openapi": "3.1.0",
            "info": {"title": "fixture", "version": "1"},
            "paths": {
                "/things": {
                    "post": {
                        "operationId": "create_thing",
                        "requestBody": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/CreateThing"}}}},
                        "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                    }
                }
            },
            "components": {"schemas": {
                "Thing": {"type": "object", "properties": {"id": {"type": "string"}}},
                "CreateThingBase": {
                    "type": "object",
                    "additionalProperties": False,
                    "properties": {"name": {"type": "string"}},
                    "required": ["name"],
                },
                "CreateThing": {
                    "allOf": [
                        {"$ref": "#/components/schemas/CreateThingBase"},
                        {"type": "object", "properties": {"stream": {"type": "boolean", "enum": [False]}}},
                    ]
                },
            }},
        })
        fields = {"CreateThing": (
            SimpleNamespace(name="name", type="String"),
            SimpleNamespace(name="stream", type="Option<bool>"),
        )}
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums={},
            symbol_paths={"CreateThing": "types"},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        request = expanded["models"]["CreateThingParams"]
        self.assertEqual(["name"], request["constructor"])
        self.assertEqual(["stream"], request["exclude"])

    def test_projects_named_scalar_enum_without_raw_type_leak(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"] = {
            "type": "object", "required": ["visibility"],
            "properties": {
                "visibility": {"$ref": "#/components/schemas/ResourceVisibility"},
            },
        }
        api.schemas["ResourceVisibility"] = {
            "type": "string", "enum": ["shared_global", "private"],
        }
        fields = {"CreateThing": (
            SimpleNamespace(name="visibility", type="ResourceVisibility"),
        )}
        enums = {"ResourceVisibility": (
            SimpleNamespace(name="SharedGlobal", payload=None, wire_name="shared_global"),
            SimpleNamespace(name="Private", payload=None, wire_name="private"),
        )}
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums=enums,
            symbol_paths={"CreateThing": "types", "ResourceVisibility": "types"},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        self.assertEqual(
            {"visibility": "ResourceVisibilityValue"},
            expanded["models"]["CreateThingParams"]["adapters"],
        )
        self.assertEqual(
            {"raw": "ResourceVisibility",
             "scalar_enum": {"root": "ResourceVisibility", "path": []}},
            expanded["models"]["ResourceVisibilityValue"],
        )

    def test_scalar_enum_projection_fails_closed_on_wire_drift(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"] = {
            "type": "object", "required": ["visibility"],
            "properties": {
                "visibility": {"$ref": "#/components/schemas/ResourceVisibility"},
            },
        }
        api.schemas["ResourceVisibility"] = {"type": "string", "enum": ["private"]}
        fields = {"CreateThing": (
            SimpleNamespace(name="visibility", type="ResourceVisibility"),
        )}
        enums = {"ResourceVisibility": (
            SimpleNamespace(name="Private", payload=None, wire_name="shared_global"),
        )}
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums=enums,
            symbol_paths={"CreateThing": "types", "ResourceVisibility": "types"},
            fields=lambda name: fields[name],
        )
        _, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])

    def test_rejects_required_field_missing_from_wire_properties(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"]["required"] = ["name", "missing"]
        fields = {
            "CreateThing": (
                SimpleNamespace(name="name", type="String"),
                SimpleNamespace(name="enabled", type="Option<bool>"),
            ),
        }
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums={},
            symbol_paths={"CreateThing": "types"},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])
        self.assertNotIn("CreateThingParams", expanded["models"])

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
            structs=set(fields), aliases={}, symbol_paths={name: "types" for name in fields},
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

    def test_projects_mixed_primitive_and_referenced_request_unions(self):
        api = FakeOpenApi()
        api.schemas.update({
            "CreateThing": {
                "type": "object", "required": ["input"],
                "properties": {
                    "input": {"$ref": "#/components/schemas/InputChoice"},
                    "version": {
                        "anyOf": [
                            {"type": "string"},
                            {"type": "integer"},
                            {"type": "null"},
                        ]
                    },
                },
            },
            "InputChoice": {
                "anyOf": [
                    {"type": "string"},
                    {"$ref": "#/components/schemas/InputEntries"},
                ]
            },
            "InputEntries": {
                "type": "array",
                "items": {
                    "oneOf": [
                        {"$ref": "#/components/schemas/TextInput"},
                        {"$ref": "#/components/schemas/NumberInput"},
                    ]
                },
            },
            "TextInput": {
                "type": "object", "required": ["text"],
                "properties": {"text": {"type": "string"}},
            },
            "NumberInput": {
                "type": "object", "required": ["value"],
                "properties": {"value": {"type": "integer"}},
            },
        })
        fields = {
            "CreateThing": (
                SimpleNamespace(name="input", type="InputChoice"),
                SimpleNamespace(name="version", type="Option<Option<VersionChoice>>"),
            ),
            "TextInput": (SimpleNamespace(name="text", type="String"),),
            "NumberInput": (SimpleNamespace(name="value", type="i64"),),
        }
        enums = {
            "InputChoice": (
                SimpleNamespace(name="String", payload="String"),
                SimpleNamespace(name="InputEntries", payload="InputEntries"),
            ),
            "InputEntryUnion": (
                SimpleNamespace(name="TextInput", payload="TextInput"),
                SimpleNamespace(name="NumberInput", payload="NumberInput"),
            ),
            "VersionChoice": (
                SimpleNamespace(name="String", payload="String"),
                SimpleNamespace(name="Integer", payload="i64"),
            ),
        }
        aliases = {"InputEntries": parse_type("Vec<InputEntryUnion>")}
        rust = SimpleNamespace(
            structs=set(fields), aliases=aliases, enums=enums,
            symbol_paths={name: "types" for name in (*fields, *enums, *aliases)},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        request = expanded["models"]["CreateThingParams"]
        self.assertEqual(
            {"input": "InputChoiceValue", "version": "VersionChoiceValue"},
            request["adapters"],
        )
        self.assertEqual(
            {
                "String": "String",
                "InputEntries": {"name": "InputEntries", "adapter": "InputEntryUnionValue"},
            },
            expanded["models"]["InputChoiceValue"]["simple_union"]["variants"],
        )
        self.assertEqual(
            {"String": "String", "Integer": "Integer"},
            expanded["models"]["VersionChoiceValue"]["simple_union"]["variants"],
        )

    def test_projects_inline_simple_array_request_union(self):
        api = FakeOpenApi()
        api.schemas.update({
            "CreateThing": {
                "type": "object", "required": ["stop"],
                "properties": {"stop": {"$ref": "#/components/schemas/StopChoice"}},
            },
            "StopChoice": {
                "anyOf": [
                    {"type": "string"},
                    {"type": "array", "items": {"type": "string"}},
                    {"type": "null"},
                ]
            },
        })
        fields = {"CreateThing": (SimpleNamespace(name="stop", type="StopChoice"),)}
        enums = {
            "StopChoice": (
                SimpleNamespace(name="String", payload="String"),
                SimpleNamespace(name="StringArray", payload="StopStringArray"),
            ),
        }
        aliases = {"StopStringArray": parse_type("Vec<String>")}
        rust = SimpleNamespace(
            structs=set(fields), aliases=aliases, enums=enums,
            symbol_paths={name: "types" for name in (*fields, *enums, *aliases)},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        self.assertEqual(
            {"stop": "StopChoiceValue"},
            expanded["models"]["CreateThingParams"]["adapters"],
        )
        self.assertEqual(
            {"String": "String", "StringArray": "Array"},
            expanded["models"]["StopChoiceValue"]["simple_union"]["variants"],
        )

    def test_projects_discriminated_request_union_without_raw_type_leaks(self):
        api = FakeOpenApi()
        api.schemas.update({
            "CreateThing": {
                "type": "object", "required": ["name", "mode"],
                "properties": {
                    "name": {"type": "string"},
                    "mode": {
                        "oneOf": [
                            {"$ref": "#/components/schemas/FastMode"},
                            {"$ref": "#/components/schemas/SafeMode"},
                        ],
                        "discriminator": {
                            "propertyName": "type",
                            "mapping": {
                                "fast": "#/components/schemas/FastMode",
                                "safe": "#/components/schemas/SafeMode",
                            },
                        },
                    },
                },
            },
            "FastMode": {
                "type": "object", "required": ["value"],
                "properties": {
                    "value": {"type": "string"},
                    "type": {"$ref": "#/components/schemas/FastModeType"},
                },
            },
            "SafeMode": {
                "type": "object", "required": ["value"],
                "properties": {
                    "value": {"type": "integer"},
                    "type": {"$ref": "#/components/schemas/SafeModeType"},
                },
            },
            "FastModeType": {"type": "string", "enum": ["fast"]},
            "SafeModeType": {"type": "string", "enum": ["safe"]},
        })
        fields = {
            "CreateThing": (
                SimpleNamespace(name="name", type="String"),
                SimpleNamespace(name="mode", type="ModeUnion"),
            ),
            "FastMode": (
                SimpleNamespace(name="value", type="String"),
                SimpleNamespace(name="type", type="Option<FastModeType>"),
            ),
            "SafeMode": (
                SimpleNamespace(name="value", type="i64"),
                SimpleNamespace(name="type", type="Option<SafeModeType>"),
            ),
        }
        enums = {
            "ModeUnion": (
                SimpleNamespace(name="FastMode", payload="FastMode"),
                SimpleNamespace(name="SafeMode", payload="SafeMode"),
            ),
            "FastModeType": (SimpleNamespace(name="Fast", payload=None),),
            "SafeModeType": (SimpleNamespace(name="Safe", payload=None),),
        }
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums=enums,
            symbol_paths={name: "types" for name in (*fields, *enums)},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        self.assertEqual(
            {"mode": "ModeUnionValue"},
            expanded["models"]["CreateThingParams"]["adapters"],
        )
        self.assertEqual(["type"], expanded["models"]["FastModeParams"]["exclude"])
        self.assertEqual(["type"], expanded["models"]["SafeModeParams"]["exclude"])
        variants = expanded["models"]["ModeUnionValue"]["simple_union"]["variants"]
        self.assertEqual({
            "FastMode": {"name": "Fast", "adapter": "FastModeParams"},
            "SafeMode": {"name": "Safe", "adapter": "SafeModeParams"},
        }, variants)

    def test_projects_generated_map_wrapper_without_leaking_raw_struct(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"]["properties"]["metadata"] = {
            "type": "object", "additionalProperties": True,
        }
        fields = {
            "CreateThing": (
                SimpleNamespace(name="name", type="String"),
                SimpleNamespace(name="enabled", type="Option<bool>"),
                SimpleNamespace(name="metadata", type="Option<CreateThingMetadata>"),
            ),
            "CreateThingMetadata": (
                SimpleNamespace(
                    name="additional_properties",
                    type="std::collections::BTreeMap<String, serde_json::Value>",
                ),
            ),
        }
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums={},
            symbol_paths={name: "types" for name in fields},
            fields=lambda name: fields[name],
        )
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual(1, report["added_count"])
        request = expanded["models"]["CreateThingParams"]
        self.assertEqual({"metadata": "CreateThingMetadataMap"}, request["adapters"])
        self.assertEqual(
            {"root": "CreateThing", "path": ["metadata"]},
            expanded["models"]["CreateThingMetadataMap"]["map"],
        )

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
            symbol_paths={"CreateThing": "types", "MetadataDict": "types"},
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

    def test_json_transport_resolves_stream_convenience_alias(self):
        expanded, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {"list_things": ["things.complete", "things.stream"]}},
            raw_coverage("list_things"),
        )
        self.assertEqual(1, report["added_count"])
        self.assertIn("complete", expanded["resources"]["things"]["operations"])

    def test_missing_official_taxonomy_uses_openapi_tag_fallback(self):
        api = FakeOpenApi()
        api.operations["list_things"]["tags"] = ["beta.things"]
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {}}, raw_coverage("list_things"),
        )
        self.assertEqual(1, report["added_count"])
        self.assertIn("list_things", expanded["resources"]["beta_things"]["operations"])

    def test_fallback_inherits_known_parent_resource_mapping(self):
        api = FakeOpenApi()
        api.operations["list_things"]["tags"] = ["beta.workflows"]
        api.operations["worker_info"] = {
            "tags": ["beta.workflows.workers"],
            "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
            "parameters": [],
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"list_things": ["workflows.list"]}},
            raw_coverage("list_things", "worker_info"),
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn("worker_info", expanded["resources"]["workflows_workers"]["operations"])

    def test_fallback_strips_only_nonsemantic_common_operation_prefix(self):
        api = FakeOpenApi()
        api.operations = {
            "connector_list_v1": {
                "tags": ["beta.connectors"],
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
            "connector_create_or_update_credentials_v1": {
                "tags": ["beta.connectors"],
                "responses": {"200": {"content": {"application/json": {"schema": {"$ref": "#/components/schemas/Thing"}}}}},
                "parameters": [],
            },
        }
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"connector_list_v1": ["beta.connectors.list"]}},
            raw_coverage("connector_list_v1", "connector_create_or_update_credentials_v1"),
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn(
            "create_or_update_credentials",
            expanded["resources"]["beta_connectors"]["operations"],
        )

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
