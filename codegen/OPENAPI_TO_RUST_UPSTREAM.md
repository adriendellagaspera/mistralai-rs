# `openapi-to-rust` upstream issue inventory

```yaml
generator:
  crate: openapi-to-rust
  version: 0.16.0
  commit: 2af34b86ca9f38c35787f13ec5841989efcf4b99
  patch_sha256: 5047f7877c2e62b45f4fefb7e0ecd35929368d6e00051f460a87d9c77d631a41
consumer:
  repository: adriendellagaspera/mistralai-rs
  openapi_repository: mistralai/platform-docs-public
  openapi_commit: ff846cf93d91df6fe04d0a5e540ecbe1e0ecc691
  openapi_version: 3.1.0
  rust: 1.94.0
```

## 1. Multipart arrays generate unusable client methods

```yaml
title: Support arrays of scalar values in multipart/form-data
priority: critical
component: src/client_generator.rs
```

### Input

```yaml
type: object
properties:
  context_bias:
    type: array
    items:
      type: string
  timestamp_granularities:
    type: array
    items:
      $ref: '#/components/schemas/TimestampGranularity'
```

### Actual

```text
Generated client method returns a configuration error.
HTTP request construction becomes unreachable.
```

### Expected wire format

```http
Content-Disposition: form-data; name="context_bias"

Mistral
Content-Disposition: form-data; name="context_bias"

Voxtral
```

### Expected generation

```rust
for item in value {
    form = form.text("context_bias", item.to_string());
}
```

### Acceptance criteria

```yaml
- optional arrays
- required arrays
- arrays of strings
- arrays of string enums
- empty arrays
- repeated multipart fields
- no comma-joined encoding
- no generated configuration stub
```

## 2. Nullable binary multipart schemas are not resolved

```yaml
title: Resolve nullable binary schemas in multipart/form-data
priority: critical
component: src/client_generator.rs
```

### Input

```yaml
File:
  type: string
  format: binary
AudioTranscriptionRequest:
  type: object
  properties:
    file:
      anyOf:
        - $ref: '#/components/schemas/File'
        - type: 'null'
```

### Actual

```text
anyOf[$ref(binary), null] is not classified as a binary multipart field.
```

### Expected resolution

```text
anyOf[$ref(File), null]
→ $ref(File)
→ type: string, format: binary
→ bytes::Bytes
```

### Expected Rust type

```rust
pub file: Option<Option<bytes::Bytes>>
```

### Acceptance criteria

```yaml
- inline binary schema
- binary schema through $ref
- anyOf[binary, null]
- anyOf[$ref(binary), null]
- absent field omitted
- explicit null omitted
- present bytes preserved byte-for-byte
```

## 3. Multipart binary parts lack filenames

```yaml
title: Preserve or configure filenames for multipart binary parts
priority: high
component: src/client_generator.rs
```

### Actual

```rust
reqwest::multipart::Part::bytes(value.to_vec())
```

### Required API shape

```rust
pub struct Upload {
    pub bytes: bytes::Bytes,
    pub filename: Option<String>,
    pub content_type: Option<mime::Mime>,
}
```

### Expected wire format

```http
Content-Disposition: form-data; name="file"; filename="sample.wav"
```

### Acceptance criteria

```yaml
- filename can be supplied per file
- concurrent uploads can use different filenames
- filename is escaped by reqwest
- binary content is not converted to text or base64
- optional Content-Type support
- backward-compatible Bytes conversion
```

## 4. Generated streaming method does not enforce request stream mode

```yaml
title: Align request stream discriminator with selected response media type
priority: critical
component: src/client_generator.rs
```

### Input

```yaml
requestBody:
  content:
    application/json:
      schema:
        type: object
        properties:
          stream:
            type: boolean
            default: false
responses:
  '200':
    content:
      application/json: {}
      text/event-stream: {}
```

### Actual

```text
The generated SSE method may send stream=false or omit stream.
The server can return JSON while the generated method expects SSE.
```

### Required mappings

```text
bool                         → true / false
Option<bool>                 → Some(true) / Some(false)
Option<Option<bool>>         → Some(Some(true)) / Some(Some(false))
```

### Selection rule

```text
selected success media type == text/event-stream
    => stream = true
selected success media type != text/event-stream
    => stream = false
```

### Acceptance criteria

```yaml
- required boolean
- optional boolean
- nullable boolean
- optional and nullable boolean
- referenced request schemas
- JSON request bodies
- multipart request bodies
- generated JSON and SSE alternatives cannot disagree with payload
```

## 5. Returned streams capture the client lifetime

```yaml
title: Return owned 'static streams for event-stream responses
priority: critical
component: src/client_generator.rs
rust_edition: 2024
```

### Actual signature

```rust
async fn stream(
    &self,
    request: Request,
) -> Result<
    impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>>,
    Error,
>
```

### Failing usage

```rust
let stream = {
    let client = Client::new();
    client.stream(request).await?
};
consume(stream).await;
```

### Expected signature

```rust
async fn stream(
    &self,
    request: Request,
) -> Result<
    futures_util::stream::BoxStream<
        'static,
        Result<bytes::Bytes, reqwest::Error>,
    >,
    Error,
>
```

### Expected implementation

```rust
Ok(Box::pin(response.bytes_stream()))
```

### Acceptable alternative

```rust
-> Result<
    impl Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
    Error,
>
```

### Acceptance criteria

```yaml
- stream does not borrow HttpClient
- stream does not borrow request arguments
- stream is Send
- client can be dropped after await
- dropping stream cancels response consumption
- no response buffering
```

## 6. Normalized enum variant collisions produce non-idiomatic identifiers

```yaml
title: Generate valid CamelCase suffixes for colliding enum variants
priority: medium
component: src/client_generator.rs
```

### Reproduction

```yaml
type: string
enum:
  - created
  - Created
```

### Actual

```rust
enum Status {
    Created,
    Created_2,
}
```

### Expected

```rust
enum Status {
    Created,
    Created2,
}
```

### Acceptance criteria

```yaml
- generated variants satisfy non_camel_case_types
- serde rename preserves the original wire value
- deterministic numbering
- collisions after case normalization
- collisions after punctuation removal
- three or more colliding variants
```

## 7. Multiline descriptions generate unstable doc attributes

```yaml
title: Emit one Rust doc attribute per OpenAPI description line
priority: medium
component: src/generator.rs
```

### Input

```yaml
description: |
  First paragraph.

  Second paragraph.
```

### Actual

```rust
#[doc = "First paragraph.\n\nSecond paragraph."]
```

### Expected

```rust
#[doc = "First paragraph."]
#[doc = ""]
#[doc = "Second paragraph."]
```

### Acceptance criteria

```yaml
- multiline descriptions
- blank lines
- fenced code blocks
- indentation
- quotes and backslashes
- cargo doc succeeds
- rustfmt is stable
- no clippy::empty_docs regression
```

## 8. Generated code fails strict Clippy

```yaml
title: Make generated Rust pass clippy -D warnings
priority: medium
scope: generated output
```

### Diagnostics observed

```text
clippy::double_must_use
clippy::redundant_field_names
clippy::nonminimal_bool
clippy::too_many_arguments
clippy::match_single_binding
clippy::collapsible_if
clippy::empty_docs
```

### Required command

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Acceptance criteria

```yaml
double_must_use:
  - do not annotate both builder method and returned builder type
redundant_field_names:
  - emit field shorthand when source and destination identifiers match
nonminimal_bool:
  - simplify generated boolean expressions
too_many_arguments:
  - generate request structs or scope the allow attribute to generated methods
match_single_binding:
  - emit the inner expression directly
collapsible_if:
  - combine nested generated conditions
empty_docs:
  - preserve meaningful blank lines without producing empty documented items
lint_policy:
  - no crate-wide allow(warnings)
  - generated-only allow attributes accepted when structural elimination is impossible
```

## 9. Binary success bodies need a streaming client variant

```yaml
title: Generate an owned streaming variant for binary response bodies
priority: critical
component: src/client_generator.rs
```

### Actual

```text
Binary success responses are buffered into bytes::Bytes and therefore inherit the global in-memory response limit.
```

### Required API shape

```rust
async fn download_stream(...) -> Result<
    futures_util::stream::BoxStream<'static, Result<bytes::Bytes, reqwest::Error>>,
    ApiOpError<_>,
>
```

The existing buffered method remains available for compatibility. The streaming variant must preserve the same path, parameters, authentication, Accept header, status selection, and bounded error-body handling.

### Acceptance criteria

```yaml
- existing buffered binary method remains unchanged
- companion streaming method is additive
- success body is never buffered
- returned stream is owned, Send and static
- dropping the stream cancels response consumption
- non-success responses remain bounded and inspectable
- content negotiation matches the binary media type
```
