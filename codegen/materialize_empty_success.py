from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    source = target.read_text()
    count = source.count(old)
    if count != 1:
        raise SystemExit(f"expected one match in {path}, got {count}: {old[:80]!r}")
    target.write_text(source.replace(old, new, 1))


replace_once(
    "codegen/sdk-semantics.schema.json",
    '      "request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "stream": {"$ref": "#/$defs/stream"},\n',
    '      "request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},\n',
)

replace_once(
    "codegen/sdk_codegen.py",
    '    request_raw: str | None = None\n',
    '    request_raw: str | None = None\n    empty_response: bool = False\n',
)
replace_once(
    "codegen/sdk_codegen.py",
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
)
replace_once(
    "codegen/sdk_codegen.py",
    '            request, response = item.get("request"), item.get("response")\n            for referenced in (request, response):\n',
    '            request, response = item.get("request"), item.get("response")\n            empty_response = item.get("empty_response", False)\n            success_modes = int(bool(response)) + int(bool(item.get("stream"))) + int(empty_response)\n            if success_modes != 1:\n                raise GenerationError(\n                    f"operation {module}.{public_name} requires exactly one response, stream or empty_response projection"\n                )\n            for referenced in (request, response):\n',
)
replace_once(
    "codegen/sdk_codegen.py",
    '            if response and not item.get("stream"):\n                model = next(model for model in models if model.name == response)\n',
    '            if empty_response:\n                success = [value for status, value in wire_operation.get("responses", {}).items()\n                           if str(status).startswith("2")]\n                if len(success) != 1 or success[0].get("content"):\n                    raise GenerationError(f"empty response drift for {operation_id}")\n                if raw_operation.success_type != "()":\n                    raise GenerationError(f"raw empty response drift for {raw_method}")\n            if response and not item.get("stream"):\n                model = next(model for model in models if model.name == response)\n',
)
replace_once(
    "codegen/sdk_codegen.py",
    '            operations.append(OperationSpec(public_name, operation_id, raw_method, request, response,\n                                            tuple(item.get("request_overrides", {}).items()),\n                                            stream_policy(item.get("stream")), request_raw))\n',
    '            operations.append(OperationSpec(public_name, operation_id, raw_method, request, response,\n                                            tuple(item.get("request_overrides", {}).items()),\n                                            stream_policy(item.get("stream")), request_raw, empty_response))\n',
)
replace_once(
    "codegen/sdk_codegen.py",
    '    if not operation.response:\n        raise GenerationError(f"operation {operation.operation_id} needs a response projection")\n',
    '    if operation.empty_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<(), SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n    if not operation.response:\n        raise GenerationError(f"operation {operation.operation_id} needs a response projection")\n',
)

replace_once(
    "codegen/sdk_autoproject.py",
    '''def _success_json_schema(operation: dict[str, Any]) -> tuple[dict[str, Any] | None, str | None]:\n    success = [response for status, response in operation.get("responses", {}).items()\n               if str(status).startswith("2")]\n    if len(success) != 1:\n        return None, "multiple_success_contracts"\n    content = success[0].get("content", {})\n    if "application/json" not in content:\n        return None, "non_json_success"\n    if set(content) != {"application/json"}:\n        return None, "multiple_success_media"\n    return content["application/json"].get("schema", {}), None\n''',
    '''def _success_contract(operation: dict[str, Any]) -> tuple[str | None, dict[str, Any] | None, str | None]:\n    success = [response for status, response in operation.get("responses", {}).items()\n               if str(status).startswith("2")]\n    if len(success) != 1:\n        return None, None, "multiple_success_contracts"\n    content = success[0].get("content", {})\n    if not content:\n        return "empty", None, None\n    if "application/json" not in content:\n        return None, None, "non_json_success"\n    if set(content) != {"application/json"}:\n        return None, None, "multiple_success_media"\n    return "json", content["application/json"].get("schema", {}), None\n''',
)
replace_once(
    "codegen/sdk_autoproject.py",
    '''        response_schema, reason = _success_json_schema(operation)\n        if reason:\n            rejected[operation_id] = reason\n            continue\n        response_raw = _schema_ref(response_schema or {})\n        if not response_raw:\n            rejected[operation_id] = "inline_or_unresolved_response"\n            continue\n        existing_response = _existing_model_by_raw(models, response_raw, False)\n        response = existing_response or _ensure_view_model(models, openapi.schemas, response_raw)\n''',
    '''        response_kind, response_schema, reason = _success_contract(operation)\n        if reason:\n            rejected[operation_id] = reason\n            continue\n        response = None\n        if response_kind == "json":\n            response_raw = _schema_ref(response_schema or {})\n            if not response_raw:\n                rejected[operation_id] = "inline_or_unresolved_response"\n                continue\n            existing_response = _existing_model_by_raw(models, response_raw, False)\n            response = existing_response or _ensure_view_model(models, openapi.schemas, response_raw)\n''',
)
replace_once(
    "codegen/sdk_autoproject.py",
    '''        item: dict[str, Any] = {"operation_id": operation_id, "response": response}\n        if raw_method != operation_id:\n''',
    '''        item: dict[str, Any] = {"operation_id": operation_id}\n        if response is not None:\n            item["response"] = response\n        elif response_kind == "empty":\n            item["empty_response"] = True\n        if raw_method != operation_id:\n''',
)

replace_once(
    "codegen/sdk_contracts.py",
    '''        if not any("application/json" in response.get("content", {}) for response in responses):\n            reasons.append("non_json_success")\n''',
    '''        if (any(response.get("content") for response in responses)\n                and not any("application/json" in response.get("content", {}) for response in responses)):\n            reasons.append("non_json_success")\n''',
)

replace_once(
    "scripts/test_sdk_autoproject.py",
    '''    def test_equal_depth_aliases_require_review(self):\n''',
    '''    def test_projects_empty_success_without_fake_response_model(self):\n        api = FakeOpenApi()\n        api.operations["delete_thing"] = {\n            "responses": {"204": {"description": "No Content"}},\n            "parameters": [],\n        }\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"delete_thing": ["things.delete"]}},\n            raw_coverage("delete_thing"),\n        )\n        self.assertEqual(1, report["added_count"])\n        operation = expanded["resources"]["things"]["operations"]["delete"]\n        self.assertTrue(operation["empty_response"])\n        self.assertNotIn("response", operation)\n\n    def test_binary_success_stays_review_debt(self):\n        api = FakeOpenApi()\n        api.operations["download_thing"] = {\n            "responses": {"200": {"content": {"application/octet-stream": {"schema": {"type": "string", "format": "binary"}}}}},\n            "parameters": [],\n        }\n        _, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"download_thing": ["things.download"]}},\n            raw_coverage("download_thing"),\n        )\n        self.assertEqual("non_json_success", report["rejected"]["download_thing"])\n\n    def test_equal_depth_aliases_require_review(self):\n''',
)

replace_once(
    "scripts/test_sdk_facade.py",
    '''    def test_inventory_includes_unmapped_operations(self):\n''',
    '''    def test_empty_success_is_validated_and_emitted_as_unit(self):\n        document = openapi_document()\n        document["paths"]["/animals/{animal_id}"] = {\n            "delete": {\n                "operationId": "delete_animal",\n                "parameters": [{\n                    "name": "animal_id", "in": "path", "required": True,\n                    "schema": {"type": "string"},\n                }],\n                "responses": {"204": {"description": "No Content"}},\n            }\n        }\n        overlay = manifest()\n        overlay["resources"]["zoo"]["operations"]["delete"] = {\n            "operation_id": "delete_animal", "empty_response": True,\n        }\n        self.openapi.write_text(json.dumps(document))\n        self.overlay.write_text(json.dumps(overlay))\n        (self.raw / "client.rs").write_text(CLIENT + """\nimpl HttpClient {\n    pub async fn delete_animal(&self, animal_id: impl AsRef<str>) -> Result<(), Error> { todo!() }\n}\n""")\n        resource = (self.generate() / "zoo.rs").read_text()\n        self.assertIn(\n            "pub async fn delete(&self, animal_id: impl AsRef<str>) -> Result<(), SdkError>",\n            resource,\n        )\n        self.assertIn("self.raw.delete_animal(animal_id.as_ref()).await.map_err(Into::into)", resource)\n\n    def test_empty_success_raw_drift_fails_closed(self):\n        document = openapi_document()\n        document["paths"]["/delete"] = {\n            "delete": {"operationId": "delete_animal", "responses": {"204": {"description": "No Content"}}}\n        }\n        overlay = manifest()\n        overlay["resources"]["zoo"]["operations"]["delete"] = {\n            "operation_id": "delete_animal", "empty_response": True,\n        }\n        self.openapi.write_text(json.dumps(document))\n        self.overlay.write_text(json.dumps(overlay))\n        (self.raw / "client.rs").write_text(CLIENT + """\nimpl HttpClient {\n    pub async fn delete_animal(&self) -> Result<AnimalResponse, Error> { todo!() }\n}\n""")\n        with self.assertRaisesRegex(sdk_codegen.GenerationError, "raw empty response drift"):\n            self.generate()\n\n    def test_inventory_includes_unmapped_operations(self):\n''',
)
