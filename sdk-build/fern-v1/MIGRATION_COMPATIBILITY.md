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


## Repository-level runtime compatibility gaps

A source-level comparison with the current production runtime found additional
behavior that is not covered by the generated-crate compile gate:

- The production runtime exposes `OAuthTokenProvider` with cached token
  acquisition/refresh and an API-specific token-exchange contract. The v1
  generated `config.rs` contains OAuth configuration fields but explicitly
  omits the provider, and no generated runtime code consumes those fields.
  OAuth client-credentials authentication would therefore be silently
  nonfunctional after a direct source replacement.
- Production exposes `RequestExecutor` injection, custom `reqwest::Client`
  injection, and `RawResponse<T>` (body, status, and headers). The generated
  v1 runtime currently exposes none of these contracts.
- Production's `HttpClient` accepts `ClientConfig` and exposes JSON, SSE,
  multipart-SSE, and binary-stream execution methods. The v1 runtime instead
  has a base-URL/timeout/retry constructor and lower-level reqwest builders.
  Existing `tests/transport_contracts.rs` is consequently not a drop-in
  compile target.
- Production exposes pagination helpers, structured query parsing/building,
  URL helpers, and the `SseStream` wrapper. These modules are absent from the
  generated v1 runtime.
- The public resource topology changes from grouped clients
  (`Mistral.beta.*`, `Mistral.audio.*`, `Mistral.workflows.*`) to a flat
  set of resource clients on `Mistral` (`Mistral.chat`, `Mistral.sessions`,
  `Mistral.workflows`, etc.). Equal HTTP operation counts do not establish
  source compatibility.

These are migration blockers, not cosmetic differences. A production cutover
needs either explicit, tested compatibility adapters/ported runtime
capabilities or a deliberate major-version API break with corresponding
release notes and integration tests. In particular, do not treat the presence
of OAuth configuration fields as evidence that OAuth is implemented.
