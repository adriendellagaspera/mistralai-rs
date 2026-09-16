"""Temporary materializer for the #19 request-map projection increment."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"expected snippet not found in {path}: {old[:160]!r}")
    target.write_text(text.replace(old, new, 1))


# Closed semantic policy + resolved render IR.
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
    "@dataclass(frozen=True)\nclass AliasModelSpec:\n    public_type: str\n\n\nModelRenderSpec = (WrapperModelSpec | UnionModelSpec | SimpleUnionModelSpec |\n                   ViewModelSpec | AliasModelSpec)\n",
    "@dataclass(frozen=True)\nclass AliasModelSpec:\n    public_type: str\n\n\n@dataclass(frozen=True)\nclass MapModelSpec:\n    public_type: str\n    raw_field: str\n\n\nModelRenderSpec = (WrapperModelSpec | UnionModelSpec | SimpleUnionModelSpec |\n                   ViewModelSpec | AliasModelSpec | MapModelSpec)\n",
)
replace(
    "codegen/sdk_ir.py",
    "    if config.get(\"type_alias\"):\n        return TypeAliasPolicy()\n    if \"accessors\" in config:\n",
    "    if config.get(\"type_alias\"):\n        return TypeAliasPolicy()\n    if \"map\" in config:\n        mapping = config[\"map\"]\n        return MapPolicy(mapping[\"root\"], tuple(mapping.get(\"path\", ())))\n    if \"accessors\" in config:\n",
)

# Semantic overlay schema: map root may itself be the map, so its path can be empty.
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

# The validated frontend must accept the new policy key. Shape reconciliation happens in model lowering.
replace(
    "codegen/sdk_codegen.py",
    '_validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "union_factory", "accessors", "borrowed"}, f"model {name}")',
    '_validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "union_factory", "accessors", "borrowed"}, f"model {name}")',
)

# Auto-project only the generated one-field additionalProperties wrapper shape.
replace(
    "codegen/sdk_autoproject.py",
    "def _union_name(raw: str) -> str:\n    return f\"{raw}Value\"\n\n\n",
    "def _union_name(raw: str) -> str:\n    return f\"{raw}Value\"\n\n\ndef _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n\n\n",
)
replace(
    "codegen/sdk_autoproject.py",
    "def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:\n",
    '''def _generated_map_wrapper(schema: dict[str, Any], raw: str, rust: Any) -> bool:\n    if raw not in rust.structs:\n        return False\n    fields = rust.fields(raw)\n    if len(fields) != 1 or fields[0].name.removeprefix("r#") != "additional_properties":\n        return False\n    mapping = parse_type(fields[0].type)\n    if mapping.constructor != "std::collections::BTreeMap" or len(mapping.arguments) != 2:\n        return False\n    key, value = mapping.arguments\n    if key.spelling != "String":\n        return False\n    additional = schema.get("additionalProperties")\n    if additional is True:\n        return value.spelling == "serde_json::Value"\n    if not isinstance(additional, dict):\n        return False\n    if value.spelling == "serde_json::Value":\n        return True\n    additional, _ = _nullable(additional)\n    expected = {"string": "String", "integer": "i64", "number": "f64", "boolean": "bool"}.get(additional.get("type"))\n    return expected == value.spelling\n\n\ndef _ensure_map_model(models: dict[str, Any], schema: dict[str, Any], raw: str, rust: Any,\n                      root: str, path: tuple[str, ...]) -> tuple[str | None, str | None]:\n    if not _generated_map_wrapper(schema, raw, rust):\n        return None, "request_model_projection"\n    for name, config in models.items():\n        if config.get("raw", name) == raw and "map" in config:\n            return name, None\n    name = _map_name(raw)\n    if name in models and models[name].get("raw", name) != raw:\n        return None, "request_model_projection"\n    models[name] = {"raw": raw, "map": {"root": root, "path": list(path)}}\n    return name, None\n\n\ndef _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:\n''',
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
    '''        adapter, reason = _request_field_adapter(\n            models, schemas, field_schema, raw_fields[field].type, rust, (*resolving, raw),\n            raw, (field,)\n        )\n''',
)

# Resolve source contracts into a source-agnostic map render spec.
replace(
    "codegen/sdk_model_lowering.py",
    "from sdk_ir import (AccessorKind, AliasModelSpec, ArgumentKind, ArgumentSpec,\n                    CollectIntoValue, ConstructorSpec, EnumValue, FactorySpec,\n                    FacadeIr, IntoModelValue, IntoStringValue, LiteralValue,\n                    MapIntoValue, ModelSpec, RequestPolicy, ResolvedAccessor,\n",
    "from sdk_ir import (AccessorKind, AliasModelSpec, ArgumentKind, ArgumentSpec,\n                    CollectIntoValue, ConstructorSpec, EnumValue, FactorySpec,\n                    FacadeIr, IntoModelValue, IntoStringValue, LiteralValue,\n                    MapIntoValue, MapModelSpec, MapPolicy, ModelSpec, RequestPolicy, ResolvedAccessor,\n",
)
replace(
    "codegen/sdk_model_lowering.py",
    "def _resolve_model(model: ModelSpec, openapi, raw_index):\n",
    '''def _unwrap_nullable_schema(schema):\n    branches = schema.get("anyOf", [])\n    non_null = [branch for branch in branches if branch.get("type") != "null"]\n    return non_null[0] if len(non_null) == 1 and len(non_null) != len(branches) else schema\n\n\ndef _schema_at(openapi, root: str, path: tuple[str, ...]):\n    schema = openapi.schema(root)\n    for segment in path:\n        schema = _unwrap_nullable_schema(schema)\n        schema = (schema.get("items", {}) if segment == "items"\n                  else schema.get("properties", {}).get(segment, {}))\n    return _unwrap_nullable_schema(schema)\n\n\ndef _resolve_map(model: ModelSpec, openapi, raw_index) -> MapModelSpec:\n    assert isinstance(model.config, MapPolicy)\n    schema = _schema_at(openapi, model.config.root, model.config.path)\n    additional = schema.get("additionalProperties")\n    if schema.get("type") != "object" or not additional:\n        raise ModelLoweringError(\n            f"map policy {model.name} does not resolve to additionalProperties"\n        )\n    fields = raw_index.fields(model.raw)\n    if len(fields) != 1 or fields[0].name.removeprefix("r#") != "additional_properties":\n        raise ModelLoweringError(\n            f"raw map wrapper {model.raw} must contain only additional_properties"\n        )\n    field = fields[0]\n    mapping = parse_type(field.type)\n    if mapping.constructor != "std::collections::BTreeMap" or len(mapping.arguments) != 2:\n        raise ModelLoweringError(\n            f"raw map wrapper {model.raw}.{field.name} is not a BTreeMap"\n        )\n    key, value = mapping.arguments\n    if key.spelling != "String":\n        raise ModelLoweringError(f"raw map wrapper {model.raw} has non-String keys")\n    effective = _expand_alias(value, raw_index)\n    if effective.spelling != "serde_json::Value":\n        if additional is True or not isinstance(additional, dict):\n            raise ModelLoweringError(\n                f"raw map value drift for {model.raw}: {effective.spelling}"\n            )\n        additional = _unwrap_nullable_schema(additional)\n        expected = {\n            "string": "String", "integer": "i64", "number": "f64", "boolean": "bool",\n        }.get(additional.get("type"))\n        if expected != effective.spelling:\n            raise ModelLoweringError(\n                f"raw map value drift for {model.raw}: {effective.spelling} != {expected}"\n            )\n    return MapModelSpec(_public_alias_type(mapping, raw_index), field.name)\n\n\ndef _resolve_model(model: ModelSpec, openapi, raw_index):\n''',
)
replace(
    "codegen/sdk_model_lowering.py",
    "    if isinstance(model.config, TypeAliasPolicy):\n        return AliasModelSpec(\n            _public_alias_type(raw_index.aliases[model.raw], raw_index, (model.raw,))\n        )\n    if isinstance(model.config, ViewPolicy):\n",
    "    if isinstance(model.config, TypeAliasPolicy):\n        return AliasModelSpec(\n            _public_alias_type(raw_index.aliases[model.raw], raw_index, (model.raw,))\n        )\n    if isinstance(model.config, MapPolicy):\n        return _resolve_map(model, openapi, raw_index)\n    if isinstance(model.config, ViewPolicy):\n",
)

# Pure emitter: no OpenAPI/raw inspection, only the resolved map spec.
replace(
    "codegen/sdk_emit.py",
    "                    FacadeIr, IntoModelValue, IntoStringValue, JsonResponse,\n                    LiteralValue, MapIntoValue, ModelSpec, OperationSpec,\n",
    "                    FacadeIr, IntoModelValue, IntoStringValue, JsonResponse,\n                    LiteralValue, MapIntoValue, MapModelSpec, ModelSpec, OperationSpec,\n",
)
replace(
    "codegen/sdk_emit.py",
    "def emit_model(model: ModelSpec) -> str:\n",
    '''def _emit_map(model: ModelSpec, spec: MapModelSpec) -> str:\n    return (\n        f"#[derive(Debug, Clone, Default)]\\npub struct {model.name} {{ values: {spec.public_type} }}\\n\\n"\n        f"impl {model.name} {{\\n"\n        f"    pub fn new(values: {spec.public_type}) -> Self {{ Self {{ values }} }}\\n"\n        f"    pub fn as_map(&self) -> &{spec.public_type} {{ &self.values }}\\n"\n        f"    pub fn into_map(self) -> {spec.public_type} {{ self.values }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{spec.public_type}> for {model.name} {{\\n"\n        f"    fn from(values: {spec.public_type}) -> Self {{ Self {{ values }} }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{model.raw}> for {model.name} {{\\n"\n        f"    fn from(value: {model.raw}) -> Self {{ Self {{ values: value.{spec.raw_field} }} }}\\n"\n        f"}}\\n\\n"\n        f"impl From<{model.name}> for {model.raw} {{\\n"\n        f"    fn from(value: {model.name}) -> Self {{ Self {{ {spec.raw_field}: value.values }} }}\\n"\n        f"}}"\n    )\n\n\ndef emit_model(model: ModelSpec) -> str:\n''',
)
replace(
    "codegen/sdk_emit.py",
    "    if isinstance(spec, AliasModelSpec):\n        return f\"pub type {model.name} = {spec.public_type};\"\n",
    "    if isinstance(spec, AliasModelSpec):\n        return f\"pub type {model.name} = {spec.public_type};\"\n    if isinstance(spec, MapModelSpec):\n        return _emit_map(model, spec)\n",
)

# Structural frontend test.
replace(
    "scripts/test_sdk_autoproject.py",
    "    def test_projects_safe_map_alias_without_leaking_generated_alias_name(self):\n",
    '''    def test_projects_generated_map_wrapper_without_leaking_raw_struct(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["metadata"] = {\n            "type": "object", "additionalProperties": True,\n        }\n        fields = {\n            "CreateThing": (\n                SimpleNamespace(name="name", type="String"),\n                SimpleNamespace(name="enabled", type="Option<bool>"),\n                SimpleNamespace(name="metadata", type="Option<CreateThingMetadata>"),\n            ),\n            "CreateThingMetadata": (\n                SimpleNamespace(\n                    name="additional_properties",\n                    type="std::collections::BTreeMap<String, serde_json::Value>",\n                ),\n            ),\n        }\n        rust = SimpleNamespace(\n            structs=set(fields), aliases={}, enums={},\n            symbol_modules={name: "types" for name in fields},\n            fields=lambda name: fields[name],\n        )\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"), rust,\n        )\n        self.assertEqual(1, report["added_count"])\n        request = expanded["models"]["CreateThingParams"]\n        self.assertEqual({"metadata": "CreateThingMetadataMap"}, request["adapters"])\n        self.assertEqual(\n            {"root": "CreateThing", "path": ["metadata"]},\n            expanded["models"]["CreateThingMetadataMap"]["map"],\n        )\n\n    def test_projects_safe_map_alias_without_leaking_generated_alias_name(self):\n''',
)

# Lowering + pure emitter tests on a hand-built map policy.
replace(
    "scripts/test_sdk_emit.py",
    "import sdk_emit\nimport sdk_pipeline\nfrom sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, ModelSpec, NoRequest,\n",
    "import sdk_emit\nimport sdk_model_lowering\nimport sdk_pipeline\nfrom sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, MapModelSpec, MapPolicy, ModelSpec, NoRequest,\n",
)
replace(
    "scripts/test_sdk_emit.py",
    "    def test_renderer_emits_hand_built_ir_without_source_contracts(self):\n",
    '''    def test_map_policy_lowers_to_source_agnostic_render_spec(self):\n        rust = sdk_codegen.RustIndex(\n            b"pub struct RawMetadata { pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>, }",\n            b"impl HttpClient {}",\n        )\n        openapi = sdk_codegen.OpenApiIndex({\n            "openapi": "3.1.0", "paths": {},\n            "components": {"schemas": {"Root": {\n                "type": "object",\n                "properties": {"metadata": {"type": "object", "additionalProperties": True}},\n            }}},\n        })\n        model = ModelSpec("Metadata", "RawMetadata", MapPolicy("Root", ("metadata",)))\n        resolved = sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)\n        spec = resolved.models[0].render\n        self.assertIsInstance(spec, MapModelSpec)\n        self.assertEqual(\n            "std::collections::BTreeMap<String, serde_json::Value>", spec.public_type\n        )\n        source = sdk_emit.emit_model(resolved.models[0])\n        self.assertIn("pub struct Metadata { values: std::collections::BTreeMap<String, serde_json::Value> }", source)\n        self.assertIn("value.additional_properties", source)\n\n    def test_map_policy_fails_closed_on_raw_shape_drift(self):\n        rust = sdk_codegen.RustIndex(\n            b"pub struct RawMetadata { pub values: std::collections::BTreeMap<String, serde_json::Value>, }",\n            b"impl HttpClient {}",\n        )\n        openapi = sdk_codegen.OpenApiIndex({\n            "openapi": "3.1.0", "paths": {},\n            "components": {"schemas": {"Root": {\n                "type": "object",\n                "properties": {"metadata": {"type": "object", "additionalProperties": True}},\n            }}},\n        })\n        model = ModelSpec("Metadata", "RawMetadata", MapPolicy("Root", ("metadata",)))\n        with self.assertRaisesRegex(\n            sdk_model_lowering.ModelLoweringError, "must contain only additional_properties"\n        ):\n            sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)\n\n    def test_renderer_emits_hand_built_ir_without_source_contracts(self):\n''',
)
