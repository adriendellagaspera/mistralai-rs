# SDK build ownership

`sdk-build/` owns Mistral-specific pinned inputs, orchestration, operation
coverage, publication and public API review. It does **not** contain generic
OpenAPI rewriting or SDK derivation logic. The private `mistralai-sdk-build`
executable lives in `src/main.rs` with Mistral-specific gates in `src/gates.rs`.
Its separate Cargo workspace has `publish = false` and a separate
`Cargo.lock`; it is neither linked by nor added to the public SDK crate.

## Commands and consumers

From the repository root:

| Command | Responsibility | Consumer |
| --- | --- | --- |
| `cargo run --locked --manifest-path sdk-build/Cargo.toml -- raw` | Verify pinned source/tool inputs, regenerate raw bindings in an isolated workspace, publish raw outputs for an explicit source update. | `just sync-sdk-surface`, scheduled source update, CI dry run. |
| `cargo run --locked --manifest-path sdk-build/Cargo.toml -- generate` | Derive all operations, verify baseline and public facade, safely publish compatible outputs. | `just generate`, scheduled source update, CI dry run. |
| `cargo run --locked --manifest-path sdk-build/Cargo.toml -- check` | Verify deterministic generation and exact public facade without modifying the checkout. | `just check-generated`, `just validate`, CI. |
| `cargo run --locked --manifest-path sdk-build/Cargo.toml -- probe` | Diagnostic canonical derivation without publication; optional `--require-parity` and `--compatibility-definition PATH`. | Explicit developer diagnostics. |
| `cargo test --locked --manifest-path sdk-build/Cargo.toml --all-targets` | Regression tests for source-operation identities, closed-world coverage and byte-level facade parity. | `just test-tooling`, `just validate`, CI. |

The separate `openapi/update.py` and `official-sdks/update.py` commands
are **source discovery and pin updates**, never implicit prerequisites that
refresh sources during `check` or `generate`.

## Ownership of remaining inputs and scripts

| Path | Owner and contract |
| --- | --- |
| `provenance.lock.json` | Mistral source SHA-256, Rust toolchain and immutable tool commits/trees; consumed by the builder, collectors, API semver gate and CI cache. |
| `openapi-to-rust.toml`, `openapi/overlays/` | Reviewed Mistral configuration and overlay; applied by pinned `openapi-to-rust`. |
| `openapi/published.yaml`, `openapi/LICENSE` | Immutable published snapshot and source attribution. |
| `openapi/check_published.py` | Narrow Mistral source SHA/dialect/overlay-assumption gate invoked by the Rust builder; no general orchestration. |
| `openapi/update.py` | Explicit upstream OpenAPI discovery and source-pin updates. |
| `official-sdks/harvest.py`, `official-sdks/update.py` | Pinned official Python/TypeScript SDK source parsing, public-path evidence and optional upstream discovery. These remain Python because parsing two other source languages in Rust would add maintenance without improving the build composition root. |
| `official-sdks/surface.json` | Reviewed public-path evidence consumed by the generic SDK compiler. |
| `sdk-overrides.json`, `compatibility-definition.json` | Reviewed Mistral-specific overrides and frozen public API definition, passed unchanged to the generic SDK compiler. |
| `coverage-baseline.json`, `src/gates.rs` | Mistral-specific 173-operation closed-world gate, raw-file/identity proof and exact public facade check. |
| `check_api_compatibility.py`, `test_api_compatibility.py`, `api-review.json` | Independent comparison of actual public Rust sources and pinned rustdoc-semver API review. |
| `openapi-to-rust-MIT.txt` | Upstream generator license attribution; retain. |
| `.github/scripts/` | Repository policy, PR-title validation and scheduled-source-update reporting, independent from the SDK build. |

`openapi-to-rust` owns generic overlay application/raw Rust output,
`openapi-to-rust-bindings` owns canonical Bindings v3 adaptation, and
`rust-sdk-generator` owns generic canonical derivation and Rust facade
emission. This repository invokes their pinned CLI contracts rather than
copying their implementations.

## Determinism and compatibility

`check` and `probe` use isolated workspaces. The raw generator runs twice
with a byte-level overlaid-spec determinism assertion, plus its own `--check`
and `--dry-run` modes. Raw bindings are formatted with pinned rustfmt;
the exact file inventory and output are compared against committed raw files
with **only** previously reviewed source-provenance and operation-path
normalizations. The Mistral coverage gate proves the 173 operation identities
with no unreviewed override or rejection. The canonical SDK derivation is
distinct from generation using `compatibility-definition.json`: the latter
must exactly reproduce the committed public facade before publication.

The historical `tooling/pipeline/compile_sdk.py` generated-file marker and
raw provenance-path normalization are compatibility contracts, not active
Python dependencies. No generated Rust, source pins or public SDK dependencies
were changed by the orchestration migration. The Python/Rust candidate was
exercised side by side and its generated raw/public outputs compared in
[PR #120](https://github.com/adriendellagaspera/mistralai-rs/pull/120).
CI continues to test a clean checkout, the explicit source-update sequence,
negative source/coverage/tool-pin cases and the independent public API gate.
