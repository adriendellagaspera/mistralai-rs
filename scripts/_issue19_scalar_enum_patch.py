from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"patch anchor not found in {path}: {old[:80]!r}")
    target.write_text(text.replace(old, new, 1))


# Raw enum wire provenance.
replace(
    "codegen/sdk_frontend.py",
    "class RustVariant:\n    name: str\n    payload: str | None\n",
    "class RustVariant:\n    name: str\n    payload: str | None\n    wire_name: str | None = None\n",
)
replace(
    "codegen/sdk_frontend.py",
    "def _children(node: Node | None, kind: str) -> list[Node]:\n    return [child for child in node.named_children if child.type == kind] if node else []\n",
    "def _children(node: Node | None, kind: str) -> list[Node]:\n    return [child for child in node.named_children if child.type == kind] if node else []\n\n\ndef _serde_rename(attributes: list[str]) -> str | None:\n    prefix, suffix = '#[serde(rename = \\\"', '\\\")]'\n    matches = [value[len(prefix):-len(suffix)] for value in attributes\n               if value.startswith(prefix) and value.endswith(suffix)]\n    if len(matches) > 1:\n        raise GenerationError(\"multiple serde rename attributes on raw enum variant\")\n    return matches[0] if matches else None\n",
)
replace(
    "codegen/sdk_frontend.py",
    """            elif item.type == \"enum_item\":
                name = _text(source, item.child_by_field_name(\"name\"))
                variants = []
                for variant in _children(item.child_by_field_name(\"body\"), \"enum_variant\"):
                    variant_name = _text(source, variant.child_by_field_name(\"name\"))
                    body = variant.child_by_field_name(\"body\")
                    payload = None
                    if body and body.type == \"ordered_field_declaration_list\":
                        payload_nodes = [child for child in body.named_children]
                        if len(payload_nodes) != 1:
                            raise GenerationError(f\"raw enum {name}::{variant_name} is not unary\")
                        payload = _text(source, payload_nodes[0])
                    variants.append(RustVariant(variant_name, payload))
                self.enums[name] = tuple(variants)
""",
    """            elif item.type == \"enum_item\":
                name = _text(source, item.child_by_field_name(\"name\"))
                variants = []
                pending_attributes: list[str] = []
                body_node = item.child_by_field_name(\"body\")
                for child in body_node.named_children if body_node else ():
                    if child.type == \"attribute_item\":
                        pending_attributes.append(_text(source, child))
                        continue
                    if child.type != \"enum_variant\":
                        continue
                    variant = child
                    variant_name = _text(source, variant.child_by_field_name(\"name\"))
                    attributes = pending_attributes + [
                        _text(source, attribute)
                        for attribute in _children(variant, \"attribute_item\")
                    ]
                    pending_attributes = []
                    body = variant.child_by_field_name(\"body\")
                    payload = None
                    if body and body.type == \"ordered_field_declaration_list\":
                        payload_nodes = [child for child in body.named_children]
                        if len(payload_nodes) != 1:
                            raise GenerationError(f\"raw enum {name}::{variant_name} is not unary\")
                        payload = _text(source, payload_nodes[0])
                    variants.append(RustVariant(variant_name, payload, _serde_rename(attributes)))
                self.enums[name] = tuple(variants)
""",
)
replace(
    "codegen/sdk_frontend.py",
    '{"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "union_factory", "accessors", "borrowed"}',
    '{"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "scalar_enum", "union_factory", "accessors", "borrowed"}',
)

# Policy and resolved IR.
replace(
    "codegen/sdk_ir.py",
    "class MapPolicy:\n    root: str\n    path: tuple[str, ...]\n",
    "class MapPolicy:\n    root: str\n    path: tuple[str, ...]\n\n\n@dataclass(frozen=True)\nclass ScalarEnumPolicy:\n    root: str\n    path: tuple[str, ...]\n",
)
replace(
    "codegen/sdk_ir.py",
    "ModelPolicy = UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | MapPolicy | RequestPolicy | ViewPolicy",
    "ModelPolicy = (UnionPolicy | SimpleUnionPolicy | TypeAliasPolicy | MapPolicy | ScalarEnumPolicy |\n               RequestPolicy | ViewPolicy)",
)
replace(
    "codegen/sdk_ir.py",
    "class MapModelSpec:\n    public_type: str\n    raw_field: str\n\n\nModelRenderSpec = (WrapperModelSpec | UnionModelSpec | SimpleUnionModelSpec |\n                   ViewModelSpec | AliasModelSpec | MapModelSpec)",
    "class MapModelSpec:\n    public_type: str\n    raw_field: str\n\n\n@dataclass(frozen=True)\nclass ScalarEnumModelSpec:\n    variants: tuple[tuple[str, str], ...]\n\n\nModelRenderSpec = (WrapperModelSpec | UnionModelSpec | SimpleUnionModelSpec |\n                   ViewModelSpec | AliasModelSpec | MapModelSpec | ScalarEnumModelSpec)",
)
replace(
    "codegen/sdk_ir.py",
    "    if \"map\" in config:\n        mapping = config[\"map\"]\n        return MapPolicy(mapping[\"root\"], tuple(mapping.get(\"path\", ())))\n",
    "    if \"map\" in config:\n        mapping = config[\"map\"]\n        return MapPolicy(mapping[\"root\"], tuple(mapping.get(\"path\", ())))\n    if \"scalar_enum\" in config:\n        enum = config[\"scalar_enum\"]\n        return ScalarEnumPolicy(enum[\"root\"], tuple(enum.get(\"path\", ())))\n",
)

# Semantic overlay schema. Keep formatting localized.
replace(
    "codegen/sdk-semantics.schema.json",
    '    "map": {"type": "object", "additionalProperties": false, "required": ["root", "path"], "properties": {\n      "root": {"$ref": "#/$defs/identifier"}, "path": {"$ref": "#/$defs/schema_path"}\n    }},',
    '    "map": {"type": "object", "additionalProperties": false, "required": ["root", "path"], "properties": {\n      "root": {"$ref": "#/$defs/identifier"}, "path": {"$ref": "#/$defs/schema_path"}\n    }},\n    "scalar_enum": {"type": "object", "additionalProperties": false, "required": ["root", "path"], "properties": {\n      "root": {"$ref": "#/$defs/identifier"}, "path": {"$ref": "#/$defs/schema_path"}\n    }},',
)
replace(
    "codegen/sdk-semantics.schema.json",
    '"simple_union": {"$ref": "#/$defs/simple_union"}, "type_alias": {"const": true}, "map": {"$ref": "#/$defs/map"}, "union_factory"',
    '"simple_union": {"$ref": "#/$defs/simple_union"}, "type_alias": {"const": true}, "map": {"$ref": "#/$defs/map"}, "scalar_enum": {"$ref": "#/$defs/scalar_enum"}, "union_factory"',
)
# Insert scalar_enum exclusion into every existing model-policy branch, then add its own branch.
schema_path = ROOT / "codegen/sdk-semantics.schema.json"
schema_text = schema_path.read_text().replace(
    '{"required": ["map"]}, {"required": ["constructor"]}',
    '{"required": ["map"]}, {"required": ["scalar_enum"]}, {"required": ["constructor"]}',
).replace(
    '{"required": ["type_alias"]}, {"required": ["constructor"]}',
    '{"required": ["type_alias"]}, {"required": ["scalar_enum"]}, {"required": ["constructor"]}',
)
anchor = '      {"required": ["map"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n'
if anchor not in schema_text:
    raise SystemExit("scalar enum schema branch anchor not found")
schema_text = schema_text.replace(
    anchor,
    anchor.replace('{"required": ["constructor"]}', '{"required": ["scalar_enum"]}, {"required": ["constructor"]}')
    + '      {"required": ["scalar_enum"], "not": {"anyOf": [{"required": ["union"]}, {"required": ["simple_union"]}, {"required": ["type_alias"]}, {"required": ["map"]}, {"required": ["constructor"]}, {"required": ["union_factory"]}, {"required": ["accessors"]}]}},\n',
    1,
)
schema_path.write_text(schema_text)

# Automatic projection.
replace(
    "codegen/sdk_autoproject.py",
    "def _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n",
    "def _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n\n\ndef _scalar_enum_name(raw: str) -> str:\n    return f\"{raw}Value\"\n",
)
replace(
    "codegen/sdk_autoproject.py",
    "def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:",
    """def _generated_scalar_enum(schema: dict[str, Any], raw: str, rust: Any) -> bool:
    values = schema.get("enum")
    if schema.get("type") != "string" or not isinstance(values, list) or not values or not all(isinstance(value, str) for value in values):
        return False
    variants = rust.enums.get(raw) if hasattr(rust, "enums") else None
    if not variants or any(variant.payload is not None or getattr(variant, "wire_name", None) is None for variant in variants):
        return False
    wire = [variant.wire_name for variant in variants]
    return len(set(wire)) == len(wire) and set(wire) == set(values)


def _ensure_scalar_enum_model(models: dict[str, Any], schema: dict[str, Any], raw: str, rust: Any,
                              root: str, path: tuple[str, ...]) -> tuple[str | None, str | None]:
    if not _generated_scalar_enum(schema, raw, rust):
        return None, "request_model_projection"
    for name, config in models.items():
        if config.get("raw", name) == raw and "scalar_enum" in config:
            return name, None
    name = _scalar_enum_name(raw)
    if name in models and models[name].get("raw", name) != raw:
        return None, "request_model_projection"
    models[name] = {"raw": raw, "scalar_enum": {"root": root, "path": list(path)}}
    return name, None


def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:""",
)
replace(
    "codegen/sdk_autoproject.py",
    """    reference = _schema_ref(schema)
    if reference:
        target = schemas.get(reference, {})
        if target.get("type") == "object" and target.get("properties"):
""",
    """    reference = _schema_ref(schema)
    if reference:
        target = schemas.get(reference, {})
        if target.get("type") == "string" and target.get("enum"):
            return _ensure_scalar_enum_model(models, target, syntax.spelling, rust, reference, ())
        if target.get("type") == "object" and target.get("properties"):
""",
)
replace(
    "codegen/sdk_autoproject.py",
    """    if schema.get("type") == "object" and schema.get("additionalProperties"):
""",
    """    if schema.get("type") == "string" and schema.get("enum"):
        return _ensure_scalar_enum_model(models, schema, syntax.spelling, rust, root, path)
    if schema.get("type") == "object" and schema.get("additionalProperties"):
""",
)

# Source-aware enum lowering.
replace(
    "codegen/sdk_model_lowering.py",
    "MapIntoValue, MapModelSpec, MapPolicy, ModelSpec, RequestPolicy, ResolvedAccessor,",
    "MapIntoValue, MapModelSpec, MapPolicy, ModelSpec, RequestPolicy, ResolvedAccessor,\n                    ScalarEnumModelSpec, ScalarEnumPolicy,",
)
replace(
    "codegen/sdk_model_lowering.py",
    "def _resolve_model(model: ModelSpec, openapi, raw_index):\n",
    """def _resolve_scalar_enum(model: ModelSpec, openapi, raw_index) -> ScalarEnumModelSpec:
    assert isinstance(model.config, ScalarEnumPolicy)
    schema = _schema_at(openapi, model.config.root, model.config.path)
    values = schema.get("enum")
    if schema.get("type") != "string" or not isinstance(values, list) or not values or not all(isinstance(value, str) for value in values):
        raise ModelLoweringError(f"scalar enum policy {model.name} does not resolve to a string enum")
    variants = raw_index.variants(model.raw)
    if any(variant.payload is not None or variant.wire_name is None for variant in variants):
        raise ModelLoweringError(f"raw scalar enum {model.raw} requires unit variants with serde rename provenance")
    by_wire = {variant.wire_name: variant.name for variant in variants}
    if len(by_wire) != len(variants) or set(by_wire) != set(values):
        raise ModelLoweringError(
            f"raw scalar enum {model.raw} wire drift: expected={sorted(values)}, actual={sorted(by_wire)}"
        )
    return ScalarEnumModelSpec(tuple((by_wire[value], by_wire[value]) for value in values))


def _resolve_model(model: ModelSpec, openapi, raw_index):
""",
)
replace(
    "codegen/sdk_model_lowering.py",
    "    if isinstance(model.config, MapPolicy):\n        return _resolve_map(model, openapi, raw_index)\n",
    "    if isinstance(model.config, MapPolicy):\n        return _resolve_map(model, openapi, raw_index)\n    if isinstance(model.config, ScalarEnumPolicy):\n        return _resolve_scalar_enum(model, openapi, raw_index)\n",
)

# Pure renderer.
replace(
    "codegen/sdk_emit.py",
    "LiteralValue, MapIntoValue, MapModelSpec, ModelSpec, OperationSpec,",
    "LiteralValue, MapIntoValue, MapModelSpec, ModelSpec, OperationSpec,\n                    ScalarEnumModelSpec,",
)
replace(
    "codegen/sdk_emit.py",
    "def emit_model(model: ModelSpec) -> str:\n",
    """def _emit_scalar_enum(model: ModelSpec, spec: ScalarEnumModelSpec) -> str:
    variants = ",".join(public for public, _ in spec.variants)
    arms = ",".join(
        f"{model.name}::{public} => Self::{raw}" for public, raw in spec.variants
    )
    return (
        f"#[derive(Debug, Clone, Copy, PartialEq, Eq)]\\n#[non_exhaustive]\\npub enum {model.name} {{\\n"
        f"{_indent(variants)}\\n}}\\n\\n"
        f"impl From<{model.name}> for {model.raw} {{\\n"
        f"    fn from(value: {model.name}) -> Self {{ match value {{\\n"
        f"{_indent(arms, 8)}\\n    }} }}\\n}}"
    )


def emit_model(model: ModelSpec) -> str:
""",
)
replace(
    "codegen/sdk_emit.py",
    "    if isinstance(spec, MapModelSpec):\n        return _emit_map(model, spec)\n",
    "    if isinstance(spec, MapModelSpec):\n        return _emit_map(model, spec)\n    if isinstance(spec, ScalarEnumModelSpec):\n        return _emit_scalar_enum(model, spec)\n",
)

# Auto-projection tests.
test_auto = ROOT / "scripts/test_sdk_autoproject.py"
text = test_auto.read_text()
anchor = "    def test_projects_nested_request_objects_with_generated_adapters(self):\n"
if anchor not in text:
    raise SystemExit("autoproject test anchor missing")
test = '''    def test_projects_named_scalar_enum_without_raw_type_leak(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"] = {
            "type": "object", "required": ["name", "visibility"],
            "properties": {
                "name": {"type": "string"},
                "visibility": {"$ref": "#/components/schemas/ResourceVisibility"},
            },
        }
        api.schemas["ResourceVisibility"] = {
            "type": "string", "enum": ["shared_global", "private"],
        }
        fields = {"CreateThing": (
            SimpleNamespace(name="name", type="String"),
            SimpleNamespace(name="visibility", type="ResourceVisibility"),
        )}
        enums = {"ResourceVisibility": (
            SimpleNamespace(name="SharedGlobal", payload=None, wire_name="shared_global"),
            SimpleNamespace(name="Private", payload=None, wire_name="private"),
        )}
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums=enums,
            symbol_modules={"CreateThing": "types", "ResourceVisibility": "types"},
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
            {"raw": "ResourceVisibility", "scalar_enum": {"root": "ResourceVisibility", "path": []}},
            expanded["models"]["ResourceVisibilityValue"],
        )

    def test_scalar_enum_projection_fails_closed_on_wire_drift(self):
        api = FakeOpenApi()
        api.schemas["CreateThing"] = {
            "type": "object", "required": ["name", "visibility"],
            "properties": {
                "name": {"type": "string"},
                "visibility": {"$ref": "#/components/schemas/ResourceVisibility"},
            },
        }
        api.schemas["ResourceVisibility"] = {"type": "string", "enum": ["private"]}
        fields = {"CreateThing": (
            SimpleNamespace(name="name", type="String"),
            SimpleNamespace(name="visibility", type="ResourceVisibility"),
        )}
        enums = {"ResourceVisibility": (
            SimpleNamespace(name="Private", payload=None, wire_name="shared_global"),
        )}
        rust = SimpleNamespace(
            structs=set(fields), aliases={}, enums=enums,
            symbol_modules={"CreateThing": "types", "ResourceVisibility": "types"},
            fields=lambda name: fields[name],
        )
        _, report = sdk_autoproject.expand_manifest(
            api, manifest(), {"operations": {"create_thing": ["things.create"]}},
            raw_coverage("create_thing"), rust,
        )
        self.assertEqual("request_model_projection", report["rejected"]["create_thing"])

'''
test_auto.write_text(text.replace(anchor, test + anchor, 1))

# Lowering + pure-emitter tests, including parsing serde rename provenance.
replace(
    "scripts/test_sdk_emit.py",
    "from sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, MapModelSpec, MapPolicy, ModelSpec, NoRequest,",
    "from sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, MapModelSpec, MapPolicy, ModelSpec, NoRequest,\n                    ScalarEnumModelSpec, ScalarEnumPolicy,",
)
test_emit = ROOT / "scripts/test_sdk_emit.py"
text = test_emit.read_text()
anchor = "    def test_map_policy_lowers_to_source_agnostic_render_spec(self):\n"
if anchor not in text:
    raise SystemExit("emit test anchor missing")
test = '''    def test_scalar_enum_policy_uses_serde_wire_provenance(self):
        rust = sdk_codegen.RustIndex(
            b''' + "'''" + '''pub enum RawVisibility {
                #[serde(rename = "shared_global")] SharedGlobal,
                #[serde(rename = "private")] Private,
            }''' + "'''" + ''',
            b"impl HttpClient {}",
        )
        self.assertEqual(
            ["shared_global", "private"],
            [variant.wire_name for variant in rust.variants("RawVisibility")],
        )
        openapi = sdk_codegen.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"visibility": {"type": "string", "enum": ["shared_global", "private"]}},
            }}},
        })
        model = ModelSpec("VisibilityValue", "RawVisibility", ScalarEnumPolicy("Root", ("visibility",)))
        resolved = sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)
        spec = resolved.models[0].render
        self.assertIsInstance(spec, ScalarEnumModelSpec)
        self.assertEqual((("SharedGlobal", "SharedGlobal"), ("Private", "Private")), spec.variants)
        source = sdk_emit.emit_model(resolved.models[0])
        self.assertIn("pub enum VisibilityValue", source)
        self.assertIn("VisibilityValue::SharedGlobal => Self::SharedGlobal", source)
        self.assertNotIn("shared_global", source)

    def test_scalar_enum_policy_fails_closed_on_wire_drift(self):
        rust = sdk_codegen.RustIndex(
            b''' + "'''" + '''pub enum RawVisibility {
                #[serde(rename = "shared_global")] SharedGlobal,
            }''' + "'''" + ''',
            b"impl HttpClient {}",
        )
        openapi = sdk_codegen.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"visibility": {"type": "string", "enum": ["private"]}},
            }}},
        })
        model = ModelSpec("VisibilityValue", "RawVisibility", ScalarEnumPolicy("Root", ("visibility",)))
        with self.assertRaisesRegex(sdk_model_lowering.ModelLoweringError, "wire drift"):
            sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, rust)

'''
test_emit.write_text(text.replace(anchor, test + anchor, 1))
