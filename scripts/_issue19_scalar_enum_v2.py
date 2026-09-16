from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def replace(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text()
    if old not in text:
        raise SystemExit(f"patch anchor not found in {path}: {old[:100]!r}")
    target.write_text(text.replace(old, new, 1))


# Preserve exact wire names at the generator adapter boundary.
replace(
    "codegen/sdk_raw_ir.py",
    "class RawVariant:\n    name: str\n    payload: str | None\n",
    "class RawVariant:\n    name: str\n    payload: str | None\n    wire_name: str | None = None\n",
)
replace(
    "codegen/sdk_openapi_to_rust.py",
    "from pathlib import Path\n\nfrom tree_sitter",
    "from pathlib import Path\nimport re\n\nfrom tree_sitter",
)
replace(
    "codegen/sdk_openapi_to_rust.py",
    "def _children(node: Node | None, kind: str) -> list[Node]:\n    return [child for child in node.named_children if child.type == kind] if node else []\n",
    '''def _children(node: Node | None, kind: str) -> list[Node]:
    return [child for child in node.named_children if child.type == kind] if node else []


def _serde_rename(attributes: list[str]) -> str | None:
    matches = []
    for attribute in attributes:
        match = re.fullmatch(r'#\\[serde\\(rename\\s*=\\s*"([^"]+)"\\)\\]', attribute)
        if match:
            matches.append(match.group(1))
    if len(matches) > 1:
        raise OpenApiToRustAdapterError("multiple serde rename attributes on raw enum variant")
    return matches[0] if matches else None
''',
)
replace(
    "codegen/sdk_openapi_to_rust.py",
    '''            elif item.type == "enum_item":
                name = _text(types_source, item.child_by_field_name("name"))
                variants = []
                for variant in _children(item.child_by_field_name("body"), "enum_variant"):
                    variant_name = _text(types_source, variant.child_by_field_name("name"))
                    body = variant.child_by_field_name("body")
                    payload = None
                    if body and body.type == "ordered_field_declaration_list":
                        payload_nodes = list(body.named_children)
                        if len(payload_nodes) != 1:
                            raise OpenApiToRustAdapterError(
                                f"raw enum {name}::{variant_name} is not unary"
                            )
                        payload = _text(types_source, payload_nodes[0])
                    variants.append(RawVariant(variant_name, payload))
                enums[name] = tuple(variants)
''',
    '''            elif item.type == "enum_item":
                name = _text(types_source, item.child_by_field_name("name"))
                variants = []
                pending_attributes: list[str] = []
                body_node = item.child_by_field_name("body")
                for child in body_node.named_children if body_node else ():
                    if child.type == "attribute_item":
                        pending_attributes.append(_text(types_source, child))
                        continue
                    if child.type != "enum_variant":
                        continue
                    variant = child
                    variant_name = _text(types_source, variant.child_by_field_name("name"))
                    attributes = pending_attributes + [
                        _text(types_source, attribute)
                        for attribute in _children(variant, "attribute_item")
                    ]
                    pending_attributes = []
                    body = variant.child_by_field_name("body")
                    payload = None
                    if body and body.type == "ordered_field_declaration_list":
                        payload_nodes = [node for node in body.named_children if node.type != "attribute_item"]
                        if len(payload_nodes) != 1:
                            raise OpenApiToRustAdapterError(
                                f"raw enum {name}::{variant_name} is not unary"
                            )
                        payload = _text(types_source, payload_nodes[0])
                    variants.append(RawVariant(
                        variant_name, payload, _serde_rename(attributes)
                    ))
                enums[name] = tuple(variants)
''',
)

# Closed policy/resolved model IR.
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
    "class MapModelSpec:\n    public_type: str\n    raw_field: str\n\n\n@dataclass(frozen=True)\nclass ScalarEnumModelSpec:\n    variants: tuple[str, ...]\n\n\nModelRenderSpec = (WrapperModelSpec | UnionModelSpec | SimpleUnionModelSpec |\n                   ViewModelSpec | AliasModelSpec | MapModelSpec | ScalarEnumModelSpec)",
)
replace(
    "codegen/sdk_ir.py",
    '''    if "map" in config:
        mapping = config["map"]
        return MapPolicy(mapping["root"], tuple(mapping.get("path", ())))
''',
    '''    if "map" in config:
        mapping = config["map"]
        return MapPolicy(mapping["root"], tuple(mapping.get("path", ())))
    if "scalar_enum" in config:
        enum = config["scalar_enum"]
        return ScalarEnumPolicy(enum["root"], tuple(enum.get("path", ())))
''',
)

# Semantic overlay schema, with scalar_enum mutually exclusive with every old model policy.
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
schema_path = ROOT / "codegen/sdk-semantics.schema.json"
lines = schema_path.read_text().splitlines()
out = []
inserted = False
for line in lines:
    is_policy = line.startswith('      {"required": [') and '"not": {"anyOf": [' in line
    if is_policy:
        line = line.replace(
            '"not": {"anyOf": [',
            '"not": {"anyOf": [{"required": ["scalar_enum"]}, ',
            1,
        )
    out.append(line)
    if line.startswith('      {"required": ["map"]'):
        out.append(
            '      {"required": ["scalar_enum"], "not": {"anyOf": '
            '[{"required": ["union"]}, {"required": ["simple_union"]}, '
            '{"required": ["type_alias"]}, {"required": ["map"]}, '
            '{"required": ["constructor"]}, {"required": ["union_factory"]}, '
            '{"required": ["accessors"]}]}},'
        )
        inserted = True
if not inserted:
    raise SystemExit("scalar enum schema insertion point not found")
schema_path.write_text("\n".join(out) + "\n")

# Frontend accepts the explicit policy; wire reconciliation happens in lowering.
replace(
    "codegen/sdk_frontend.py",
    '{"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "union_factory", "accessors", "borrowed"}',
    '{"raw", "constructor", "exclude", "adapters", "union", "simple_union", "type_alias", "map", "scalar_enum", "union_factory", "accessors", "borrowed"}',
)

# Automatic projection: only exact string enums with explicit raw wire provenance.
replace(
    "codegen/sdk_autoproject.py",
    "def _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n",
    "def _map_name(raw: str) -> str:\n    return f\"{raw}Map\"\n\n\ndef _scalar_enum_name(raw: str) -> str:\n    return f\"{raw}Value\"\n",
)
replace(
    "codegen/sdk_autoproject.py",
    "def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:",
    '''def _generated_scalar_enum(schema: dict[str, Any], raw: str, rust: Any) -> bool:
    values = schema.get("enum")
    if (schema.get("type") != "string" or not isinstance(values, list) or not values
            or not all(isinstance(value, str) for value in values)):
        return False
    variants = rust.enums.get(raw)
    if not variants or any(variant.payload is not None or variant.wire_name is None
                           for variant in variants):
        return False
    wire = [variant.wire_name for variant in variants]
    return len(set(wire)) == len(wire) and set(wire) == set(values)


def _ensure_scalar_enum_model(models: dict[str, Any], schema: dict[str, Any], raw: str,
                              rust: Any, root: str,
                              path: tuple[str, ...]) -> tuple[str | None, str | None]:
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


def _omittable_singleton_enum(schema: dict[str, Any], raw_type: str, schemas: dict[str, Any], rust: Any) -> bool:''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''    reference = _schema_ref(schema)
    if reference:
        target = schemas.get(reference, {})
        if target.get("type") == "object" and target.get("properties"):
''',
    '''    reference = _schema_ref(schema)
    if reference:
        target = schemas.get(reference, {})
        if target.get("type") == "string" and target.get("enum"):
            return _ensure_scalar_enum_model(
                models, target, syntax.spelling, rust, reference, ()
            )
        if target.get("type") == "object" and target.get("properties"):
''',
)
replace(
    "codegen/sdk_autoproject.py",
    '''    if schema.get("type") == "object" and schema.get("additionalProperties"):
''',
    '''    if schema.get("type") == "string" and schema.get("enum"):
        return _ensure_scalar_enum_model(
            models, schema, syntax.spelling, rust, root, path
        )
    if schema.get("type") == "object" and schema.get("additionalProperties"):
''',
)

# Last source-aware stage validates exact OpenAPI/raw enum correspondence.
replace(
    "codegen/sdk_model_lowering.py",
    "MapIntoValue, MapModelSpec, MapPolicy, ModelSpec, RequestPolicy, ResolvedAccessor,",
    "MapIntoValue, MapModelSpec, MapPolicy, ModelSpec, RequestPolicy, ResolvedAccessor,\n                    ScalarEnumModelSpec, ScalarEnumPolicy,",
)
replace(
    "codegen/sdk_model_lowering.py",
    "def _resolve_model(model: ModelSpec, openapi, raw_index):\n",
    '''def _resolve_scalar_enum(model: ModelSpec, openapi, raw_index) -> ScalarEnumModelSpec:
    assert isinstance(model.config, ScalarEnumPolicy)
    schema = _schema_at(openapi, model.config.root, model.config.path)
    values = schema.get("enum")
    if (schema.get("type") != "string" or not isinstance(values, list) or not values
            or not all(isinstance(value, str) for value in values)):
        raise ModelLoweringError(
            f"scalar enum policy {model.name} does not resolve to a string enum"
        )
    variants = raw_index.variants(model.raw)
    if not variants or any(variant.payload is not None or variant.wire_name is None
                           for variant in variants):
        raise ModelLoweringError(
            f"raw scalar enum {model.raw} requires unit variants with serde rename provenance"
        )
    by_wire = {variant.wire_name: variant.name for variant in variants}
    if len(by_wire) != len(variants) or set(by_wire) != set(values):
        raise ModelLoweringError(
            f"raw scalar enum {model.raw} wire drift: "
            f"expected={sorted(values)}, actual={sorted(by_wire)}"
        )
    return ScalarEnumModelSpec(tuple(by_wire[value] for value in values))


def _resolve_model(model: ModelSpec, openapi, raw_index):
''',
)
replace(
    "codegen/sdk_model_lowering.py",
    '''    if isinstance(model.config, MapPolicy):
        return _resolve_map(model, openapi, raw_index)
''',
    '''    if isinstance(model.config, MapPolicy):
        return _resolve_map(model, openapi, raw_index)
    if isinstance(model.config, ScalarEnumPolicy):
        return _resolve_scalar_enum(model, openapi, raw_index)
''',
)

# Source-free emission from resolved enum variants only.
replace(
    "codegen/sdk_emit.py",
    "LiteralValue, MapIntoValue, MapModelSpec, ModelSpec, OperationSpec,",
    "LiteralValue, MapIntoValue, MapModelSpec, ModelSpec, OperationSpec,\n                    ScalarEnumModelSpec,",
)
replace(
    "codegen/sdk_emit.py",
    "def emit_model(model: ModelSpec) -> str:\n",
    '''def _emit_scalar_enum(model: ModelSpec, spec: ScalarEnumModelSpec) -> str:
    variants = ",".join(spec.variants)
    arms = ",".join(
        f"{model.name}::{variant} => Self::{variant}" for variant in spec.variants
    )
    return (
        f"#[derive(Debug, Clone, Copy, PartialEq, Eq)]\\n"
        f"#[non_exhaustive]\\npub enum {model.name} {{\\n"
        f"{_indent(variants)}\\n}}\\n\\n"
        f"impl From<{model.name}> for {model.raw} {{\\n"
        f"    fn from(value: {model.name}) -> Self {{ match value {{\\n"
        f"{_indent(arms, 8)}\\n    }} }}\\n}}"
    )


def emit_model(model: ModelSpec) -> str:
''',
)
replace(
    "codegen/sdk_emit.py",
    '''    if isinstance(spec, MapModelSpec):
        return _emit_map(model, spec)
''',
    '''    if isinstance(spec, MapModelSpec):
        return _emit_map(model, spec)
    if isinstance(spec, ScalarEnumModelSpec):
        return _emit_scalar_enum(model, spec)
''',
)

# Raw adapter contract test.
test_compiler = ROOT / "scripts/test_sdk_compiler.py"
text = test_compiler.read_text()
anchor = "    def test_adapter_normalizes_supported_raw_shapes(self):\n"
if anchor not in text:
    raise SystemExit("compiler test anchor missing")
test = '''    def test_adapter_preserves_explicit_enum_wire_names(self):
        raw = OpenApiToRustAdapter.parse(
            b''' + "'''" + '''pub enum Visibility {
                #[serde(rename = "shared_global")]
                SharedGlobal,
                #[serde(rename = "private")]
                Private,
            }''' + "'''" + ''',
            b"impl HttpClient {}",
        )
        self.assertEqual(
            [("SharedGlobal", None, "shared_global"), ("Private", None, "private")],
            [(variant.name, variant.payload, variant.wire_name)
             for variant in raw.variants("Visibility")],
        )

'''
test_compiler.write_text(text.replace(anchor, test + anchor, 1))

# Automatic projection tests.
test_auto = ROOT / "scripts/test_sdk_autoproject.py"
text = test_auto.read_text()
anchor = "    def test_projects_nested_request_objects_with_generated_adapters(self):\n"
if anchor not in text:
    raise SystemExit("autoproject test anchor missing")
test = '''    def test_projects_named_scalar_enum_without_raw_type_leak(self):
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

# Lowering and pure renderer test on normalized RawIr.
replace(
    "scripts/test_sdk_emit.py",
    "import sdk_pipeline\nfrom sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, ModelSpec, NoRequest,",
    "import sdk_pipeline\nimport sdk_model_lowering\nfrom sdk_openapi_to_rust import OpenApiToRustAdapter\nfrom sdk_ir import (AliasModelSpec, EmptyResponse, FacadeIr, ModelSpec, NoRequest,",
)
replace(
    "scripts/test_sdk_emit.py",
    "OperationCall, OperationSpec, RawSignature, ResourceSpec,\n                    TypeAliasPolicy)",
    "OperationCall, OperationSpec, RawSignature, ResourceSpec,\n                    ScalarEnumModelSpec, ScalarEnumPolicy, TypeAliasPolicy)",
)
test_emit = ROOT / "scripts/test_sdk_emit.py"
text = test_emit.read_text()
anchor = "    def test_renderer_emits_hand_built_ir_without_source_contracts(self):\n"
if anchor not in text:
    raise SystemExit("emit test anchor missing")
test = '''    def test_scalar_enum_lowers_wire_contract_before_source_free_emission(self):
        raw = OpenApiToRustAdapter.parse(
            b''' + "'''" + '''pub enum RawVisibility {
                #[serde(rename = "shared_global")]
                SharedGlobal,
                #[serde(rename = "private")]
                Private,
            }''' + "'''" + ''',
            b"impl HttpClient {}",
        )
        openapi = sdk_compiler.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"visibility": {
                    "type": "string", "enum": ["shared_global", "private"],
                }},
            }}},
        })
        model = ModelSpec(
            "VisibilityValue", "RawVisibility",
            ScalarEnumPolicy("Root", ("visibility",)),
        )
        resolved = sdk_model_lowering.resolve_models(
            FacadeIr("Client", (model,), ()), openapi, raw,
        )
        spec = resolved.models[0].render
        self.assertIsInstance(spec, ScalarEnumModelSpec)
        self.assertEqual(("SharedGlobal", "Private"), spec.variants)
        source = sdk_emit.emit_model(resolved.models[0])
        self.assertIn("pub enum VisibilityValue", source)
        self.assertIn("VisibilityValue::SharedGlobal => Self::SharedGlobal", source)
        self.assertNotIn("shared_global", source)

    def test_scalar_enum_lowering_rejects_wire_drift(self):
        raw = OpenApiToRustAdapter.parse(
            b''' + "'''" + '''pub enum RawVisibility {
                #[serde(rename = "shared_global")]
                SharedGlobal,
            }''' + "'''" + ''',
            b"impl HttpClient {}",
        )
        openapi = sdk_compiler.OpenApiIndex({
            "openapi": "3.1.0", "paths": {},
            "components": {"schemas": {"Root": {
                "type": "object",
                "properties": {"visibility": {"type": "string", "enum": ["private"]}},
            }}},
        })
        model = ModelSpec(
            "VisibilityValue", "RawVisibility",
            ScalarEnumPolicy("Root", ("visibility",)),
        )
        with self.assertRaisesRegex(sdk_model_lowering.ModelLoweringError, "wire drift"):
            sdk_model_lowering.resolve_models(FacadeIr("Client", (model,), ()), openapi, raw)

'''
test_emit.write_text(text.replace(anchor, test + anchor, 1))
