#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f"expected one replacement in {path}: found {count}\n{old[:160]}")
    path.write_text(text.replace(old, new, 1))


def append_before_main(path: Path, addition: str) -> None:
    marker = '\n\nif __name__ == "__main__":\n'
    text = path.read_text()
    if addition.strip() in text:
        return
    if marker not in text:
        raise RuntimeError(f"main marker missing in {path}")
    path.write_text(text.replace(marker, "\n\n" + addition.rstrip() + marker, 1))


def update_taxonomy_harvester() -> None:
    path = ROOT / "scripts/sdk_taxonomy.py"
    replace_once(
        path,
        '''            endpoint = parse_ts_function(root / "src/funcs" / f"{imports[function]}.ts")
            records.append({
                **endpoint,
                "function": function,
                "public_path": ".".join((*resource, match.group(1))),
                "normalized_public_path": normalize_public_path([*resource, match.group(1)]),
            })
''',
        '''            endpoint = parse_ts_function(root / "src/funcs" / f"{imports[function]}.ts")
            record = {
                **endpoint,
                "function": function,
                "public_path": ".".join((*resource, match.group(1))),
                "normalized_public_path": normalize_public_path([*resource, match.group(1)]),
            }
            if re.search(
                r"\\)\\s*:\\s*Promise<\\s*ReadableStream<\\s*Uint8Array\\s*>\\s*>",
                segment,
                re.MULTILINE | re.DOTALL,
            ):
                record["response_transport"] = "binary_stream"
            records.append(record)
''',
    )
    replace_once(
        path,
        '''            http_method = http_path = operation_id = None
            for inner in ast.walk(statement):
''',
        '''            http_method = http_path = operation_id = accept_media = None
            streamed_response = False
            for inner in ast.walk(statement):
''',
    )
    replace_once(
        path,
        '''                if name == "_build_request":
                    http_method = literal_keyword(inner, "method")
                    http_path = literal_keyword(inner, "path")
                elif name == "HookContext":
                    operation_id = literal_keyword(inner, "operation_id")
            if http_method and http_path and operation_id:
                records.append({
                    "http_method": http_method,
                    "http_path": normalized_http_path(http_path),
                    "operation_id": operation_id,
                    "transport_variant": transport_variant(http_path),
                    "public_path": ".".join((*resource, statement.name)),
                    "normalized_public_path": normalize_public_path([*resource, statement.name]),
                    "source_file": path.relative_to(root).as_posix(),
                })
''',
        '''                if name == "_build_request":
                    http_method = literal_keyword(inner, "method")
                    http_path = literal_keyword(inner, "path")
                    accept_media = literal_keyword(inner, "accept_header_value")
                elif name == "HookContext":
                    operation_id = literal_keyword(inner, "operation_id")
                elif name == "do_request":
                    streamed_response = any(
                        keyword.arg == "stream"
                        and isinstance(keyword.value, ast.Constant)
                        and keyword.value.value is True
                        for keyword in inner.keywords
                    )
            if http_method and http_path and operation_id:
                record = {
                    "http_method": http_method,
                    "http_path": normalized_http_path(http_path),
                    "operation_id": operation_id,
                    "transport_variant": transport_variant(http_path),
                    "public_path": ".".join((*resource, statement.name)),
                    "normalized_public_path": normalize_public_path([*resource, statement.name]),
                    "source_file": path.relative_to(root).as_posix(),
                }
                if streamed_response and accept_media != "text/event-stream":
                    record["response_transport"] = "binary_stream"
                records.append(record)
''',
    )
    replace_once(
        path,
        '''        if "function" in record:
            public["function"] = record["function"]
        matched[raw["operation_id"]].append(public)
''',
        '''        if "function" in record:
            public["function"] = record["function"]
        if record.get("response_transport"):
            public["response_transport"] = record["response_transport"]
        matched[raw["operation_id"]].append(public)
''',
    )


def update_taxonomy_compactor() -> None:
    path = ROOT / "scripts/sdk_taxonomy_inventory.py"
    replace_once(
        path,
        '''    operations: dict[str, list[str]] = {}
    alias_differences: list[dict[str, Any]] = []
    divergences: list[dict[str, Any]] = []
''',
        '''    operations: dict[str, list[str]] = {}
    operation_transports: dict[str, str] = {}
    alias_differences: list[dict[str, Any]] = []
    divergences: list[dict[str, Any]] = []
''',
    )
    replace_once(
        path,
        '''        primary = typescript or python
        if primary:
            operations[operation["operation_id"]] = primary
        if not typescript or not python or typescript == python:
            continue
''',
        '''        primary = typescript or python
        if primary:
            operations[operation["operation_id"]] = primary
        ts_transports = sorted({
            record["response_transport"]
            for record in operation["typescript"]
            if record.get("response_transport")
        })
        py_transports = sorted({
            record["response_transport"]
            for record in operation["python"]
            if record.get("response_transport")
        })
        if len(ts_transports) == 1 and ts_transports == py_transports:
            operation_transports[operation["operation_id"]] = ts_transports[0]
        elif ts_transports and py_transports and ts_transports != py_transports:
            divergences.append({
                "operation_id": operation["operation_id"],
                "typescript": ts_transports,
                "python": py_transports,
                "reason": "response_transport_conflict",
            })
        if not typescript or not python or typescript == python:
            continue
''',
    )
    replace_once(path, '        "schema_version": 2,\n', '        "schema_version": 3,\n')
    replace_once(
        path,
        '''        "operations": operations,
        "openapi_only": harvested["openapi_only"],
''',
        '''        "operations": operations,
        "operation_transports": operation_transports,
        "openapi_only": harvested["openapi_only"],
''',
    )


def update_autoprojector() -> None:
    path = ROOT / "codegen/sdk_autoproject.py"
    replace_once(
        path,
        '''def _success_contract(operation: dict[str, Any]) -> tuple[str | None, dict[str, Any] | None, str | None]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None, None, "multiple_success_contracts"
    content = success[0].get("content", {})
    if not content:
        return "empty", None, None
    if "application/json" in content:
        if set(content) != {"application/json"}:
            return None, None, "multiple_success_media"
        return "json", content["application/json"].get("schema", {}), None
    if len(content) == 1:
        payload = next(iter(content.values()))
        schema = payload.get("schema", {})
        if schema.get("type") == "string" and schema.get("format") == "binary":
            return "binary", schema, None
    return None, None, "non_json_success"
''',
        '''def _binary_media(operation: dict[str, Any]) -> dict[str, dict[str, Any]]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return {}
    result = {}
    for media, payload in success[0].get("content", {}).items():
        schema = payload.get("schema", {})
        if schema.get("type") == "string" and schema.get("format") == "binary":
            result[media] = schema
    return result


def _success_contract(operation: dict[str, Any], preferred_transport: str | None = None) -> tuple[str | None, dict[str, Any] | None, str | None]:
    success = [response for status, response in operation.get("responses", {}).items()
               if str(status).startswith("2")]
    if len(success) != 1:
        return None, None, "multiple_success_contracts"
    content = success[0].get("content", {})
    if not content:
        return "empty", None, None
    if preferred_transport == "binary_stream":
        binary = _binary_media(operation)
        if len(binary) == 1:
            return "binary", next(iter(binary.values())), None
        return None, None, "official_binary_transport_mismatch"
    if "application/json" in content:
        if set(content) != {"application/json"}:
            return None, None, "multiple_success_media"
        return "json", content["application/json"].get("schema", {}), None
    if len(content) == 1:
        binary = _binary_media(operation)
        if len(binary) == 1:
            return "binary", next(iter(binary.values())), None
    return None, None, "non_json_success"
''',
    )
    replace_once(
        path,
        '''    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_coverage.get("operations", []) if item.get("upstream", True)}
''',
        '''    raw_operations = raw_coverage.get("operations", [])
    raw_methods = {item["operation_id"]: item["rust_method"]
                   for item in raw_operations if item.get("upstream", True)}
''',
    )
    replace_once(
        path,
        '''        response_kind, response_schema, reason = _success_contract(operation)
        if reason:
            rejected[operation_id] = reason
            continue
''',
        '''        response_transport = taxonomy.get("operation_transports", {}).get(operation_id)
        response_kind, response_schema, reason = _success_contract(operation, response_transport)
        if reason:
            rejected[operation_id] = reason
            continue
        if response_kind == "binary":
            binary_media = set(_binary_media(operation))
            route_path = operation.get("x-sdk-path", "").split("#", 1)[0].rstrip("/") or "/"
            route_method = operation.get("x-sdk-method", "").upper()
            candidates = [
                item for item in raw_operations
                if item.get("method") == route_method
                and (item.get("path", "").split("#", 1)[0].rstrip("/") or "/") == route_path
                and item.get("success_media")
                and set(item["success_media"]).issubset(binary_media)
            ]
            if len(candidates) == 1:
                raw_method = candidates[0]["rust_method"] + "_stream"
            else:
                rejected[operation_id] = "raw_binary_transport_mapping"
                continue
''',
    )


def update_facade_compiler() -> None:
    path = ROOT / "codegen/sdk_codegen.py"
    replace_once(
        path,
        '''                if raw_operation.success_type != "bytes::Bytes":
                    raise GenerationError(f"raw binary response drift for {raw_method}")
''',
        '''                transport = parse_type(raw_operation.success_type)
                if transport.constructor != "futures_util::stream::BoxStream" or len(transport.arguments) != 2:
                    raise GenerationError(f"raw binary response is not an owned BoxStream: {raw_method}")
                lifetime, event = transport.arguments
                if lifetime.spelling != "'static" or event.constructor != "Result" or tuple(arg.spelling for arg in event.arguments) != ("bytes::Bytes", "reqwest::Error"):
                    raise GenerationError(f"raw binary response ownership/item drift: {raw_method}")
''',
    )
    replace_once(
        path,
        '''    if operation.binary_response:
        separator = ", " if arguments else ""
        return (
            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<bytes::Bytes, SdkError> {{\\n"
            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"
        )
''',
        '''    if operation.binary_response:
        separator = ", " if arguments else ""
        return (
            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<BinaryStream, SdkError> {{\\n"
            f"    let bytes = self.raw.{operation.raw_method}({call}).await.map_err(SdkError::from)?;\\n"
            f"    let chunks = bytes.map(|chunk| chunk.map_err(|error| {{\\n"
            f"        let transport = TransportError::from(crate::generated::client::HttpError::Network(error));\\n"
            f"        SdkError::from(transport)\\n"
            f"    }}));\\n"
            f"    Ok(Box::pin(chunks))\\n}}"
        )
''',
    )
    replace_once(
        path,
        '''def _emit_resource(resource: ResourceSpec, rust: RustIndex, resources: tuple[ResourceSpec, ...]) -> str:
    streaming = any(operation.stream for operation in resource.operations)
    imports = ("use futures_util::StreamExt;\\nuse crate::streaming;\\n"
               "use crate::generated::types::*;\\n" if streaming else "")
''',
        '''def _emit_resource(resource: ResourceSpec, rust: RustIndex, resources: tuple[ResourceSpec, ...]) -> str:
    streaming = any(operation.stream for operation in resource.operations)
    binary = any(operation.binary_response for operation in resource.operations)
    imports = "use futures_util::StreamExt;\\n" if streaming or binary else ""
    if streaming:
        imports += "use crate::streaming;\\nuse crate::generated::types::*;\\n"
''',
    )
    replace_once(
        path,
        '''    exported_types.extend(operation.stream.type
                          for resource in ir.resources for operation in resource.operations
                          if operation.stream)
    models = ", ".join(exported_types)
''',
        '''    exported_types.extend(operation.stream.type
                          for resource in ir.resources for operation in resource.operations
                          if operation.stream)
    if any(operation.binary_response for resource in ir.resources for operation in resource.operations):
        exported_types.append("BinaryStream")
    models = ", ".join(exported_types)
''',
    )
    replace_once(
        path,
        '''    aliases = []
    for resource in ir.resources:
''',
        '''    aliases = []
    if any(operation.binary_response for resource in ir.resources for operation in resource.operations):
        aliases.append(
            "pub type BinaryStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, SdkError>> + Send + 'static>>;"
        )
    for resource in ir.resources:
''',
    )


def update_tests() -> None:
    taxonomy = ROOT / "scripts/test_sdk_taxonomy.py"
    replace_once(
        taxonomy,
        '''                '  async list(): Promise<void> { return filesList(this); }\\n'
''',
        '''                '  async list(): Promise<ReadableStream<Uint8Array>> { return filesList(this); }\\n'
''',
    )
    replace_once(
        taxonomy,
        '''            self.assertEqual("files_list", records[0]["operation_id"])
''',
        '''            self.assertEqual("files_list", records[0]["operation_id"])
            self.assertEqual("binary_stream", records[0]["response_transport"])
''',
    )
    addition = '''    def test_python_binary_stream_transport_is_harvested(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            client = root / "src/mistralai/client"
            client.mkdir(parents=True)
            (client / "sdk.py").write_text(
                "class Mistral(BaseSDK):\\n"
                "    def download(self):\\n"
                "        self._build_request(method='GET', path='/v1/files/x', accept_header_value='application/octet-stream')\\n"
                "        HookContext(operation_id='files_download')\\n"
                "        self.do_request(stream=True)\\n"
            )
            records, _ = sdk_taxonomy.parse_python(root)
            self.assertEqual("binary_stream", records[0]["response_transport"])

    def test_compact_inventory_requires_cross_sdk_transport_agreement(self):
        lock = {
            "upstream_repository": "mistralai/spec",
            "upstream_commit": "a" * 40,
            "official_typescript_sdk_repository": "mistralai/client-ts",
            "official_typescript_sdk_commit": "b" * 40,
            "official_python_sdk_repository": "mistralai/client-python",
            "official_python_sdk_commit": "c" * 40,
        }
        coverage = {"operations": [{
            "operation_id": "files_download", "method": "GET",
            "path": "/v1/files/x", "upstream": True,
        }]}
        common = {
            "operation_id": "files_download", "http_method": "GET",
            "http_path": "/v1/files/x", "transport_variant": "default",
            "public_path": "files.download", "normalized_public_path": "files.download",
            "response_transport": "binary_stream",
        }
        harvested = sdk_taxonomy.build_inventory(lock, coverage, [common], [common])
        inventory = sdk_taxonomy_inventory.compact_inventory(harvested)
        self.assertEqual(3, inventory["schema_version"])
        self.assertEqual("binary_stream", inventory["operation_transports"]["files_download"])
'''
    append_before_main(taxonomy, addition)

    auto = ROOT / "scripts/test_sdk_autoproject.py"
    addition = '''    def test_official_binary_transport_selects_mixed_media_binary_variant(self):
        api = FakeOpenApi()
        api.operations["sample_audio"] = {
            "x-sdk-path": "/v1/audio/voices/{voice_id}/sample",
            "x-sdk-method": "get",
            "responses": {"200": {"content": {
                "application/json": {"schema": {"$ref": "#/components/schemas/Thing"}},
                "audio/wav": {"schema": {"type": "string", "format": "binary"}},
            }}},
            "parameters": [],
        }
        coverage = {"operations": [
            {"operation_id": "sample_audio", "rust_method": "sample_audio", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["application/json", "audio/wav"], "upstream": True},
            {"operation_id": "sample_audio_wav", "rust_method": "sample_audio_wav", "method": "GET",
             "path": "/v1/audio/voices/{voice_id}/sample", "success_media": ["audio/wav"], "upstream": False},
        ]}
        expanded, report = sdk_autoproject.expand_manifest(
            api, manifest(),
            {"operations": {"sample_audio": ["audio.voices.get_sample_audio"]},
             "operation_transports": {"sample_audio": "binary_stream"}},
            coverage,
        )
        self.assertEqual(1, report["added_count"])
        operation = expanded["resources"]["audio_voices"]["operations"]["get_sample_audio"]
        self.assertTrue(operation["binary_response"])
        self.assertEqual("sample_audio_wav_stream", operation["raw_method"])
'''
    append_before_main(auto, addition)

    facade = ROOT / "scripts/test_sdk_facade.py"
    replace_once(facade, "def test_binary_success_is_validated_and_emitted_as_bytes(self):", "def test_binary_success_is_validated_and_emitted_as_stream(self):")
    replace_once(
        facade,
        '''    pub async fn download_animal(&self, animal_id: impl AsRef<str>) -> Result<bytes::Bytes, Error> { todo!() }
''',
        '''    pub async fn download_animal(&self, animal_id: impl AsRef<str>) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
''',
    )
    replace_once(
        facade,
        '''            "pub async fn download(&self, animal_id: impl AsRef<str>) -> Result<bytes::Bytes, SdkError>",
''',
        '''            "pub async fn download(&self, animal_id: impl AsRef<str>) -> Result<BinaryStream, SdkError>",
''',
    )


def update_generator_patch() -> str:
    lock = json.loads((ROOT / "codegen.lock").read_text())
    with tempfile.TemporaryDirectory(prefix="binary-generator-") as temporary:
        source = Path(temporary)
        subprocess.run(["git", "init", "-q", source], check=True)
        subprocess.run([
            "git", "fetch", "-q", "--depth=1",
            f"https://github.com/{lock['generator_repository']}.git",
            lock["generator_commit"],
        ], cwd=source, check=True)
        subprocess.run(["git", "checkout", "-q", "--detach", "FETCH_HEAD"], cwd=source, check=True)
        subprocess.run(["git", "apply", ROOT / "codegen/patches/generator.patch"], cwd=source, check=True)
        path = source / "src/client_generator.rs"
        text = path.read_text()
        old = '''        let accept = success.accept;
        let error_handling = self.generate_error_handling(op, success);
'''
        new = '''        let binary_stream_error_handling = if matches!(success.body, ClientSuccessBody::Binary) {
            Some(self.generate_binary_stream_error_handling(op, success.clone()))
        } else {
            None
        };
        let accept = success.accept;
        let error_handling = self.generate_error_handling(op, success);
'''
        if text.count(old) != 1:
            raise RuntimeError("generator single-operation insertion point drift")
        text = text.replace(old, new, 1)
        old = '''        quote! {
            #doc_comment
            pub async fn #method_name(
                &self,
                #request_param
            ) -> Result<#response_type, ApiOpError<#op_error_type>> {
                #url_construction

                let mut req = #http_method_call;
                #stream_parameter
                #request_body

                #query_params
                #header_params
                #cookie_params

                // Apply configured authentication (T3). Was previously
                // hardcoded to bearer_auth regardless of GeneratorConfig.
                #auth_application

                // Add custom headers
                #custom_headers

                // Keep content negotiation aligned with the generated return type,
                // replacing any custom Accept value for this operation.
                #accept_header

                let response = req.send().await?;
                #error_handling
            }
        }
'''
        new = '''        let binary_stream_method = if let Some(binary_error_handling) = binary_stream_error_handling {
            let stream_method_name = format_ident!("{}_stream", method_name);
            quote! {
                /// Stream the binary response body without buffering it in memory.
                pub async fn #stream_method_name(
                    &self,
                    #request_param
                ) -> Result<
                    futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>,
                    ApiOpError<#op_error_type>,
                > {
                    #url_construction
                    let mut req = #http_method_call;
                    #stream_parameter
                    #request_body
                    #query_params
                    #header_params
                    #cookie_params
                    #auth_application
                    #custom_headers
                    #accept_header
                    let response = req.send().await?;
                    #binary_error_handling
                }
            }
        } else {
            TokenStream::new()
        };

        quote! {
            #doc_comment
            pub async fn #method_name(
                &self,
                #request_param
            ) -> Result<#response_type, ApiOpError<#op_error_type>> {
                #url_construction

                let mut req = #http_method_call;
                #stream_parameter
                #request_body

                #query_params
                #header_params
                #cookie_params

                // Apply configured authentication (T3). Was previously
                // hardcoded to bearer_auth regardless of GeneratorConfig.
                #auth_application

                // Add custom headers
                #custom_headers

                // Keep content negotiation aligned with the generated return type,
                // replacing any custom Accept value for this operation.
                #accept_header

                let response = req.send().await?;
                #error_handling
            }

            #binary_stream_method
        }
'''
        if text.count(old) != 1:
            raise RuntimeError("generator operation body drift")
        text = text.replace(old, new, 1)
        marker = '''    /// Generate error handling.
'''
        method = '''    fn generate_binary_stream_error_handling(
        &self,
        op: &OperationInfo,
        success: ClientSuccessSelection<'_>,
    ) -> TokenStream {
        let op_error_type = self.op_error_type_token(op);
        let success_status_guard = Self::success_status_guard(&success.statuses);
        let selected_status = if success.statuses.is_empty() {
            "any declared 2xx response".to_string()
        } else {
            success.statuses.join(", ")
        };
        let error_match_arms = self.generate_error_match_arms(op);
        quote! {
            let status = response.status();
            let status_code = status.as_u16();
            let headers = response.headers().clone();
            if #success_status_guard {
                Ok(Box::pin(response.bytes_stream()))
            } else {
                if status.is_success() {
                    return Err(ApiOpError::Api(ApiError {
                        status: status_code,
                        headers,
                        body: String::new(),
                        raw_body: Vec::new(),
                        typed: None,
                        parse_error: Some(format!(
                            "unexpected successful status {}; generated return type selects `{}`; live response body was not buffered",
                            status_code,
                            #selected_status,
                        )),
                    }));
                }
                let body_bytes = __read_bounded_response_body(
                    response,
                    self.max_response_body_bytes,
                ).await?;
                let raw_body = body_bytes;
                let body_text = String::from_utf8_lossy(&raw_body).into_owned();
                let typed: Option<#op_error_type>;
                let parse_error: Option<String>;
                #error_match_arms
                Err(ApiOpError::Api(ApiError {
                    status: status_code,
                    headers,
                    body: body_text,
                    raw_body,
                    typed,
                    parse_error,
                }))
            }
        }
    }

'''
        if text.count(marker) != 1:
            raise RuntimeError("generator error handling marker drift")
        text = text.replace(marker, method + marker, 1)
        path.write_text(text)
        patch = subprocess.check_output(
            ["git", "diff", "--", "src/client_generator.rs", "src/generator.rs"],
            cwd=source,
            text=True,
        )
    patch_path = ROOT / "codegen/patches/generator.patch"
    patch_path.write_text(patch)
    digest = hashlib.sha256(patch.encode()).hexdigest()
    lock["generator_patch_sha256"] = digest
    (ROOT / "codegen.lock").write_text(json.dumps(lock, indent=2) + "\n")
    doc = ROOT / "codegen/OPENAPI_TO_RUST_UPSTREAM.md"
    text = doc.read_text()
    text = re.sub(r"patch_sha256: [0-9a-f]{64}", f"patch_sha256: {digest}", text, count=1)
    if "## 9. Binary success bodies need a streaming client variant" not in text:
        text += '''\n## 9. Binary success bodies need a streaming client variant\n\n```yaml\ntitle: Generate an owned streaming variant for binary response bodies\npriority: critical\ncomponent: src/client_generator.rs\n```\n\n### Actual\n\n```text\nBinary success responses are buffered into bytes::Bytes and therefore inherit the global in-memory response limit.\n```\n\n### Required API shape\n\n```rust\nasync fn download_stream(...) -> Result<\n    futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>,\n    ApiOpError<_>,\n>\n```\n\nThe existing buffered method remains available for compatibility. The streaming variant must preserve the same path, parameters, authentication, Accept header, status selection, and bounded error-body handling.\n\n### Acceptance criteria\n\n```yaml\n- existing buffered binary method remains unchanged\n- companion streaming method is additive\n- success body is never buffered\n- returned stream is owned, Send and static\n- dropping the stream cancels response consumption\n- non-success responses remain bounded and inspectable\n- content negotiation matches the binary media type\n```\n'''
    doc.write_text(text)
    return digest


def update_transport_test() -> None:
    path = ROOT / "tests/transports.rs"
    text = path.read_text()
    marker = '''    assert!(headers.to_lowercase().contains("accept: audio/wav"));
}
'''
    addition = '''    assert!(headers.to_lowercase().contains("accept: audio/wav"));

    let (url, worker) = server("200 OK", "application/octet-stream", bytes);
    let streamed = Client::new()
        .with_base_url(&url)
        .files_api_routes_download_file_stream("streamed")
        .await
        .unwrap()
        .fold(Vec::new(), |mut body, chunk| async move {
            body.extend_from_slice(&chunk.unwrap());
            body
        })
        .await;
    assert_eq!(streamed, bytes);
    let (headers, _) = worker.join().unwrap();
    assert!(headers.starts_with("GET /v1/files/streamed/content HTTP/1.1"));
}
'''
    if text.count(marker) != 1:
        raise RuntimeError("transport binary test marker drift")
    path.write_text(text.replace(marker, addition, 1))


def main() -> None:
    update_taxonomy_harvester()
    update_taxonomy_compactor()
    update_autoprojector()
    update_facade_compiler()
    update_tests()
    update_generator_patch()
    update_transport_test()


if __name__ == "__main__":
    main()
