"""Temporary materializer for the #19 structural map-projection increment."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"expected snippet not found in {path}: {old[:120]!r}")
    target.write_text(text.replace(old, new, 1))


# The production pipeline must use the Rust AST for every structural projection.
replace(
    "codegen/sdk_pipeline.py",
    "        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage)\n",
    "        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage, rust)\n",
)

# Keep map wrappers explicit in the closed model-policy algebra.
replace(
    "codegen/sdk_ir.py",
    "@dataclass(frozen=True)\nclass TypeAliasPolicy:\n    pass\n\n\n@dataclass(frozen=True)\nclass RequestPolicy:",
    "@dataclass(frozen=True)\nclass TypeAliasPolicy:\n    pass\n\n\n@dataclass(frozen=True)\nclass MapPolicy:\n    root: str\n    path: tuple[str, ...]\n\n\n@dataclass(frozen=True)\nclass RequestPolicy:",
)
replace(
    "codegen/sdk_ir.py",
    "ModelPolicy = UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | RequestPolicy | ViewPolicy\n",
    "ModelPolicy = UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | MapPolicy | RequestPolicy | ViewPolicy\n",
)
replace(
    "codegen/sdk_ir.py",
    "    if config.get(\"type_alias\"):\n        return TypeAliasPolicy()\n    if \"accessors\" in config:\n",
    "    if config.get(\"type_alias\"):\n        return TypeAliasPolicy()\n    if \"map\" in config:\n        mapping = config[\"map\"]\n        return MapPolicy(mapping[\"root\"], tuple(mapping.get(\"path\", ())))\n    if \"accessors\" in config:\n",
)

# Extend the semantic overlay schema without broadening any existing policy shape.
replace(
    "codegen/sdk-semantics.schema.json",
    '    "path": {"type": "array", "items": {"$ref": "#/$defs/identifier"}, "minItems": 1},\n',
    '    "path": {"type": "array", "items": {"$ref": "#/$defs/identifier"}, "minItems": 1},\n'
    '    "schema_path": {"type": "array", "items": {"$ref": "#/$defs/identifier"}},\n',
)
replace(
    "codegen/sdk-semantics.schema.json",
    '    "factory": {"type": "object", "additionalProperties": false, "required": ["field"], "properties": {\n',
    '    "map": {"type": "object", "additionalProperties": false, "required": ["root", "path"], "properties": {\n'
    '      "root": {"$ref": "#/$defs/identifier"}, "path": {"$ref": "#/$defs/schema_path"}\n'
    '    }},\n'
    '    "factory": {"type": "object", "additionalProperties": false, "required": ["field"], "properties": {\n',
)
replace(
    "codegen/sdk-semantics.schema.json",
    '      "adapters": {"$ref": "#/$defs/mapping"}, "union": {"$ref": "#/$defs/union"}, "simple_union": {"$ref": "#/$defs/simple_union"}, "type_alias": {"const": true}, "union_factory": {"$ref": "#/$defs/factory"},\n',
    '      "adapters": {"$ref": "#/$defs/mapping"}, "union": {"$ref": "#/$defs/union"}, "simple_union": {"$ref": "#/$defs/simple_union"}, "type_alias": {"const": true}, "map": {"$ref": "#/$defs/map"}, "union_factory": {"$ref": "#/$defs/factory"},\n',
)
# Make every policy branch mutually exclusive with map, then add the map branch.
for old, new in (
    ('{"required": ["union"], "not": {"anyOf": [{"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}',
     '{"required": ["union"], "not": {"anyOf": [{"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}'),
    ('{"required": ["simple_union"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["type_alias"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}',
     '{"required": ["simple_union"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["type_alias"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}'),
    ('{"required": ["type_alias"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}',
     '{"required": ["type_alias"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}'),
    ('{"required": ["constructor"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}',
     '{"required": ["constructor"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["map"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}}'),
    ('{"required": ["union_factory"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["constructor"]}, {"required": ["accessors"]}]}}',
     '{"required": ["union_factory"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["accessors"]}]}}'),
    ('{"required": ["accessors"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}]}}',
     '{"required": ["accessors"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}]}}'),
):
    replace("codegen/sdk-semantics.schema.json", old, new)
replace(
    "codegen/sdk-semantics.schema.json",
    '      {"required": ["type_alias"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n',
    '      {"required": ["type_alias"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n'
    '      {"required": ["map"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n',
)

# Auto-project generated additionalProperties structs through a public map newtype.
replace(
    "codegen/sdk_autoproject.py",
    "def _union_name(raw: str) -> str:\n    return f\"{raw}Value\"\n\n\n",
    "def _union_name(raw: str) -> str:\n    return f\"{raw}Value\"\n\n\ndef _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n\n\n",
)
replace(
    "codegen/sdk_autoproject.py",
    "def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:\n",
    '''def _generated_map_wrapper(schema: dict[str, Any], raw: str, rust: Any) -> bool:\n    if raw not in rust.structs:\n        return False\n    fields = rust.fields(raw)\n    if len(fields) != 1 or fields[0].name.removeprefix("r#") != "additional_properties":\n        return False\n    mapping = parse_type(fields[0].type)\n    if mapping.constructor != "std::collections::BTreeMap" or len(mapping.arguments) != 2:\n        return False\n    key, value = mapping.arguments\n    if key.spelling != "String":\n        return False\n    additional = schema.get("additionalProperties")\n    if additional is True:\n        return value.spelling == "serde_json::Value"\n    if not isinstance(additional, dict):\n        return False\n    # serde_json::Value is a faithful carrier for any JSON-valued map contract.\n    if value.spelling == "serde_json::Value":\n        return True\n    additional, _ = _nullable(additional)\n    expected = {"string": "String", "integer": "i64", "number": "f64", "boolean": "bool"}.get(additional.get("type"))\n    return expected == value.spelling\n\n\ndef _ensure_map_model(models: dict[str, Any], schema: dict[str, Any], raw: str, rust: Any,\n                      root: str, path: tuple[str, ...]) -> tuple[str | None, str | None]:\n    if not _generated_map_wrapper(schema, raw, rust):\n        return None, "request_model_projection"\n    for name, config in models.items():\n        if config.get("raw", name) == raw and "map" in config:\n            return name, None\n    name = _map_name(raw)\n    if name in models and models[name].get("raw", name) != raw:\n        return None, "request_model_projection"\n    models[name] = {"raw": raw, "map": {"root": root, "path": list(path)}}\n    return name, None\n\n\ndef _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    "def _request_field_adapter(models: dict[str, Any], schemas: dict[str, Any], schema: dict[str, Any],\n                           raw_type: str, rust: Any, resolving: tuple[str, ...]) -> tuple[str | None, str | None]:\n",
    "def _request_field_adapter(models: dict[str, Any], schemas: dict[str, Any], schema: dict[str, Any],\n                           raw_type: str, rust: Any, resolving: tuple[str, ...],\n                           root: str, path: tuple[str, ...]) -> tuple[str | None, str | None]:\n",
)
replace(
    "codegen/sdk_autoproject.py",
    "        return _request_field_adapter(models, schemas, schema.get(\"items\", {}), inner.spelling, rust, resolving)\n",
    "        return _request_field_adapter(models, schemas, schema.get(\"items\", {}), inner.spelling, rust, resolving, root, (*path, \"items\"))\n",
)
replace(
    "codegen/sdk_autoproject.py",
    '''        if target.get("type") == "object" and target.get("additionalProperties"):\n            if syntax.spelling in rust.aliases:\n                return _ensure_alias_model(models, syntax.spelling, rust)\n            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n''',
    '''        if target.get("type") == "object" and target.get("additionalProperties"):\n            if syntax.spelling in rust.aliases:\n                return _ensure_alias_model(models, syntax.spelling, rust)\n            if _generated_map_wrapper(target, syntax.spelling, rust):\n                return _ensure_map_model(models, target, syntax.spelling, rust, reference, ())\n            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''    if schema.get("type") == "object" and schema.get("additionalProperties"):\n        if syntax.spelling in rust.aliases:\n            return _ensure_alias_model(models, syntax.spelling, rust)\n        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n''',
    '''    if schema.get("type") == "object" and schema.get("additionalProperties"):\n        if syntax.spelling in rust.aliases:\n            return _ensure_alias_model(models, syntax.spelling, rust)\n        if _generated_map_wrapper(schema, syntax.spelling, rust):\n            return _ensure_map_model(models, schema, syntax.spelling, rust, root, path)\n        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''        adapter, reason = _request_field_adapter(\n            models, schemas, field_schema, raw_fields[field].type, rust, (*resolving, raw)\n        )\n''',
    '''        adapter, reason = _request_field_adapter(\n            models, schemas, field_schema, raw_fields[field].type, rust, (*resolving, raw), raw, (field,)\n        )\n''',
)

# Backend: validate map provenance against OpenAPI + raw Rust, and emit the newtype.
replace(
    "codegen/sdk_codegen.py",
    "from sdk_ir import (Accessor, ModelPolicy, RequestPolicy, SimpleUnionPolicy, TypeAliasPolicy,\n                    UnionPolicy, ViewPolicy, StreamPolicy, model_policy, stream_policy)\n",
    "from sdk_ir import (Accessor, MapPolicy, ModelPolicy, RequestPolicy, SimpleUnionPolicy, TypeAliasPolicy,\n                    UnionPolicy, ViewPolicy, StreamPolicy, model_policy, stream_policy)\n",
)
replace(
    "codegen/sdk_codegen.py",
    "    def union(self, root: str, path: Iterable[str]) -> tuple[str, dict[str, str]]:\n        schema = self.schema(root)\n        for segment in path:\n            schema = schema.get(\"items\", {}) if segment == \"items\" else schema.get(\"properties\", {}).get(segment, {})\n            non_null = [part for part in schema.get(\"anyOf\", []) if part.get(\"type\") != \"null\"]\n            if len(non_null) == 1:\n                schema = non_null[0]\n",
    "    def schema_at(self, root: str, path: Iterable[str]) -> dict[str, Any]:\n        schema = self.schema(root)\n        for segment in path:\n            schema = schema.get(\"items\", {}) if segment == \"items\" else schema.get(\"properties\", {}).get(segment, {})\n            non_null = [part for part in schema.get(\"anyOf\", []) if part.get(\"type\") != \"null\"]\n            if len(non_null) == 1 and len(non_null) != len(schema.get(\"anyOf\", [])):\n                schema = non_null[0]\n        return schema\n\n    def union(self, root: str, path: Iterable[str]) -> tuple[str, dict[str, str]]:\n        schema = self.schema_at(root, path)\n",
)
replace(
    "codegen/sdk_codegen.py",
    '''        _validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "union_factory", "accessors", "borrowed"}, f"model {name}")\n''',
    '''        _validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "union_factory", "accessors", "borrowed"}, f"model {name}")\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''        elif "simple_union" in config:\n            configured = set(config["simple_union"]["variants"])\n            actual = {variant.name for variant in rust.variants(raw)}\n            if configured != actual:\n                raise GenerationError(\n                    f"raw union {raw} variant drift: missing={sorted(actual - configured)}, "\n                    f"extra={sorted(configured - actual)}"\n                )\n        else:\n''',
    '''        elif "simple_union" in config:\n            configured = set(config["simple_union"]["variants"])\n            actual = {variant.name for variant in rust.variants(raw)}\n            if configured != actual:\n                raise GenerationError(\n                    f"raw union {raw} variant drift: missing={sorted(actual - configured)}, "\n                    f"extra={sorted(configured - actual)}"\n                )\n        elif config.get("type_alias"):\n            if raw not in rust.aliases:\n                raise GenerationError(f"raw type alias {raw} not found")\n            _public_alias_type(rust.aliases[raw], rust, (raw,))\n        elif "map" in config:\n            _map_contract(ModelSpec(name, raw, model_policy(config)), openapi, rust)\n        else:\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''def _emit_type_alias(model: ModelSpec, rust: RustIndex) -> str:\n    assert isinstance(model.config, TypeAliasPolicy)\n    return f"pub type {model.name} = {_public_alias_type(rust.aliases[model.raw], rust, (model.raw,))};"\n\n\ndef _emit_model(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n''',
    '''def _emit_type_alias(model: ModelSpec, rust: RustIndex) -> str:\n    assert isinstance(model.config, TypeAliasPolicy)\n    return f"pub type {model.name} = {_public_alias_type(rust.aliases[model.raw], rust, (model.raw,))};"\n\n\ndef _map_contract(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n    assert isinstance(model.config, MapPolicy)\n    schema = openapi.schema_at(model.config.root, model.config.path)\n    if schema.get("type") != "object" or schema.get("properties") or not schema.get("additionalProperties"):\n        raise GenerationError(\n            f"map policy {model.name} does not point to an additionalProperties object"\n        )\n    fields = rust.fields(model.raw)\n    if len(fields) != 1 or fields[0].name.removeprefix("r#") != "additional_properties":\n        raise GenerationError(f"raw map wrapper drift for {model.raw}")\n    mapping = fields[0].syntax\n    if mapping.constructor != "std::collections::BTreeMap" or len(mapping.arguments) != 2:\n        raise GenerationError(f"raw map wrapper {model.raw} is not a BTreeMap")\n    key, value = mapping.arguments\n    if key.spelling != "String":\n        raise GenerationError(f"raw map wrapper {model.raw} does not use String keys")\n    additional = schema["additionalProperties"]\n    if additional is True:\n        compatible = value.spelling == "serde_json::Value"\n    elif value.spelling == "serde_json::Value":\n        compatible = isinstance(additional, dict)\n    elif isinstance(additional, dict):\n        non_null = [part for part in additional.get("anyOf", []) if part.get("type") != "null"]\n        normalized = non_null[0] if len(non_null) == 1 and len(non_null) != len(additional.get("anyOf", [])) else additional\n        expected = {"string": "String", "integer": "i64", "number": "f64", "boolean": "bool"}.get(normalized.get("type"))\n        compatible = expected == value.spelling\n    else:\n        compatible = False\n    if not compatible:\n        raise GenerationError(\n            f"OpenAPI/raw map value drift for {model.raw}: {value.spelling}"\n        )\n    value_type = _public_alias_type(value, rust)\n    return f"std::collections::BTreeMap<String, {value_type}>"\n\n\ndef _emit_map(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n    assert isinstance(model.config, MapPolicy)\n    map_type = _map_contract(model, openapi, rust)\n    return (\n        f"#[derive(Debug, Clone, Default)]\\npub struct {model.name} {{ values: {map_type} }}\\n\\n"\n        f"impl {model.name} {{\\n"\n        f"    pub fn new(values: {map_type}) -> Self {{ Self {{ values }} }}\\n"\n        f"    pub fn as_map(&self) -> &{map_type} {{ &self.values }}\\n"\n        f"    pub fn into_map(self) -> {map_type} {{ self.values }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{map_type}> for {model.name} {{\\n"\n        f"    fn from(values: {map_type}) -> Self {{ Self {{ values }} }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{model.raw}> for {model.name} {{\\n"\n        f"    fn from(value: {model.raw}) -> Self {{ Self {{ values: value.additional_properties }} }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{model.name}> for {model.raw} {{\\n"\n        f"    fn from(value: {model.name}) -> Self {{ Self {{ additional_properties: value.values }} }}\\n"\n        f"}}"\n    )\n\n\ndef _emit_model(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''    if isinstance(model.config, TypeAliasPolicy):\n        return _emit_type_alias(model, rust)\n    if isinstance(model.config, ViewPolicy):\n''',
    '''    if isinstance(model.config, TypeAliasPolicy):\n        return _emit_type_alias(model, rust)\n    if isinstance(model.config, MapPolicy):\n        return _emit_map(model, openapi, rust)\n    if isinstance(model.config, ViewPolicy):\n''',
)

# Focused auto-projection test: generated map struct becomes a safe facade adapter.
test_path = ROOT / "scripts/test_sdk_autoproject.py"
test_text = test_path.read_text()
marker = "    def test_projects_empty_success_without_fake_response_model(self):\n"
if marker not in test_text:
    raise SystemExit("autoproject insertion marker missing")
addition = '''    def test_projects_generated_inline_map_wrapper_without_raw_type_leak(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["metadata"] = {\n            "type": "object", "additionalProperties": True,\n        }\n        fields = {\n            "CreateThing": (\n                SimpleNamespace(name="name", type="String"),\n                SimpleNamespace(name="enabled", type="Option<bool>"),\n                SimpleNamespace(name="metadata", type="Option<MetadataMapRaw>"),\n            ),\n            "MetadataMapRaw": (\n                SimpleNamespace(\n                    name="additional_properties",\n                    type="std::collections::BTreeMap<String, serde_json::Value>",\n                ),\n            ),\n        }\n        rust = SimpleNamespace(\n            structs=set(fields), aliases={}, enums={},\n            symbol_modules={name: "types" for name in fields},\n            fields=lambda name: fields[name],\n        )\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"), rust,\n        )\n        self.assertEqual(1, report["added_count"])\n        self.assertEqual(\n            {"metadata": "MetadataMapRawMap"},\n            expanded["models"]["CreateThingParams"]["adapters"],\n        )\n        self.assertEqual(\n            {"root": "CreateThing", "path": ["metadata"]},\n            expanded["models"]["MetadataMapRawMap"]["map"],\n        )\n\n    def test_generated_inline_map_wrapper_drift_fails_closed(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["metadata"] = {\n            "type": "object", "additionalProperties": True,\n        }\n        fields = {\n            "CreateThing": (\n                SimpleNamespace(name="name", type="String"),\n                SimpleNamespace(name="enabled", type="Option<bool>"),\n                SimpleNamespace(name="metadata", type="Option<MetadataMapRaw>"),\n            ),\n            "MetadataMapRaw": (SimpleNamespace(name="value", type="serde_json::Value"),),\n        }\n        rust = SimpleNamespace(\n            structs=set(fields), aliases={}, enums={},\n            symbol_modules={name: "types" for name in fields},\n            fields=lambda name: fields[name],\n        )\n        _, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"), rust,\n        )\n        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])\n\n'''
test_path.write_text(test_text.replace(marker, addition + marker, 1))

# Generic compiler test: MapPolicy validates provenance and emits only the public map shape.
test_path = ROOT / "scripts/test_sdk_facade.py"
test_text = test_path.read_text()
marker = "    def test_unrelated_api_uses_the_same_generic_emitters(self):\n"
if marker not in test_text:
    raise SystemExit("facade insertion marker missing")
addition = '''    def test_generated_map_wrapper_emits_public_map_newtype(self):\n        document = openapi_document()\n        document["components"]["schemas"]["CreateThing"] = {\n            "type": "object",\n            "properties": {"metadata": {"type": "object", "additionalProperties": True}},\n        }\n        source = TYPES + "\\npub struct MetadataMapRaw { pub additional_properties: std::collections::BTreeMap<String, serde_json::Value> }\\n"\n        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())\n        model = sdk_codegen.ModelSpec(\n            "MetadataMap", "MetadataMapRaw",\n            sdk_codegen.model_policy({"map": {"root": "CreateThing", "path": ["metadata"]}}),\n        )\n        emitted = sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(document), rust)\n        self.assertIn("pub struct MetadataMap { values: std::collections::BTreeMap<String, serde_json::Value> }", emitted)\n        self.assertIn("impl From<std::collections::BTreeMap<String, serde_json::Value>> for MetadataMap", emitted)\n        self.assertIn("additional_properties: value.values", emitted)\n\n    def test_generated_map_wrapper_value_drift_fails_closed(self):\n        document = openapi_document()\n        document["components"]["schemas"]["CreateThing"] = {\n            "type": "object",\n            "properties": {"metadata": {"type": "object", "additionalProperties": True}},\n        }\n        source = TYPES + "\\npub struct MetadataMapRaw { pub additional_properties: std::collections::BTreeMap<String, String> }\\n"\n        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())\n        model = sdk_codegen.ModelSpec(\n            "MetadataMap", "MetadataMapRaw",\n            sdk_codegen.model_policy({"map": {"root": "CreateThing", "path": ["metadata"]}}),\n        )\n        with self.assertRaisesRegex(sdk_codegen.GenerationError, "map value drift"):\n            sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(document), rust)\n\n'''
test_path.write_text(test_text.replace(marker, addition + marker, 1))
