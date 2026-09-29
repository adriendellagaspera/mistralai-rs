# Fern production cutover: API compatibility

This pre-1.0 migration intentionally replaces the former raw-bindings plus generated-facade API with Fern's native Rust SDK surface. Breaking changes are reviewed here rather than hidden behind compatibility shims.

## Intentional public changes

- The former `Mistral` facade and `mistralai::raw` namespace are removed.
- The crate's Fern-native library import is `mistralai_sdk`; the root client is `ApiClient`, configured with `ClientConfig`.
- The old `src/generated` / `src/sdk` split is removed. Fern modules under `src/api`, `src/client`, `src/core` and related root modules are the published SDK.
- Fern-native request types and `Option<RequestOptions>` are exposed directly instead of being wrapped by the old facade.
- Reviewed high-value operations keep product-oriented resource/method names from `policy.yaml`; other operations use deterministic operation-derived names.
- File download and voice-sample retrieval use Fern's native `ByteStream` representation rather than the old buffered facade representation.

## Transport contracts retained

The cutover keeps explicit policy for chat JSON/SSE, FIM JSON/SSE, binary file download, speech JSON `audio_data`, and binary voice samples. Multipart+SSE audio transcription uses the narrow temporary patch tracked by fern-api/fern#17928.

The machine-readable `inventory.json` is authoritative for 288/288 operation accounting, public surface identity, closure, provenance and collision checks.
