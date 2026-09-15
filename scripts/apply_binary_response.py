from pathlib import Path


def replace(path: str, old: str, new: str) -> None:
    p = Path(path)
    text = p.read_text()
    if old not in text:
        raise SystemExit(f"missing expected text in {path}: {old[:120]!r}")
    p.write_text(text.replace(old, new, 1))


replace(
    "codegen/sdk-semantics.schema.json",
    '"request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},',
    '"request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "binary_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},',
)

replace(
    "codegen/sdk_autoproject.py",
    '    if "application/json" not in content:\n        return None, None, "non_json_success"\n    if set(content) != {"application/json"}:\n        return None, None, "multiple_success_media"\n    return "json", content["application/json"].get("schema", {}), None\n',
    '    if "application/json" in content:\n        if set(content) != {"application/json"}:\n            return None, None, "multiple_success_media"\n        return "json", content["application/json"].get("schema", {}), None\n    if len(content) == 1:\n        payload = next(iter(content.values()))\n        schema = payload.get("schema", {})\n        if schema.get("type") == "string" and schema.get("format") == "binary":\n            return "binary", schema, None\n    return None, None, "non_json_success"\n',
)
replace(
    "codegen/sdk_autoproject.py",
    '        elif response_kind == "empty":\n            item["empty_response"] = True\n',
    '        elif response_kind == "empty":\n            item["empty_response"] = True\n        elif response_kind == "binary":\n            item["binary_response"] = True\n',
)

replace(
    "codegen/sdk_codegen.py",
    '    request_raw: str | None = None\n    empty_response: bool = False\n',
    '    request_raw: str | None = None\n    empty_response: bool = False\n    binary_response: bool = False\n',
)
replace(
    "codegen/sdk_codegen.py",
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "binary_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
)
replace(
    "codegen/sdk_codegen.py",
    '            empty_response = item.get("empty_response", False)\n            success_modes = int(bool(response)) + int(bool(item.get("stream"))) + int(empty_response)\n',
    '            empty_response = item.get("empty_response", False)\n            binary_response = item.get("binary_response", False)\n            success_modes = int(bool(response)) + int(bool(item.get("stream"))) + int(empty_response) + int(binary_response)\n',
)
replace(
    "codegen/sdk_codegen.py",
    '                    f"operation {module}.{public_name} requires exactly one response, stream or empty_response projection"\n',
    '                    f"operation {module}.{public_name} requires exactly one response, stream, empty_response or binary_response projection"\n',
)
replace(
    "codegen/sdk_codegen.py",
    '            if response and not item.get("stream"):\n',
    '            if binary_response:\n                success = [value for status, value in wire_operation.get("responses", {}).items()\n                           if str(status).startswith("2")]\n                if len(success) != 1 or len(success[0].get("content", {})) != 1:\n                    raise GenerationError(f"binary response drift for {operation_id}")\n                payload = next(iter(success[0]["content"].values()))\n                schema = payload.get("schema", {})\n                if schema.get("type") != "string" or schema.get("format") != "binary":\n                    raise GenerationError(f"binary response drift for {operation_id}")\n                if raw_operation.success_type != "bytes::Bytes":\n                    raise GenerationError(f"raw binary response drift for {raw_method}")\n            if response and not item.get("stream"):\n',
)
replace(
    "codegen/sdk_codegen.py",
    '                                            stream_policy(item.get("stream")), request_raw, empty_response))\n',
    '                                            stream_policy(item.get("stream")), request_raw, empty_response, binary_response))\n',
)
replace(
    "codegen/sdk_codegen.py",
    '    if operation.empty_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<(), SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n',
    '    if operation.empty_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<(), SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n    if operation.binary_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<bytes::Bytes, SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n',
)

p = Path("scripts/test_sdk_facade.py")
text = p.read_text()
marker = '    def test_empty_success_is_validated_and_emitted_as_unit(self):\n'
if marker not in text:
    raise SystemExit("test insertion marker missing")
test = '''    def test_binary_success_is_validated_and_emitted_as_bytes(self):\n        document = openapi_document()\n        document["paths"]["/animals/{animal_id}/content"] = {\n            "get": {\n                "operationId": "download_animal",\n                "parameters": [{\n                    "name": "animal_id", "in": "path", "required": True,\n                    "schema": {"type": "string"},\n                }],\n                "responses": {"200": {\n                    "content": {"application/octet-stream": {\n                        "schema": {"type": "string", "format": "binary"}\n                    }}\n                }},\n            }\n        }\n        overlay = manifest()\n        overlay["resources"]["zoo"]["operations"]["download"] = {\n            "operation_id": "download_animal", "binary_response": True,\n        }\n        self.openapi.write_text(json.dumps(document))\n        self.overlay.write_text(json.dumps(overlay))\n        (self.raw / "client.rs").write_text(CLIENT + """\nimpl HttpClient {\n    pub async fn download_animal(&self, animal_id: impl AsRef<str>) -> Result<bytes::Bytes, Error> { todo!() }\n}\n""")\n        resource = (self.generate() / "zoo.rs").read_text()\n        self.assertIn(\n            "pub async fn download(&self, animal_id: impl AsRef<str>) -> Result<bytes::Bytes, SdkError>",\n            resource,\n        )\n\n'''
p.write_text(text.replace(marker, test + marker, 1))
