#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    source = target.read_text()
    if source.count(old) != 1:
        raise SystemExit(f"expected exactly one match in {path}, found {source.count(old)}")
    target.write_text(source.replace(old, new))


replace(
    "codegen/sdk_ir.py",
    '''@dataclass(frozen=True)\nclass RequestPolicy:\n''',
    '''@dataclass(frozen=True)\nclass TypeAliasPolicy:\n    pass\n\n\n@dataclass(frozen=True)\nclass RequestPolicy:\n''',
)
replace(
    "codegen/sdk_ir.py",
    '''ModelPolicy = UnionPolicy | SimpleUnionPolicy | RequestPolicy | ViewPolicy\n''',
    '''ModelPolicy = UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | RequestPolicy | ViewPolicy\n''',
)
replace(
    "codegen/sdk_ir.py",
    '''    if "accessors" in config:\n        return ViewPolicy(config.get("borrowed", True), tuple(\n''',
    '''    if config.get("type_alias"):\n        return TypeAliasPolicy()\n    if "accessors" in config:\n        return ViewPolicy(config.get("borrowed", True), tuple(\n''',
)

replace(
    "codegen/sdk-semantics.schema.json",
    '''      "adapters": {"$ref": "#/$defs/mapping"}, "union": {"$ref": "#/$defs/union"}, "simple_union": {"$ref": "#/$defs/simple_union"}, "union_factory": {"$ref": "#/$defs/factory"},\n''',
    '''      "adapters": {"$ref": "#/$defs/mapping"}, "union": {"$ref": "#/$defs/union"}, "simple_union": {"$ref": "#/$defs/simple_union"}, "type_alias": {"const": true}, "union_factory": {"$ref": "#/$defs/factory"},\n''',
)
replace(
    "codegen/sdk-semantics.schema.json",
    '''      {"required": ["simple_union"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n      {"required": ["constructor"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n''',
    '''      {"required": ["simple_union"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["type_alias"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n      {"required": ["type_alias"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n      {"required": ["constructor"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n''',
)

replace(
    "codegen/sdk_codegen.py",
    '''from sdk_ir import (Accessor, ModelPolicy, RequestPolicy, SimpleUnionPolicy, UnionPolicy, ViewPolicy,\n                    StreamPolicy, model_policy, stream_policy)\n''',
    '''from sdk_ir import (Accessor, ModelPolicy, RequestPolicy, SimpleUnionPolicy, TypeAliasPolicy,\n                    UnionPolicy, ViewPolicy, StreamPolicy, model_policy, stream_policy)\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''        _validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "union_factory", "accessors", "borrowed"}, f"model {name}")\n''',
    '''        _validate_keys(config, {"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "union_factory", "accessors", "borrowed"}, f"model {name}")\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''        elif "simple_union" in config:\n            configured = set(config["simple_union"]["variants"])\n            actual = {variant.name for variant in rust.variants(raw)}\n            if configured != actual:\n                raise GenerationError(\n                    f"raw union {raw} variant drift: missing={sorted(actual - configured)}, "\n                    f"extra={sorted(configured - actual)}"\n                )\n        else:\n''',
    '''        elif "simple_union" in config:\n            configured = set(config["simple_union"]["variants"])\n            actual = {variant.name for variant in rust.variants(raw)}\n            if configured != actual:\n                raise GenerationError(\n                    f"raw union {raw} variant drift: missing={sorted(actual - configured)}, "\n                    f"extra={sorted(configured - actual)}"\n                )\n        elif config.get("type_alias"):\n            if raw not in rust.aliases:\n                raise GenerationError(f"raw Rust type alias not found: {raw}")\n        else:\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''def _emit_model(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n    if isinstance(model.config, UnionPolicy):\n''',
    '''def _public_alias_type(syntax: RustType, rust: RustIndex, seen: tuple[str, ...] = ()) -> str:\n    if syntax.spelling in rust.aliases:\n        if syntax.spelling in seen:\n            raise GenerationError(f"recursive raw type alias: {syntax.spelling}")\n        return _public_alias_type(rust.aliases[syntax.spelling], rust, (*seen, syntax.spelling))\n    if syntax.spelling in rust.symbol_modules:\n        raise GenerationError(f"public type alias references generated symbol: {syntax.spelling}")\n    if syntax.kind == "generic_type":\n        return f"{syntax.constructor}<{', '.join(_public_alias_type(argument, rust, seen) for argument in syntax.arguments)}>"\n    return syntax.spelling\n\n\ndef _emit_type_alias(model: ModelSpec, rust: RustIndex) -> str:\n    assert isinstance(model.config, TypeAliasPolicy)\n    return f"pub type {model.name} = {_public_alias_type(rust.aliases[model.raw], rust, (model.raw,))};"\n\n\ndef _emit_model(model: ModelSpec, openapi: OpenApiIndex, rust: RustIndex) -> str:\n    if isinstance(model.config, UnionPolicy):\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''    if isinstance(model.config, SimpleUnionPolicy):\n        return _emit_simple_union(model, rust)\n    if isinstance(model.config, ViewPolicy):\n''',
    '''    if isinstance(model.config, SimpleUnionPolicy):\n        return _emit_simple_union(model, rust)\n    if isinstance(model.config, TypeAliasPolicy):\n        return _emit_type_alias(model, rust)\n    if isinstance(model.config, ViewPolicy):\n''',
)
replace(
    "codegen/sdk_codegen.py",
    '''        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage)\n''',
    '''        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage, rust)\n''',
)

replace(
    "codegen/sdk_autoproject.py",
    '''from typing import Any\n''',
    '''from typing import Any\n\nfrom rust_types import RustType, parse_type\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''def _simple_schema(schema: dict[str, Any]) -> bool:\n    schema, _ = _nullable(schema)\n    if "$ref" in schema or "enum" in schema or "const" in schema or "format" in schema:\n        return False\n    kind = schema.get("type")\n    if kind in {"string", "integer", "number", "boolean"}:\n        return True\n    if kind == "array":\n        return _simple_schema(schema.get("items", {}))\n    return False\n''',
    '''def _simple_schema(schema: dict[str, Any]) -> bool:\n    schema, _ = _nullable(schema)\n    if "$ref" in schema or "enum" in schema or "const" in schema:\n        return False\n    kind = schema.get("type")\n    if kind in {"string", "integer", "number", "boolean"}:\n        return True\n    if kind == "array":\n        return _simple_schema(schema.get("items", {}))\n    if kind == "object" and not schema.get("properties"):\n        additional = schema.get("additionalProperties")\n        return additional is True or (isinstance(additional, dict) and _simple_schema(additional))\n    return False\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''def _request_name(raw: str) -> str:\n    return f"{raw}Params"\n''',
    '''def _request_name(raw: str) -> str:\n    return f"{raw}Params"\n\n\ndef _alias_name(raw: str) -> str:\n    return f"{raw}Value"\n''',
)
old_request = '''def _ensure_request_model(models: dict[str, Any], schemas: dict[str, Any], raw: str) -> tuple[str | None, str | None]:\n    existing = _existing_model_by_raw(models, raw, True)\n    if existing:\n        return existing, None\n    schema = schemas.get(raw)\n    if not schema or schema.get("type") != "object":\n        return None, "request_model_projection"\n    properties = schema.get("properties", {})\n    if not all(_simple_schema(value) for value in properties.values()):\n        return None, "request_model_projection"\n    name = _request_name(raw)\n    models[name] = {"raw": raw, "constructor": list(schema.get("required", []))}\n    return name, None\n'''
new_request = '''def _strip_options(syntax: RustType) -> RustType:\n    while (inner := syntax.unary("Option")) is not None:\n        syntax = inner\n    return syntax\n\n\ndef _raw_public_leaf(syntax: RustType, rust: Any) -> bool:\n    if syntax.spelling in rust.aliases or syntax.spelling in rust.symbol_modules:\n        return False\n    if syntax.kind == "generic_type":\n        return all(_raw_public_leaf(argument, rust) for argument in syntax.arguments)\n    return True\n\n\ndef _safe_alias(syntax: RustType, rust: Any, seen: tuple[str, ...] = ()) -> bool:\n    if syntax.spelling in rust.aliases:\n        if syntax.spelling in seen:\n            return False\n        return _safe_alias(rust.aliases[syntax.spelling], rust, (*seen, syntax.spelling))\n    if syntax.spelling in rust.symbol_modules:\n        return False\n    if syntax.kind == "generic_type":\n        return all(_safe_alias(argument, rust, seen) for argument in syntax.arguments)\n    return True\n\n\ndef _ensure_alias_model(models: dict[str, Any], raw: str, rust: Any) -> tuple[str | None, str | None]:\n    if raw not in rust.aliases or not _safe_alias(rust.aliases[raw], rust, (raw,)):\n        return None, "request_model_projection"\n    existing = _existing_model_by_raw(models, raw, False)\n    if existing:\n        return existing, None\n    name = _alias_name(raw)\n    if name in models and models[name].get("raw", name) != raw:\n        return None, "request_model_projection"\n    models[name] = {"raw": raw, "type_alias": True}\n    return name, None\n\n\ndef _request_field_adapter(models: dict[str, Any], schemas: dict[str, Any], schema: dict[str, Any],\n                           raw_type: str, rust: Any, resolving: tuple[str, ...]) -> tuple[str | None, str | None]:\n    schema, _ = _nullable(schema)\n    syntax = _strip_options(parse_type(raw_type))\n    if schema.get("type") == "array":\n        inner = syntax.unary("Vec")\n        if inner is None:\n            return None, "request_model_projection"\n        return _request_field_adapter(models, schemas, schema.get("items", {}), inner.spelling, rust, resolving)\n\n    reference = _schema_ref(schema)\n    if reference:\n        target = schemas.get(reference, {})\n        if target.get("type") == "object" and target.get("properties"):\n            if syntax.spelling != reference:\n                return None, "request_model_projection"\n            return _ensure_request_model(models, schemas, reference, rust, resolving)\n        if target.get("type") == "object" and target.get("additionalProperties"):\n            if syntax.spelling in rust.aliases:\n                return _ensure_alias_model(models, syntax.spelling, rust)\n            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n        if _simple_schema(target):\n            if syntax.spelling in rust.aliases:\n                return _ensure_alias_model(models, syntax.spelling, rust)\n            return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n        return None, "request_model_projection"\n\n    if schema.get("type") == "object" and schema.get("additionalProperties"):\n        if syntax.spelling in rust.aliases:\n            return _ensure_alias_model(models, syntax.spelling, rust)\n        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n    if _simple_schema(schema):\n        if syntax.spelling in rust.aliases:\n            return _ensure_alias_model(models, syntax.spelling, rust)\n        return (None, None) if _raw_public_leaf(syntax, rust) else (None, "request_model_projection")\n    return None, "request_model_projection"\n\n\ndef _ensure_request_model(models: dict[str, Any], schemas: dict[str, Any], raw: str,\n                          rust: Any | None = None, resolving: tuple[str, ...] = ()) -> tuple[str | None, str | None]:\n    existing = _existing_model_by_raw(models, raw, True)\n    if existing:\n        return existing, None\n    schema = schemas.get(raw)\n    if not schema or schema.get("type") != "object" or raw in resolving:\n        return None, "request_model_projection"\n    properties = schema.get("properties", {})\n    if rust is None:\n        if not all(_simple_schema(value) for value in properties.values()):\n            return None, "request_model_projection"\n        name = _request_name(raw)\n        models[name] = {"raw": raw, "constructor": list(schema.get("required", []))}\n        return name, None\n    if raw not in rust.structs:\n        return None, "request_model_projection"\n    raw_fields = {field.name.removeprefix("r#"): field for field in rust.fields(raw)}\n    if set(raw_fields) != set(properties):\n        return None, "request_model_projection"\n    adapters: dict[str, str] = {}\n    for field, field_schema in sorted(properties.items()):\n        adapter, reason = _request_field_adapter(\n            models, schemas, field_schema, raw_fields[field].type, rust, (*resolving, raw)\n        )\n        if reason:\n            return None, reason\n        if adapter:\n            adapters[field] = adapter\n    name = _request_name(raw)\n    if name in models and models[name].get("raw", name) != raw:\n        return None, "request_model_projection"\n    config: dict[str, Any] = {"raw": raw, "constructor": list(schema.get("required", []))}\n    if adapters:\n        config["adapters"] = adapters\n    models[name] = config\n    return name, None\n'''
replace("codegen/sdk_autoproject.py", old_request, new_request)
replace(
    "codegen/sdk_autoproject.py",
    '''def expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],\n                    raw_coverage: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:\n''',
    '''def expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],\n                    raw_coverage: dict[str, Any], rust: Any | None = None) -> tuple[dict[str, Any], dict[str, Any]]:\n''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''            request_raw = _schema_ref(request_schema)\n            request, reason = _ensure_request_model(models, openapi.schemas, request_raw)\n            if reason:\n                rejected[operation_id] = reason\n                continue\n''',
    '''            request_raw = _schema_ref(request_schema)\n            model_snapshot = set(models)\n            request, reason = _ensure_request_model(models, openapi.schemas, request_raw, rust)\n            if reason:\n                for model_name in set(models) - model_snapshot:\n                    del models[model_name]\n                rejected[operation_id] = reason\n                continue\n''',
)

replace(
    "scripts/probe_sdk_coverage.py",
    '''        json.loads((ROOT / "src/generated/coverage.json").read_text()),\n    )\n''',
    '''        json.loads((ROOT / "src/generated/coverage.json").read_text()),\n        rust,\n    )\n''',
)

replace(
    "scripts/test_sdk_autoproject.py",
    '''import unittest\n''',
    '''import unittest\nfrom types import SimpleNamespace\n\nfrom rust_types import parse_type\n''',
)
replace(
    "scripts/test_sdk_autoproject.py",
    '''    def test_complex_request_is_review_debt_not_raw_type_leak(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["tool"] = {"$ref": "#/components/schemas/Tool"}\n        _, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"),\n        )\n        self.assertEqual(0, report["added_count"])\n        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])\n''',
    '''    def test_projects_nested_request_objects_with_generated_adapters(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["tool"] = {"$ref": "#/components/schemas/Tool"}\n        api.schemas["Tool"] = {\n            "type": "object", "required": ["name"],\n            "properties": {"name": {"type": "string"}},\n        }\n        fields = {\n            "CreateThing": (\n                SimpleNamespace(name="name", type="String"),\n                SimpleNamespace(name="enabled", type="Option<bool>"),\n                SimpleNamespace(name="tool", type="Option<Tool>"),\n            ),\n            "Tool": (SimpleNamespace(name="name", type="String"),),\n        }\n        rust = SimpleNamespace(\n            structs=set(fields), aliases={}, symbol_modules={name: "types" for name in fields},\n            fields=lambda name: fields[name],\n        )\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"), rust,\n        )\n        self.assertEqual(1, report["added_count"])\n        self.assertEqual({"tool": "ToolParams"}, expanded["models"]["CreateThingParams"]["adapters"])\n        self.assertEqual(["name"], expanded["models"]["ToolParams"]["constructor"])\n\n    def test_projects_safe_map_alias_without_leaking_generated_alias_name(self):\n        api = FakeOpenApi()\n        api.schemas["CreateThing"]["properties"]["metadata"] = {\n            "type": "object", "additionalProperties": True,\n        }\n        fields = {\n            "CreateThing": (\n                SimpleNamespace(name="name", type="String"),\n                SimpleNamespace(name="enabled", type="Option<bool>"),\n                SimpleNamespace(name="metadata", type="Option<MetadataDict>"),\n            ),\n        }\n        rust = SimpleNamespace(\n            structs=set(fields),\n            aliases={"MetadataDict": parse_type("std::collections::BTreeMap<String, serde_json::Value>")},\n            symbol_modules={"CreateThing": "types", "MetadataDict": "types"},\n            fields=lambda name: fields[name],\n        )\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"create_thing": ["things.create"]}},\n            raw_coverage("create_thing"), rust,\n        )\n        self.assertEqual(1, report["added_count"])\n        self.assertEqual({"metadata": "MetadataDictValue"}, expanded["models"]["CreateThingParams"]["adapters"])\n        self.assertTrue(expanded["models"]["MetadataDictValue"]["type_alias"])\n''',
)

replace(
    "scripts/test_sdk_facade.py",
    '''    def test_unrelated_api_uses_the_same_generic_emitters(self):\n''',
    '''    def test_public_type_alias_expands_safe_generated_alias(self):\n        source = TYPES + "\\npub type MetadataDict = std::collections::BTreeMap<String, serde_json::Value>;\\n"\n        rust = sdk_codegen.RustIndex(source.encode(), CLIENT.encode())\n        model = sdk_codegen.ModelSpec("MetadataValue", "MetadataDict", sdk_codegen.model_policy({"type_alias": True}))\n        emitted = sdk_codegen._emit_model(model, sdk_codegen.OpenApiIndex(openapi_document()), rust)\n        self.assertEqual(\n            "pub type MetadataValue = std::collections::BTreeMap<String, serde_json::Value>;",\n            emitted,\n        )\n\n    def test_unrelated_api_uses_the_same_generic_emitters(self):\n''',
)

print("Materialized issue #19 nested request projection support.")
