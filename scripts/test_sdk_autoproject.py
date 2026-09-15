import sys
from pathlib import Path
import unittest

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


class AutoProjectionTests(unittest.TestCase):
    def test_projects_official_nested_taxonomy(self):
        expanded, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {
                "list_things": ["beta.things.list"],
                "create_thing": ["beta.things.create"],
            }},
            {"operations": [
                {"operation_id": "list_things", "rust_method": "list_things", "upstream": True},
                {"operation_id": "create_thing", "rust_method": "create_thing", "upstream": True},
            ]},
        )
        self.assertEqual(2, report["added_count"])
        self.assertIn("beta", expanded["resources"])
        self.assertEqual(["beta", "things"], expanded["resources"]["beta__things"]["path"])
        self.assertIn("list", expanded["resources"]["beta__things"]["operations"])
        self.assertIn("create", expanded["resources"]["beta__things"]["operations"])
        self.assertEqual("CreateThingParams", expanded["resources"]["beta__things"]["operations"]["create"]["request"])
        self.assertEqual("ThingView", expanded["resources"]["beta__things"]["operations"]["list"]["response"])

    def test_complex_request_is_review_debt_not_raw_type_leak(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"]["properties"]["tool"] = {"$ref": "#/components/schemas/Tool"}
        _, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"create_thing": ["things.create"]}},
            {"operations": [{"operation_id": "create_thing", "rust_method": "create_thing", "upstream": True}]},
        )
        self.assertEqual(0, report["added_count"])
        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])

    def test_equal_depth_aliases_require_review(self):
        _, report = sdk_autoproject.expand_manifest(
            FakeOpenApi(), manifest(),
            {"operations": {"list_things": ["things.list", "items.list"]}},
            {"operations": [{"operation_id": "list_things", "rust_method": "list_things", "upstream": True}]},
        )
        self.assertEqual("taxonomy_alias_ambiguity", report["rejected"]["list_things"])


if __name__ == "__main__":
    unittest.main()
