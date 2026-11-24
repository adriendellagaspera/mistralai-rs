# Fern production cutover: API compatibility

This pre-1.0 migration intentionally replaces the former raw-bindings plus generated-facade API with Fern's native Rust SDK surface. Breaking changes are reviewed here rather than hidden behind compatibility shims.

## Intentional public changes

- The former handwritten `Mistral` facade and `mistralai::raw` namespace are removed.
- The crate's Fern-native library import is `mistralai_sdk`; the generated root client is `Mistral`, configured with `ClientConfig`.
- The old `src/generated` / `src/sdk` split is removed. Fern modules under `src/api`, `src/client`, `src/core` and related root modules are the published SDK.
- Fern-native request types and `Option<RequestOptions>` are exposed directly instead of being wrapped by the old facade.
- Reviewed high-value operations keep product-oriented resource/method names from `policy.yaml`; other operations use deterministic operation-derived names.
- File download and voice-sample retrieval use Fern's native `ByteStream` representation rather than the old buffered facade representation.

## Transport contracts retained

The cutover keeps explicit policy for chat JSON/SSE, FIM JSON/SSE, binary file download, speech JSON `audio_data`, and binary voice samples. Multipart+SSE audio transcription uses the narrow temporary patch tracked by fern-api/fern#17928.

The machine-readable `inventory.json` is authoritative for 347/347 operation accounting, public surface identity, closure, provenance and collision checks.


## Fern Rust v1 generator findings

The v1.0.0 generator emits one method for each OpenAPI operation but currently
does not materialize the two conditional-SSE aliases declared in the policy:
`chat.complete_stream` and `fim.complete_stream`. The reproducible
`sdk-build/fern-v1/patch_generated_streaming.py` step restores those methods
on the generated clients. It forces `stream: true`, preserves per-call
headers/query overrides and authentication, validates the SSE content type,
and parses `CompletionChunk` values through Fern's generated SSE parser.

This is a generator compatibility workaround, not an OpenAPI operation: the
underlying POST operations are already counted in the 347-operation inventory.
The v1 streaming return type is a boxed `futures::Stream`, rather than the
legacy `SseStream<T>`; the request model is the shared
`ChatCompletionRequest` / `FimCompletionRequest` with an optional `stream`
field rather than the legacy stream-specific request structs. These are
observable public API changes and must be called out in release notes.
