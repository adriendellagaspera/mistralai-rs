# Fern/Mistral 288 spike findings

Status: evidence captured on 2026-09-29 against Fern CLI 5.112.0 and `fernapi/fern-rust-sdk` 0.48.0.

## Current evidence

The canonical 288-operation Mistral OpenAPI source generates successfully with Fern after source-level compatibility normalization. The generated crate passes `cargo fmt --check`, `cargo check`, and Clippy correctness/suspicious gates after one isolated validation shim for multipart + SSE.

Raw Fern surface metrics:

- 288 source operations
- 301 public async methods
- 1,803 public types
- public method-name length: median 24, p95 51, max 72
- public type-name length: median 25, p95 45, max 98
- 297/301 methods expose `Option<RequestOptions>`
- 128 methods remain visibly operation-ID-shaped
- 283 non-correctness Clippy warnings

Current `mistralai-rs` facade metrics:

- 328 public async methods
- 695 public facade types
- public method-name length: median 15, p95 39, max 53
- public type-name length: median 40, p95 70, max 114
- 0 methods expose `RequestOptions`
- 4 operation-ID-shaped method names
- current checked-in facade still contains raw escape hatches; this is not the desired long-term closure target.

## Product-shape proof

Fern OpenAPI extensions can directly own public SDK naming and hierarchy:

- `x-fern-sdk-group-name` can create nested resources such as `beta.observability.*`, `audio.transcriptions`, and `workflows.executions`.
- `x-fern-sdk-method-name` produces product verbs such as `chat.complete`, `files.list`, `files.download`, `embeddings.create`, and `ocr.process`.
- `x-fern-streaming` splits chat/FIM into unary and SSE methods with request stream literals pinned appropriately.

Fern currently ignores `x-fern-sdk-group-name` unless `x-fern-sdk-method-name` is also present. This forces an explicit method-name override for every operation when only hierarchy is desired and should be treated as a Fern importer/configuration gap.

## Historical Mistral overrides

| Current override | Fern representation | Owner |
| --- | --- | --- |
| chat completion JSON + SSE aliases | native conditional streaming via `x-fern-streaming` | Fern mechanism + SDK naming policy |
| FIM JSON + SSE | native conditional streaming via `x-fern-streaming` | Fern mechanism + SDK naming policy |
| files download buffered bytes | native `ByteStream`; buffering is `.into_bytes()` / consumer policy | SDK/consumer policy |
| speech JSON | native JSON `SpeechV1AudioSpeechPostResponse { audio_data }` | Fern mechanism already sufficient |
| voice sample audio | native `ByteStream` | Fern mechanism already sufficient |

## Gap ownership

### 1. Fern mechanism

- allOf property narrowing in the conversation request family is rejected as a duplicate property;
- leading-hyphen enum default `-created` is rejected;
- inline generated type `Judge.output` collided with top-level `JudgeOutput` with no collision rejection;
- multipart + SSE chooses `execute_multipart_request<T: DeserializeOwned>`; Rust generator lacks a multipart SSE executor;
- `x-fern-sdk-group-name` is ignored unless `x-fern-sdk-method-name` is also set;
- generated Rust has substantial non-correctness Clippy noise;
- per-call `Option<RequestOptions>` is mandatory on almost every method and is currently not configurable.

These belong upstream in Fern, not in a replacement generic OpenAPI/HTTP implementation in `rust-sdk-generator`.

### 2. SDK surface policy

- stable resource hierarchy independent of source tag churn;
- stable product method names such as `complete`, `create`, `process`, `list`, `download`;
- reviewed aliases/conveniences such as `parse`, parameterless `list`, and buffered download semantics where desired;
- reviewed type-name overrides and collision-safe public identity.

This can be represented as a thin backend-independent manifest/overlay compiler. It does not require emitting Rust.

### 3. Verification / publication proof

Retain:

- exhaustive 288-operation accounting;
- source operation/schema semantic identity;
- generated public-surface inventory;
- public closure checks;
- collision detection before publication;
- provenance from OpenAPI identity to public SDK path;
- deterministic regeneration;
- semver/public API review gates;
- bounded reviewed overrides/exclusions.

This is the strongest differentiated role for `rust-sdk-generator`.

### 4. Consumer-specific behavior

Keep in `mistralai-rs` where useful:

- convenience aliases not corresponding to source operations;
- buffer-vs-stream convenience helpers;
- other Mistral-only ergonomics that do not generalize to SDK contract compilation.

## Architectural direction supported so far

Evidence supports **Outcome A**. The explicit deterministic-regeneration gate passes: two consecutive Fern generations are byte-for-byte identical for all generated files outside `.fern` metadata.

> Fern becomes the primary Rust generator; `rust-sdk-generator` contracts to a backend-independent SDK surface-policy and verification/compiler layer.

This is the final architecture decision for the spike. Do not port Fern mechanism gaps into `rust-sdk-generator`.
