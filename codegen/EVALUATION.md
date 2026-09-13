# Generator evaluation — 2026-09-10

Source: `mistralai/platform-docs-public/openapi.yaml` at
`ff846cf93d91df6fe04d0a5e540ecbe1e0ecc691` (OpenAPI 3.1.0;
173 operations and 481 component schemas).
Generator: released `openapi-to-rust 0.16.0`, installed with `--locked`.
Compiler and Clippy: Rust 1.94.0 on Ubuntu 24.04.

## Historical generator trial

Both scopes were generated directly from the unmodified official spec and
compiled against each generated `REQUIRED_DEPS.toml`.
[Evaluation run](https://github.com/adriendellagaspera/mistralai-rs/actions/runs/34536832328).
The evaluation run is deliberately failed because the raw Clippy gates failed;
this is not the final SDK validation result.

| Scope | Generation/check | Raw Clippy (`-D warnings`) | Decision |
| --- | --- | --- | --- |
| All 173 operations | Generates and compiles, with 9 Rust warnings | 874 diagnostics | Defer full API |
| Chat + list/retrieve models | Generates and compiles with no Rust warnings | 44 diagnostics, exactly 3 style-lint categories | Initial SDK scope |

Full generation is not an acceptable functional SDK merely because it compiles:
the audio transcription methods unconditionally return a configuration error
for multipart `context_bias`, leaving unreachable request code. It also has an
invalid-style `Created_2` enum variant and additional lint categories. Supporting
all endpoints would require wider generator work and endpoint-specific tests.

The selected scope's diagnostics are exclusively:

- `clippy::double_must_use`: builder methods and their return type both carry
  `#[must_use]`. Redundant but semantically harmless.
- `clippy::redundant_field_names`: generated initializers spell `field: field`.
- `clippy::nonminimal_bool`: generated equivalent Boolean expressions.

These three style exceptions are explicit attributes on the handwritten
`generated` module declaration in `src/lib.rs`. They do not apply to handwritten
code. No blanket `allow(warnings)` or `allow(clippy::all)` is added, and no Rust
compiler or correctness warning is suppressed by this project. Remove the
exceptions once a pinned upstream generator release emits clean code. The
generator itself also emits its own documented local allow attributes.

The current full-coverage branch retains the unmodified official specification
as its wire-contract source, then applies versioned, deterministic generator
patches before generation. These patches fix generator behavior (including
multipart requests and owned streaming return types); `codegen.lock` pins and
verifies their hashes. Generated output remains reproducible and is checked
byte-for-byte by CI.

A handwritten semantic facade now covers Chat and OCR. It is the primary public
API for those resources and translates directly to generated request/response
types without JSON round-trips. The complete mechanical transport remains
available explicitly through `raw`. This is intentionally an architectural
spike, not a claim that all 173 operations already have an idiomatic facade.

One preprocessing correction removes `data` from
`ChatCompletionResponse.required`: upstream lists it as required without
defining the property, and normal Mistral chat responses do not contain it.
`codegen/preprocess.py` matches the complete known schema fragment exactly and
fails closed if upstream changes it. The vendored source remains byte-identical
to upstream. The SDK exposes the spec's required `UsageInfo` counters directly
as `i64`.

## Existing clients considered

Documentation inspected on 2026-09-10:

| Project | Relevant observation |
| --- | --- |
| [Govcraft/mistral-rouille](https://github.com/Govcraft/mistral-rouille) | Its README identifies OpenAPI Generator 7.6.0, API 0.0.2 and a 2024 generation timestamp with custom templates. Useful precedent; not the current-spec pipeline sought here. |
| [ivangabriele/mistralai-client-rs](https://github.com/ivangabriele/mistralai-client-rs) | Provides sync/async chat, streaming and tools; README does not describe pinned, deterministic upstream regeneration. |
| [dcastelltort/mistral-ai-sdk](https://github.com/dcastelltort/mistral-ai-sdk) | Broad handwritten SDK and tests; documented architecture is organized around maintained API modules, not committed deterministic generation. |

None of these inspected READMEs establishes the requested maintenance pipeline.
This is not a claim that every implementation detail of those repositories was
audited. The new SDK deliberately emphasizes reproducibility and automated
upstream tracking, not broader endpoint coverage.

`openapi-to-rust` is retained because the actual selected-scope trial succeeds
without schema conversion, and its OpenAPI 3.1 support and selected-operation
model pruning fit the task. A second generator trial is unnecessary given this
result. The existing OpenAPI Generator client illustrates why conversion and
large remapping sets would be an unattractive starting point.

## Package name

The [crates.io index entry for mistralai](https://github.com/rust-lang/crates.io-index/blob/master/mi/st/mistralai)
contains an existing 0.1.0 publication from December 2023. No `mistralai-sdk`
entry was present when checked. Choose package `mistralai-sdk`, library
`mistralai`; availability must be checked again before a future publication.
