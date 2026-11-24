# Fern Rust v1 compatibility review

The current production inventory declares 347 HTTP operations and 349 public
method paths. The two additional method paths are the streaming aliases
`chat.complete_stream` and `fim.complete_stream`, both backed by conditional
SSE responses on the same POST operation as their non-streaming counterpart.

Fern Rust generator 1.0.0 emits the 347 HTTP operations but omits those two
conditional-SSE aliases. The deterministic patch in
`patch_generated_streaming.py` restores them. It clones the generated request
model, forces `stream = true`, preserves per-call query/header overrides and
authentication, rejects non-SSE success responses, and parses
`CompletionChunk` values with Fern's SSE parser.

Public API changes to account for during cutover:
- The generated resource clients are flattened into top-level modules rather
  than retaining all legacy nested resource group paths.
- `complete_stream` takes the shared `ChatCompletionRequest` or
  `FimCompletionRequest`, with `stream` optional in the model, rather than
  legacy stream-specific request structs.
- The streaming return type is a boxed `futures::Stream`, not the legacy
  `SseStream<T>`. This changes timeout/metadata behavior and must be covered
  in release notes and downstream integration tests.
- The crate's generated API surface uses Fern-native client/config types and
  module exports; compatibility with the prior handwritten facade is not
  implied.

The current CI candidate validates generation and compilation, not behavioral
parity with the legacy transport implementation. Before cutover, add request
serialization and SSE parsing contract tests, and review downstream examples
and integration tests against the new module/type names.
