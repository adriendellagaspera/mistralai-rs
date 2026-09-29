# Fern production cutover: API compatibility report

This migration intentionally replaces the pre-1.0 handwritten/generated facade with Fern's native Rust SDK surface. Breaking changes are accepted for this cutover and are made explicit here rather than hidden behind a compatibility facade.

## Baseline and new surface

The evidence spike on the same 288-operation source measured the previous facade at 328 public async methods and 695 public facade types. The Fern product-shaped output contained 303 public async methods and 1,826 public types before the additional production naming policy in this cutover; method renaming changes names, not operation coverage. The checked-in `public-inventory.json` is the authoritative machine-readable inventory for the production result.

## Intentional breaking changes

- `Mistral` plus `mistralai::raw` are removed. The Fern-native root client is `mistralai_sdk::ApiClient`, configured with `ClientConfig`.
- The old `src/generated` raw transport and `src/sdk` facade split is removed. Fern-generated modules under `src/api`, `src/client`, `src/core`, and related root modules are the public SDK.
- Fern-native request types and `Option<RequestOptions>` are exposed directly instead of wrapping them behind the former facade.
- High-value resources use reviewed short verbs: `chat.complete`, `chat.complete_stream`, FIM equivalents, `files.*`, `audio.*`, `embeddings.create`, `ocr.process`, and short workflow verbs. Other operations retain stable operation-derived names rather than adding another Rust facade.
- `files.download` and voice sample retrieval are native `ByteStream` responses. This is a streaming binary API rather than the old buffered/facade representation.

## Historical transport overrides reviewed

The production gates explicitly assert all five previously special cases:

1. chat completion has JSON unary plus SSE streaming;
2. FIM completion has JSON unary plus SSE streaming;
3. file download is binary `ByteStream`;
4. speech remains a JSON response containing `audio_data`;
5. voice sample retrieval is binary `ByteStream`.

Audio transcription multipart+SSE uses one narrow generated-output workaround until fern-api/fern#17928 is fixed. All other generic Fern importer workarounds are input-only and linked from `sdk-build/fern/README.md`.
