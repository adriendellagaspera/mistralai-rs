from __future__ import annotations

import hashlib
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
PIN = "2af34b86ca9f38c35787f13ec5841989efcf4b99"


def replace_once(path: Path | str, old: str, new: str) -> None:
    target = Path(path)
    source = target.read_text()
    count = source.count(old)
    if count != 1:
        raise SystemExit(f"expected one match in {target}, got {count}: {old[:100]!r}")
    target.write_text(source.replace(old, new, 1))


def replace_between(path: Path | str, start: str, end: str, replacement: str) -> None:
    target = Path(path)
    source = target.read_text()
    start_index = source.find(start)
    if start_index < 0:
        raise SystemExit(f"start marker not found in {target}: {start!r}")
    end_index = source.find(end, start_index)
    if end_index < 0:
        raise SystemExit(f"end marker not found in {target}: {end!r}")
    target.write_text(source[:start_index] + replacement + source[end_index:])


# Extend the pinned raw generator additively: keep buffered binary methods for
# compatibility and generate an owned streaming companion for binary successes.
with tempfile.TemporaryDirectory(prefix="binary-generator-") as directory:
    upstream = Path(directory) / "openapi-to-rust"
    subprocess.run(
        ["git", "clone", "--quiet", "https://github.com/gpu-cli/openapi-to-rust.git", str(upstream)],
        check=True,
    )
    subprocess.run(["git", "checkout", "--quiet", PIN], cwd=upstream, check=True)
    subprocess.run(
        ["git", "apply", str(ROOT / "codegen/patches/generator.patch")],
        cwd=upstream,
        check=True,
    )
    generator = upstream / "src/client_generator.rs"
    replace_once(
        generator,
        '''        let methods: Vec<TokenStream> = operations
            .iter()
            .copied()
            .map(|op| self.generate_single_operation_method(analysis, op))
            .collect();

        let (operation_builders, builder_entries) =
''',
        '''        let methods: Vec<TokenStream> = operations
            .iter()
            .copied()
            .map(|op| self.generate_single_operation_method(analysis, op))
            .collect();
        let binary_stream_methods: Vec<TokenStream> = operations
            .iter()
            .copied()
            .filter_map(|op| self.generate_binary_stream_method(analysis, op))
            .collect();

        let (operation_builders, builder_entries) =
''',
    )
    replace_once(
        generator,
        '''            impl HttpClient {
                #(#methods)*
                #(#builder_entries)*
            }
''',
        '''            impl HttpClient {
                #(#methods)*
                #(#binary_stream_methods)*
                #(#builder_entries)*
            }
''',
    )
    method_block = r'''    fn generate_single_operation_method(
        &self,
        analysis: &SchemaAnalysis,
        op: &OperationInfo,
    ) -> TokenStream {
        let method_name = self.get_method_name(op);
        let success = self.get_success_response(analysis, op);
        self.generate_operation_method_with_success(analysis, op, method_name, success)
    }

    /// Add an owned streaming companion for binary success bodies while
    /// retaining the existing buffered method for backwards compatibility.
    fn generate_binary_stream_method(
        &self,
        analysis: &SchemaAnalysis,
        op: &OperationInfo,
    ) -> Option<TokenStream> {
        let success = self.get_success_response(analysis, op);
        if !matches!(success.body, ClientSuccessBody::Binary) {
            return None;
        }
        let method_name = format_ident!("{}_stream", self.get_method_name(op));
        let streaming_success = ClientSuccessSelection {
            statuses: success.statuses.clone(),
            body: ClientSuccessBody::EventStream,
            accept: success.accept,
        };
        Some(self.generate_operation_method_with_success(
            analysis,
            op,
            method_name,
            streaming_success,
        ))
    }

    fn generate_operation_method_with_success(
        &self,
        analysis: &SchemaAnalysis,
        op: &OperationInfo,
        method_name: syn::Ident,
        success: ClientSuccessSelection<'_>,
    ) -> TokenStream {
        let http_method_call = self.http_method_call(op);
        let path = &op.path;
        let request_param = self.generate_request_param(op);
        let request_body = self.generate_request_body(op, analysis);
        let stream_parameter = self.generate_stream_parameter(op, analysis);
        let query_params = self.generate_query_params(op);
        let header_params = self.generate_header_params(op);
        let cookie_params = self.generate_cookie_params(op);
        let auth_application = self.generate_auth_application();
        let response_type = self.response_type_for_body(success.body);
        let op_error_type = self.op_error_type_token(op);
        let accept = success.accept;
        let error_handling = self.generate_error_handling(op, success);
        let (custom_headers, accept_header) = if let Some(media_type) = accept {
            (
                quote! {
                    for (name, value) in &self.custom_headers {
                        if !name.eq_ignore_ascii_case("accept") {
                            req = req.header(name, value);
                        }
                    }
                },
                quote! {
                    req = req.header(reqwest::header::ACCEPT, #media_type);
                },
            )
        } else {
            (
                quote! {
                    for (name, value) in &self.custom_headers {
                        req = req.header(name, value);
                    }
                },
                TokenStream::new(),
            )
        };
        let url_construction = self.generate_url_construction(path, op);
        let doc_comment = self.generate_operation_doc_comment(op);

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

                #auth_application
                #custom_headers
                #accept_header

                let response = req.send().await?;
                #error_handling
            }
        }
    }

'''
    replace_between(
        generator,
        "    fn generate_single_operation_method(\n",
        "    // Keep a declared boolean stream parameter aligned",
        method_block,
    )
    response_type_block = r'''    /// Get response type
    fn get_response_type(&self, analysis: &SchemaAnalysis, op: &OperationInfo) -> TokenStream {
        self.response_type_for_body(self.get_success_response(analysis, op).body)
    }

    fn response_type_for_body(&self, body: ClientSuccessBody<'_>) -> TokenStream {
        match body {
            ClientSuccessBody::Json(response_type) => {
                let rust_type_name = self.to_rust_type_name(response_type);
                let response_ident =
                    syn::Ident::new(&rust_type_name, proc_macro2::Span::call_site());
                quote! { #response_ident }
            }
            ClientSuccessBody::Text => quote! { String },
            ClientSuccessBody::Binary => quote! { bytes::Bytes },
            ClientSuccessBody::EventStream => {
                quote! { futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>> }
            }
            ClientSuccessBody::Empty => quote! { () },
        }
    }

'''
    replace_between(
        generator,
        "    /// Get response type\n",
        "    fn success_status_guard",
        response_type_block,
    )
    subprocess.run(["rustfmt", str(generator)], cwd=upstream, check=True)
    patch = subprocess.run(
        ["git", "diff", "--", "src/client_generator.rs", "src/generator.rs"],
        cwd=upstream,
        check=True,
        capture_output=True,
        text=True,
    ).stdout
    if not patch.strip():
        raise SystemExit("generator patch unexpectedly empty")
    (ROOT / "codegen/patches/generator.patch").write_text(patch)

patch_path = ROOT / "codegen/patches/generator.patch"
patch_sha = hashlib.sha256(patch_path.read_bytes()).hexdigest()
upstream_doc = ROOT / "codegen/OPENAPI_TO_RUST_UPSTREAM.md"
doc = upstream_doc.read_text()
doc = __import__("re").sub(r"patch_sha256: [0-9a-f]{64}", f"patch_sha256: {patch_sha}", doc, count=1)
if "## 9. Binary responses only expose a buffered client method" not in doc:
    doc += r'''

## 9. Binary responses only expose a buffered client method

```yaml
title: Generate an owned streaming companion for binary response bodies
priority: high
component: src/client_generator.rs
```

### Actual

```rust
pub async fn download(&self, id: impl AsRef<str>)
    -> Result<bytes::Bytes, ApiOpError<_>>
```

Successful binary responses are always read through the configured in-memory
response limit before the caller can consume a byte. This is appropriate for
small payloads but not for file-download APIs.

### Expected additive API

```rust
pub async fn download(&self, id: impl AsRef<str>)
    -> Result<bytes::Bytes, ApiOpError<_>>;

pub async fn download_stream(&self, id: impl AsRef<str>)
    -> Result<
        futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>,
        ApiOpError<_>,
    >;
```

The existing buffered method remains unchanged. The streaming companion must
reuse the same path/query/header/auth/content-negotiation logic, hand successful
response chunks to the caller without buffering, and keep bounded buffering for
finite error responses.

### Acceptance criteria

```yaml
- existing buffered binary method remains source-compatible
- streaming companion is additive
- returned stream is Send + 'static
- successful response body is never buffered eagerly
- non-2xx response body remains bounded and inspectable
- Accept header and all request bindings match the buffered method
- dropping the returned stream cancels further response consumption
- deterministic companion naming and collision handling
```
'''
upstream_doc.write_text(doc)

# Semantic compiler contract: binary is a distinct success mode from SSE.
replace_once(
    ROOT / "codegen/sdk-semantics.schema.json",
    '      "request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "stream": {"$ref": "#/$defs/stream"},\n',
    '      "request": {"$ref": "#/$defs/identifier"}, "response": {"$ref": "#/$defs/identifier"}, "empty_response": {"const": true}, "binary_stream": {"const": true}, "stream": {"$ref": "#/$defs/stream"},\n',
)

compiler = ROOT / "codegen/sdk_codegen.py"
replace_once(
    compiler,
    '    empty_response: bool = False\n',
    '    empty_response: bool = False\n    binary_stream: bool = False\n',
)
replace_once(
    compiler,
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
    '_validate_keys(item, {"operation_id", "raw_method", "request", "response", "empty_response", "binary_stream", "request_overrides", "stream"}, f"operation {module}.{public_name}")',
)
replace_once(
    compiler,
    '''            empty_response = item.get("empty_response", False)\n            success_modes = int(bool(response)) + int(bool(item.get("stream"))) + int(empty_response)\n            if success_modes != 1:\n                raise GenerationError(\n                    f"operation {module}.{public_name} requires exactly one response, stream or empty_response projection"\n                )\n''',
    '''            empty_response = item.get("empty_response", False)\n            binary_stream = item.get("binary_stream", False)\n            success_modes = (int(bool(response)) + int(bool(item.get("stream")))\n                             + int(empty_response) + int(binary_stream))\n            if success_modes != 1:\n                raise GenerationError(\n                    f"operation {module}.{public_name} requires exactly one response, stream, empty_response or binary_stream projection"\n                )\n''',
)
replace_once(
    compiler,
    '''            if empty_response:\n                success = [value for status, value in wire_operation.get("responses", {}).items()\n                           if str(status).startswith("2")]\n                if len(success) != 1 or success[0].get("content"):\n                    raise GenerationError(f"empty response drift for {operation_id}")\n                if raw_operation.success_type != "()":\n                    raise GenerationError(f"raw empty response drift for {raw_method}")\n            if response and not item.get("stream"):\n''',
    '''            if empty_response:\n                success = [value for status, value in wire_operation.get("responses", {}).items()\n                           if str(status).startswith("2")]\n                if len(success) != 1 or success[0].get("content"):\n                    raise GenerationError(f"empty response drift for {operation_id}")\n                if raw_operation.success_type != "()":\n                    raise GenerationError(f"raw empty response drift for {raw_method}")\n            if binary_stream:\n                success = [value for status, value in wire_operation.get("responses", {}).items()\n                           if str(status).startswith("2")]\n                content = success[0].get("content", {}) if len(success) == 1 else {}\n                schemas = [value.get("schema", {}) for value in content.values()]\n                if (len(content) != 1 or len(schemas) != 1\n                        or schemas[0].get("type") != "string"\n                        or schemas[0].get("format") != "binary"):\n                    raise GenerationError(f"binary response drift for {operation_id}")\n                if raw_operation.success_type != "bytes::Bytes":\n                    raise GenerationError(f"raw binary response drift for {raw_method}")\n                companion_name = raw_method + "_stream"\n                try:\n                    companion = rust.operation(companion_name)\n                except GenerationError as error:\n                    raise GenerationError(f"binary stream transport missing for {raw_method}") from error\n                primary_signature = tuple((p.name, p.type) for p in raw_operation.parameters)\n                companion_signature = tuple((p.name, p.type) for p in companion.parameters)\n                if companion_signature != primary_signature:\n                    raise GenerationError(f"binary stream parameter drift for {raw_method}")\n                transport = parse_type(companion.success_type)\n                if transport.constructor != "futures_util::stream::BoxStream" or len(transport.arguments) != 2:\n                    raise GenerationError(f"binary stream transport is not an owned BoxStream: {raw_method}")\n                lifetime, event = transport.arguments\n                if (lifetime.spelling != "'static" or event.constructor != "Result"\n                        or tuple(arg.spelling for arg in event.arguments)\n                        != ("bytes::Bytes", "reqwest::Error")):\n                    raise GenerationError(f"binary stream transport ownership/item drift: {raw_method}")\n            if response and not item.get("stream"):\n''',
)
replace_once(
    compiler,
    '''            operations.append(OperationSpec(public_name, operation_id, raw_method, request, response,\n                                            tuple(item.get("request_overrides", {}).items()),\n                                            stream_policy(item.get("stream")), request_raw, empty_response))\n''',
    '''            operations.append(OperationSpec(public_name, operation_id, raw_method, request, response,\n                                            tuple(item.get("request_overrides", {}).items()),\n                                            stream_policy(item.get("stream")), request_raw, empty_response,\n                                            binary_stream))\n''',
)
replace_once(
    compiler,
    '''                if operation.stream:\n                    symbols.claim(operation.stream.type, "sdk", operation.operation_id, "facade_types")\n''',
    '''                if operation.stream:\n                    symbols.claim(operation.stream.type, "sdk", operation.operation_id, "facade_types")\n        if any(operation.binary_stream for resource in ir.resources for operation in resource.operations):\n            symbols.claim("BinaryStream", "sdk", "binary response transport", "facade_types")\n''',
)
replace_once(
    compiler,
    '''    if operation.empty_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<(), SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n    if not operation.response:\n''',
    '''    if operation.binary_stream:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<BinaryStream, SdkError> {{\\n"\n            f"    let chunks = self.raw.{operation.raw_method}_stream({call}).await.map_err(SdkError::from)?;\\n"\n            f"    Ok(Box::pin(chunks.map(|chunk| chunk.map_err(SdkError::from))))\\n}}"\n        )\n    if operation.empty_response:\n        separator = ", " if arguments else ""\n        return (\n            f"pub async fn {operation.name}(&self{separator}{arguments}) -> Result<(), SdkError> {{\\n"\n            f"    self.raw.{operation.raw_method}({call}).await.map_err(Into::into)\\n}}"\n        )\n    if not operation.response:\n''',
)
replace_once(
    compiler,
    '''def _emit_resource(resource: ResourceSpec, rust: RustIndex, resources: tuple[ResourceSpec, ...]) -> str:\n    streaming = any(operation.stream for operation in resource.operations)\n    imports = ("use futures_util::StreamExt;\\nuse crate::streaming;\\n"\n               "use crate::generated::types::*;\\n" if streaming else "")\n''',
    '''def _emit_resource(resource: ResourceSpec, rust: RustIndex, resources: tuple[ResourceSpec, ...]) -> str:\n    sse = any(operation.stream for operation in resource.operations)\n    binary = any(operation.binary_stream for operation in resource.operations)\n    imports = "use futures_util::StreamExt;\\n" if sse or binary else ""\n    if sse:\n        imports += "use crate::streaming;\\nuse crate::generated::types::*;\\n"\n''',
)
replace_once(
    compiler,
    '''    exported_types.extend(operation.stream.type\n                          for resource in ir.resources for operation in resource.operations\n                          if operation.stream)\n    models = ", ".join(exported_types)\n''',
    '''    exported_types.extend(operation.stream.type\n                          for resource in ir.resources for operation in resource.operations\n                          if operation.stream)\n    if any(operation.binary_stream for resource in ir.resources for operation in resource.operations):\n        exported_types.append("BinaryStream")\n    models = ", ".join(exported_types)\n''',
)
replace_once(
    compiler,
    '''    aliases = []\n    for resource in ir.resources:\n        for operation in resource.operations:\n            if operation.stream:\n                stream = operation.stream\n                aliases.append(\n                    f"pub type {stream.type} = Pin<Box<dyn Stream<Item = Result<{stream.wrapper}, SdkError>> + Send + 'static>>;"\n                )\n''',
    '''    aliases = []\n    if any(operation.binary_stream for resource in ir.resources for operation in resource.operations):\n        aliases.append(\n            "pub type BinaryStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, SdkError>> + Send + 'static>>;"\n        )\n    for resource in ir.resources:\n        for operation in resource.operations:\n            if operation.stream:\n                stream = operation.stream\n                aliases.append(\n                    f"pub type {stream.type} = Pin<Box<dyn Stream<Item = Result<{stream.wrapper}, SdkError>> + Send + 'static>>;"\n                )\n''',
)

# Auto-project single-media OpenAPI binary responses, but keep mixed media explicit.
autoproject = ROOT / "codegen/sdk_autoproject.py"
replace_once(
    autoproject,
    '''    if not content:\n        return "empty", None, None\n    if "application/json" not in content:\n        return None, None, "non_json_success"\n    if set(content) != {"application/json"}:\n        return None, None, "multiple_success_media"\n    return "json", content["application/json"].get("schema", {}), None\n''',
    '''    if not content:\n        return "empty", None, None\n    if len(content) == 1:\n        media = next(iter(content.values()))\n        schema = media.get("schema", {})\n        if schema.get("type") == "string" and schema.get("format") == "binary":\n            return "binary", None, None\n    if "application/json" not in content:\n        return None, None, "non_json_success"\n    if set(content) != {"application/json"}:\n        return None, None, "multiple_success_media"\n    return "json", content["application/json"].get("schema", {}), None\n''',
)
replace_once(
    autoproject,
    '''        elif response_kind == "empty":\n            item["empty_response"] = True\n        if raw_method != operation_id:\n''',
    '''        elif response_kind == "empty":\n            item["empty_response"] = True\n        elif response_kind == "binary":\n            item["binary_stream"] = True\n        if raw_method != operation_id:\n''',
)

contracts = ROOT / "codegen/sdk_contracts.py"
replace_once(
    contracts,
    '''        if (any(response.get("content") for response in responses)\n                and not any("application/json" in response.get("content", {}) for response in responses)):\n            reasons.append("non_json_success")\n''',
    '''        def supported_binary(response):\n            content = response.get("content", {})\n            if len(content) != 1:\n                return False\n            schema = next(iter(content.values())).get("schema", {})\n            return schema.get("type") == "string" and schema.get("format") == "binary"\n        if (any(response.get("content") for response in responses)\n                and not any("application/json" in response.get("content", {}) for response in responses)\n                and not all(supported_binary(response) for response in responses)):\n            reasons.append("non_json_success")\n''',
)

# Chunk-level transport errors should map into the stable semantic error type.
error_rs = ROOT / "src/sdk/error.rs"
replace_once(
    error_rs,
    '''impl<E: std::fmt::Debug> From<ApiOpError<E>> for SdkError {\n''',
    '''impl From<reqwest::Error> for SdkError {\n    fn from(error: reqwest::Error) -> Self {\n        Self::Transport(HttpError::Network(error).into())\n    }\n}\n\nimpl<E: std::fmt::Debug> From<ApiOpError<E>> for SdkError {\n''',
)

# Generic compiler/autoproject tests.
auto_test = ROOT / "scripts/test_sdk_autoproject.py"
replace_once(
    auto_test,
    '''    def test_binary_success_stays_review_debt(self):\n        api = FakeOpenApi()\n        api.operations["download_thing"] = {\n            "responses": {"200": {"content": {"application/octet-stream": {"schema": {"type": "string", "format": "binary"}}}}},\n            "parameters": [],\n        }\n        _, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"download_thing": ["things.download"]}},\n            raw_coverage("download_thing"),\n        )\n        self.assertEqual("non_json_success", report["rejected"]["download_thing"])\n''',
    '''    def test_projects_binary_success_as_stream_without_fake_model(self):\n        api = FakeOpenApi()\n        api.operations["download_thing"] = {\n            "responses": {"200": {"content": {"application/octet-stream": {"schema": {"type": "string", "format": "binary"}}}}},\n            "parameters": [],\n        }\n        expanded, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"download_thing": ["things.download"]}},\n            raw_coverage("download_thing"),\n        )\n        self.assertEqual(1, report["added_count"])\n        operation = expanded["resources"]["things"]["operations"]["download"]\n        self.assertTrue(operation["binary_stream"])\n        self.assertNotIn("response", operation)\n\n    def test_mixed_json_and_binary_success_requires_review(self):\n        api = FakeOpenApi()\n        api.operations["download_thing"] = {\n            "responses": {"200": {"content": {\n                "application/json": {"schema": {"$ref": "#/components/schemas/Thing"}},\n                "application/octet-stream": {"schema": {"type": "string", "format": "binary"}},\n            }}},\n            "parameters": [],\n        }\n        _, report = sdk_autoproject.expand_manifest(\n            api, manifest(),\n            {"operations": {"download_thing": ["things.download"]}},\n            raw_coverage("download_thing"),\n        )\n        self.assertEqual("multiple_success_media", report["rejected"]["download_thing"])\n''',
)

facade_test = ROOT / "scripts/test_sdk_facade.py"
marker = '''    def test_inventory_includes_unmapped_operations(self):\n'''
binary_tests = r'''    def test_binary_success_uses_owned_raw_stream_companion(self):
        document = openapi_document()
        document["paths"]["/binary/{animal_id}"] = {
            "get": {
                "operationId": "download_animal",
                "parameters": [{
                    "name": "animal_id", "in": "path", "required": True,
                    "schema": {"type": "string"},
                }],
                "responses": {"200": {"content": {
                    "application/octet-stream": {
                        "schema": {"type": "string", "format": "binary"}
                    }
                }}},
            }
        }
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["download"] = {
            "operation_id": "download_animal", "binary_stream": True,
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT + """
impl HttpClient {
    pub async fn download_animal(&self, animal_id: impl AsRef<str>) -> Result<bytes::Bytes, Error> { todo!() }
    pub async fn download_animal_stream(&self, animal_id: impl AsRef<str>) -> Result<futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>, Error> { todo!() }
}
""")
        target = self.generate()
        resource = (target / "zoo.rs").read_text()
        types = (target / "facade_types.rs").read_text()
        self.assertIn(
            "pub async fn download(&self, animal_id: impl AsRef<str>) -> Result<BinaryStream, SdkError>",
            resource,
        )
        self.assertIn("self.raw.download_animal_stream(animal_id.as_ref())", resource)
        self.assertIn("pub type BinaryStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, SdkError>>", types)

    def test_binary_success_requires_owned_raw_stream_companion(self):
        document = openapi_document()
        document["paths"]["/binary"] = {
            "get": {
                "operationId": "download_animal",
                "responses": {"200": {"content": {
                    "application/octet-stream": {
                        "schema": {"type": "string", "format": "binary"}
                    }
                }}},
            }
        }
        overlay = manifest()
        overlay["resources"]["zoo"]["operations"]["download"] = {
            "operation_id": "download_animal", "binary_stream": True,
        }
        self.openapi.write_text(json.dumps(document))
        self.overlay.write_text(json.dumps(overlay))
        (self.raw / "client.rs").write_text(CLIENT + """
impl HttpClient {
    pub async fn download_animal(&self) -> Result<bytes::Bytes, Error> { todo!() }
}
""")
        with self.assertRaisesRegex(sdk_codegen.GenerationError, "binary stream transport missing"):
            self.generate()

    def test_typed_binary_success_is_a_candidate_not_non_json_gap(self):
        document = openapi_document()
        document["paths"]["/binary"] = {"get": {
            "operationId": "download", "responses": {"200": {"content": {
                "application/octet-stream": {"schema": {"type": "string", "format": "binary"}}
            }}}
        }}
        self.openapi.write_text(json.dumps(document))
        inventory = json.loads((self.generate() / "coverage.json").read_text())["inventory"]
        self.assertEqual(inventory["download"]["status"], "candidate_unverified")
        self.assertNotIn("non_json_success", inventory["download"]["review_reasons"])

'''
replace_once(facade_test, marker, binary_tests + marker)

# Integration/compile evidence: buffered raw remains, raw streaming and facade streaming are additive.
transports = ROOT / "tests/transports.rs"
replace_once(
    transports,
    '''    streaming,\n};\n''',
    '''    streaming, Mistral,\n};\n''',
)
replace_once(
    transports,
    '''    assert!(headers.starts_with("GET /v1/files/a%2Fb%20%3F/content HTTP/1.1"));\n\n    let (url, worker) = server("200 OK", "audio/wav", bytes);\n''',
    '''    assert!(headers.starts_with("GET /v1/files/a%2Fb%20%3F/content HTTP/1.1"));\n\n    let (url, worker) = server("200 OK", "application/octet-stream", bytes);\n    let stream = Client::new()\n        .with_base_url(&url)\n        .files_api_routes_download_file_stream("a/b ?")\n        .await\n        .unwrap();\n    let chunks: Vec<_> = stream.collect().await;\n    let streamed: Vec<u8> = chunks\n        .into_iter()\n        .flat_map(|chunk| chunk.unwrap().to_vec())\n        .collect();\n    assert_eq!(streamed, bytes);\n    let (headers, _) = worker.join().unwrap();\n    assert!(headers.starts_with("GET /v1/files/a%2Fb%20%3F/content HTTP/1.1"));\n    assert!(headers.to_lowercase().contains("accept: application/octet-stream"));\n\n    let (url, worker) = server("200 OK", "application/octet-stream", bytes);\n    let stream = Mistral::new("test")\n        .with_base_url(&url)\n        .files()\n        .download("a/b ?")\n        .await\n        .unwrap();\n    let chunks: Vec<_> = stream.collect().await;\n    let streamed: Vec<u8> = chunks\n        .into_iter()\n        .flat_map(|chunk| chunk.unwrap().to_vec())\n        .collect();\n    assert_eq!(streamed, bytes);\n    let (headers, _) = worker.join().unwrap();\n    assert!(headers.starts_with("GET /v1/files/a%2Fb%20%3F/content HTTP/1.1"));\n\n    let (url, worker) = server("200 OK", "audio/wav", bytes);\n''',
)

facade_rs = ROOT / "tests/facade.rs"
replace_once(
    facade_rs,
    '''    let _delete = files.delete("00000000-0000-0000-0000-000000000000");\n    let _signed_url = files.get_signed_url(\n''',
    '''    let _delete = files.delete("00000000-0000-0000-0000-000000000000");\n    let _download = files.download("00000000-0000-0000-0000-000000000000");\n    let _signed_url = files.get_signed_url(\n''',
)
