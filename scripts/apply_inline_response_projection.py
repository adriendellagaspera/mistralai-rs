from pathlib import Path


def replace(path: str, old: str, new: str) -> None:
    p = Path(path)
    text = p.read_text()
    if old not in text:
        raise SystemExit(f"missing expected text in {path}: {old[:120]!r}")
    p.write_text(text.replace(old, new, 1))


# Semantic operation schema: direct scalar and homogeneous list projections.
replace(
    "codegen/sdk-semantics.schema.json",
    '"request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},',
    '"request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "list_response": {"$ref": "#/$defs/identifier"}, "scalar_response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},',
)

# Rust/OpenAPI reconciliation primitives.
replace(
    "codegen/sdk_codegen.py",
    '    def qualified_type(self, spelling: str) -> str:\n',
    '    def resolved_type(self, spelling: str) -> RustType:\n        syntax = parse_type(spelling)\n        seen: set[str] = set()\n        while syntax.spelling in self.aliases:\n            if syntax.spelling in seen:\n                raise GenerationError(f"recursive raw alias: {syntax.spelling}")\n            seen.add(syntax.spelling)\n            syntax = self.aliases[syntax.spelling]\n        return syntax\n\n    def qualified_type(self, spelling: str) -> str:\n',
)
replace(
    "codegen/sdk_codegen.py",
    '    def response_matches(self, operation_id: str, raw: str, rust: RustIndex) -> bool:\n        """Reconcile named and inline successful response schemas with raw Rust."""\n        schema = _success_schema(self.operation(operation_id))\n        referenced = _ref_name(schema)\n        if referenced:\n            return referenced == raw\n        branches = schema.get("oneOf", []) or schema.get("anyOf", [])\n        payloads = {_ref_name(branch) for branch in branches}\n        if branches and None not in payloads and raw in rust.enums:\n            return payloads == {variant.payload for variant in rust.variants(raw)}\n        return False\n',
    '    def schema_matches(self, schema: dict[str, Any], raw: str, rust: RustIndex) -> bool:\n        referenced = _ref_name(schema)\n        if referenced:\n            return referenced == raw\n        branches = schema.get("oneOf", []) or schema.get("anyOf", [])\n        payloads = {_ref_name(branch) for branch in branches}\n        if branches and None not in payloads and raw in rust.enums:\n            return payloads == {variant.payload for variant in rust.variants(raw)}\n        return False\n\n    def response_matches(self, operation_id: str, raw: str, rust: RustIndex) -> bool:\n        """Reconcile named and inline successful response schemas with raw Rust."""\n        return self.schema_matches(_success_schema(self.operation(operation_id)), raw, rust)\n',
)

# Operation IR.
replace(
    "codegen/sdk_codegen.py",
    '    request_raw: str | None = None\n    empty_response: bool = False\n',
    '    request_raw: str | None = None\n    empty_response: bool = False\n    list_response: str | None = None\n    scalar_response: str | None = None\n',
)
replace(
    "codegen/sdk_codegen.py",
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "list_response", "scalar_response", "empty_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
)
replace(
    "codegen/sdk_codegen.py",
    '            request, response = item.get("request"), item.get("response")\n            empty_response = item.get("empty_response", False)\n            success_modes = int(bool(response)) + int(bool(item.get("stream"))) + int(empty_response)\n            if success_modes != 1:\n                raise GenerationError(\n                    f"operation {module}.{public_name} requires exactly one response, stream or empty_response projection"\n                )\n            for referenced in (request, response):\n',
    '            request, response = item.get("request"), item.get("response")\n            list_response, scalar_response = item.get("list_response"), item.get("scalar_response")\n            empty_response = item.get("empty_response", False)\n            success_modes = (int(bool(response)) + int(bool(list_response)) + int(bool(scalar_response))\n                             + int(bool(item.get("stream"))) + int(empty_response))\n            if success_modes != 1:\n                raise GenerationError(\n                    f"operation {module}.{public_name} requires exactly one response, list_response, scalar_response, stream or empty_response projection"\n                )\n            for referenced in (request, response, list_response):\n',
)
replace(
    "codegen/sdk_codegen.py",
    '            if response and not item.get("stream"):\n                model = next(model for model in models if model.name == response)\n                if not openapi.response_matches(operation_id, model.raw, rust):\n                    raise GenerationError(f"OpenAPI response drift for {operation_id}")\n                if raw_operation.success_type != model.raw:\n                    raise GenerationError(f"raw response drift for {raw_method}")\n',
    '            if response and not item.get("stream"):\n                model = next(model for model in models if model.name == response)\n                if not openapi.response_matches(operation_id, model.raw, rust):\n                    raise GenerationError(f"OpenAPI response drift for {operation_id}")\n                if raw_operation.success_type != model.raw:\n                    raise GenerationError(f"raw response drift for {raw_method}")\n            if scalar_response:\n                wire_schema = _success_schema(wire_operation)\n                wire_scalar = {"string": "String"}.get(wire_schema.get("type"))\n                if wire_scalar != scalar_response:\n                    raise GenerationError(f"scalar response drift for {operation_id}")\n                if rust.resolved_type(raw_operation.success_type).spelling != scalar_response:\n                    raise GenerationError(f"raw scalar response drift for {raw_method}")\n            if list_response:\n                model = next(model for model in models if model.name == list_response)\n                wire_schema = _success_schema(wire_operation)\n                if wire_schema.get("type") != "array" or not openapi.schema_matches(wire_schema.get("items", {}), model.raw, rust):\n                    raise GenerationError(f"list response drift for {operation_id}")\n                transport = rust.resolved_type(raw_operation.success_type)\n                if transport.constructor != "Vec" or len(transport.arguments) != 1 or transport.arguments[0].spelling != model.raw:\n                    raise GenerationError(f"raw list response drift for {raw_method}")\n',
)
replace(
    "codegen/sdk_codegen.py",
    '                                            stream_policy(item.get("stream")), request_raw, empty_response))\n',
    '                                            stream_policy(item.get("stream")), request_raw, empty_response,\n                                            list_response, scalar_response))\n',
)

# Operation emission: preserve Vec ergonomics, no collection wrapper newtypes.
replace(
    "codegen/sdk_codegen.py",
    'def _emit_operation(operation: OperationSpec, rust: RustIndex, resource: ResourceSpec) -> str:\n',
    'def _response_type(operation: OperationSpec) -> str | None:\n    if operation.response:\n        return operation.response\n    if operation.list_response:\n        return f"Vec<{operation.list_response}>"\n    return operation.scalar_response\n\n\ndef _map_response(operation: OperationSpec, expression: str) -> str:\n    if operation.list_response:\n        return f"{expression}.map(|items| items.into_iter().map(Into::into).collect())"\n    return f"{expression}.map(Into::into)"\n\n\ndef _emit_operation(operation: OperationSpec, rust: RustIndex, resource: ResourceSpec) -> str:\n',
)
replace(
    "codegen/sdk_codegen.py",
    '    if not operation.response:\n        raise GenerationError(f"operation {operation.operation_id} needs a response projection")\n    raw_parameters = rust.operation(operation.raw_method).parameters\n',
    '    response_type = _response_type(operation)\n    if not response_type:\n        raise GenerationError(f"operation {operation.operation_id} needs a response projection")\n    raw_parameters = rust.operation(operation.raw_method).parameters\n',
)
replace(
    "codegen/sdk_codegen.py",
    '            f"pub async fn {operation.name}_with(&self, {arguments}) -> Result<{operation.response}, SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map(Into::into).map_err(Into::into)\\n}}"\n',
    '            f"pub async fn {operation.name}_with(&self, {arguments}) -> Result<{response_type}, SdkError> {{\\n"\n            f"    {_map_response(operation, f\'self.raw.{operation.raw_method}({call}).await\')}.map_err(Into::into)\\n}}"\n',
)
replace(
    "codegen/sdk_codegen.py",
    '            f"pub async fn {operation.name}(&self) -> Result<{operation.response}, SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({empty}).await.map(Into::into).map_err(Into::into)\\n}}\\n\\n"\n',
    '            f"pub async fn {operation.name}(&self) -> Result<{response_type}, SdkError> {{\\n"\n            f"    {_map_response(operation, f\'self.raw.{operation.raw_method}({empty}).await\')}.map_err(Into::into)\\n}}\\n\\n"\n',
)
replace(
    "codegen/sdk_codegen.py",
    '        f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<{operation.response}, SdkError> {{\\n"\n        f"    self.raw.{operation.raw_method}({call}).await.map(Into::into).map_err(Into::into)\\n}}"\n',
    '        f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<{response_type}, SdkError> {{\\n"\n        f"    {_map_response(operation, f\'self.raw.{operation.raw_method}({call}).await\')}.map_err(Into::into)\\n}}"\n',
)

# Feed raw Rust AST into autoprojection and the coverage probe.
replace(
    "codegen/sdk_codegen.py",
    '        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage)\n',
    '        manifest, projection_report = expand_manifest(openapi, manifest, taxonomy, raw_coverage, rust)\n',
)
replace(
    "scripts/probe_sdk_coverage.py",
    '        json.loads((ROOT / "src/generated/coverage.json").read_text()),\n    )\n',
    '        json.loads((ROOT / "src/generated/coverage.json").read_text()),\n        rust,\n    )\n',
)

# Autoproject inline scalar/list/union response shapes conservatively.
replace(
    "codegen/sdk_autoproject.py",
    'def expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],\n                    raw_coverage: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:\n',
    'def _response_name(resource_path: tuple[str, ...], public_name: str, suffix: str = "Response") -> str:\n    return _resource_name(resource_path) + _pascal(public_name) + suffix\n\n\ndef _resolved_alias(rust: Any, spelling: str) -> Any | None:\n    if rust is None:\n        return None\n    syntax = rust.aliases.get(spelling)\n    seen: set[str] = set()\n    while syntax is not None and syntax.spelling in rust.aliases:\n        if syntax.spelling in seen:\n            return None\n        seen.add(syntax.spelling)\n        syntax = rust.aliases[syntax.spelling]\n    return syntax\n\n\ndef _ensure_ref_union_model(models: dict[str, Any], schemas: dict[str, Any], rust: Any,\n                            raw_enum: str, branches: list[dict[str, Any]], name: str) -> str | None:\n    existing = _existing_model_by_raw(models, raw_enum, False)\n    if existing:\n        return existing\n    refs = [_schema_ref(branch) for branch in branches]\n    if not refs or any(ref is None for ref in refs) or raw_enum not in rust.enums:\n        return None\n    variants = rust.variants(raw_enum)\n    if {variant.payload for variant in variants} != set(refs):\n        return None\n    configured: dict[str, Any] = {}\n    for variant in variants:\n        adapter = _ensure_view_model(models, schemas, variant.payload)\n        configured[variant.name] = {"name": _pascal(variant.payload), "adapter": adapter}\n    models[name] = {\n        "raw": raw_enum,\n        "simple_union": {"bidirectional": True, "variants": configured},\n    }\n    return name\n\n\ndef _inline_response_projection(models: dict[str, Any], schemas: dict[str, Any], rust: Any,\n                                raw_method: str, schema: dict[str, Any], resource_path: tuple[str, ...],\n                                public_name: str) -> tuple[str | None, str | None, str | None]:\n    if schema.get("type") == "string" and not schema.get("format"):\n        return "scalar", "String", None\n    if rust is None:\n        return None, None, "inline_or_unresolved_response"\n    raw_success = rust.operation(raw_method).success_type\n    if schema.get("type") == "array":\n        transport = _resolved_alias(rust, raw_success)\n        if transport is None or transport.constructor != "Vec" or len(transport.arguments) != 1:\n            return None, None, "inline_or_unresolved_response"\n        raw_item = transport.arguments[0].spelling\n        items = schema.get("items", {})\n        item_ref = _schema_ref(items)\n        if item_ref:\n            if item_ref != raw_item:\n                return None, None, "inline_or_unresolved_response"\n            existing = _existing_model_by_raw(models, raw_item, False)\n            return "list", existing or _ensure_view_model(models, schemas, raw_item), None\n        branches = items.get("oneOf", []) or items.get("anyOf", [])\n        name = _response_name(resource_path, public_name, "Item")\n        union = _ensure_ref_union_model(models, schemas, rust, raw_item, branches, name)\n        return ("list", union, None) if union else (None, None, "inline_or_unresolved_response")\n    branches = schema.get("oneOf", []) or schema.get("anyOf", [])\n    if branches:\n        raw_enum = raw_success\n        alias = _resolved_alias(rust, raw_success)\n        if raw_enum not in rust.enums and alias is not None and alias.spelling in rust.enums:\n            raw_enum = alias.spelling\n        name = _response_name(resource_path, public_name)\n        union = _ensure_ref_union_model(models, schemas, rust, raw_enum, branches, name)\n        return ("response", union, None) if union else (None, None, "inline_or_unresolved_response")\n    return None, None, "inline_or_unresolved_response"\n\n\ndef expand_manifest(openapi: Any, manifest: dict[str, Any], taxonomy: dict[str, Any],\n                    raw_coverage: dict[str, Any], rust: Any | None = None) -> tuple[dict[str, Any], dict[str, Any]]:\n',
)
replace(
    "codegen/sdk_autoproject.py",
    '        response = None\n        if response_kind == "json":\n            response_raw = _schema_ref(response_schema or {})\n            if not response_raw:\n                rejected[operation_id] = "inline_or_unresolved_response"\n                continue\n            existing_response = _existing_model_by_raw(models, response_raw, False)\n            response = existing_response or _ensure_view_model(models, openapi.schemas, response_raw)\n',
    '        response = None\n        list_response = None\n        scalar_response = None\n        if response_kind == "json":\n            response_raw = _schema_ref(response_schema or {})\n            if response_raw:\n                existing_response = _existing_model_by_raw(models, response_raw, False)\n                response = existing_response or _ensure_view_model(models, openapi.schemas, response_raw)\n            else:\n                projection, value, reason = _inline_response_projection(\n                    models, openapi.schemas, rust, raw_method, response_schema or {}, resource_path, public_name\n                )\n                if reason:\n                    rejected[operation_id] = reason\n                    continue\n                if projection == "response":\n                    response = value\n                elif projection == "list":\n                    list_response = value\n                elif projection == "scalar":\n                    scalar_response = value\n                else:\n                    rejected[operation_id] = "inline_or_unresolved_response"\n                    continue\n',
)
replace(
    "codegen/sdk_autoproject.py",
    '        if response is not None:\n            item["response"] = response\n        elif response_kind == "empty":\n            item["empty_response"] = True\n',
    '        if response is not None:\n            item["response"] = response\n        elif list_response is not None:\n            item["list_response"] = list_response\n        elif scalar_response is not None:\n            item["scalar_response"] = scalar_response\n        elif response_kind == "empty":\n            item["empty_response"] = True\n',
)

# Tests for the two response modes that do not require a Rust AST.
p = Path("scripts/test_sdk_autoproject.py")
text = p.read_text()
marker = '    def test_equal_depth_aliases_require_review(self):\n'
if marker not in text:
    raise SystemExit("autoproject test marker missing")
tests = '''    def test_projects_inline_scalar_success(self):\n        api = FakeOpenApi()\n        api.operations["signed_url"] = {\n            "responses": {"200": {"content": {"application/json": {"schema": {"type": "string"}}}}},\n            "parameters": [],\n        }\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"signed_url": ["things.signed_url"]}},\n            raw_coverage("signed_url"),\n        )\n        self.assertEqual(1, report["added_count"])\n        self.assertEqual("String", expanded["resources"]["things"]["operations"]["signed_url"]["scalar_response"])\n\n    def test_projects_inline_named_list_success(self):\n        api = FakeOpenApi()\n        api.operations["list_inline_things"] = {\n            "responses": {"200": {"content": {"application/json": {"schema": {\n                "type": "array", "items": {"$ref": "#/components/schemas/Thing"}\n            }}}}},\n            "parameters": [],\n        }\n        class Syntax:\n            spelling = "Vec<Thing>"\n            constructor = "Vec"\n            arguments = (type("Item", (), {"spelling": "Thing"})(),)\n        class Operation:\n            success_type = "ListThingsResponse"\n        class Rust:\n            aliases = {"ListThingsResponse": Syntax()}\n            enums = {}\n            def operation(self, name): return Operation()\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"list_inline_things": ["things.list_inline"]}},\n            raw_coverage("list_inline_things"), Rust(),\n        )\n        self.assertEqual(1, report["added_count"])\n        operation = expanded["resources"]["things"]["operations"]["list_inline"]\n        self.assertEqual("ThingView", operation["list_response"])\n\n'''
p.write_text(text.replace(marker, tests + marker, 1))
