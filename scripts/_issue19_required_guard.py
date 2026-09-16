from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"patch anchor not found in {path}: {old[:100]!r}")
    target.write_text(text.replace(old, new, 1))


replace(
    "codegen/sdk_autoproject.py",
    '''    properties = schema.get("properties", {})
    if rust is None:
''',
    '''    properties = schema.get("properties", {})
    required = set(schema.get("required", []))
    if not required <= set(properties):
        return None, "request_model_projection"
    if rust is None:
''',
)

replace(
    "scripts/test_sdk_autoproject.py",
    "    def test_projects_nested_request_objects_with_generated_adapters(self):\n",
    '''    def test_rejects_required_field_missing_from_wire_properties(self):
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
            symbol_modules={"CreateThing": "types"},
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
''',
)
