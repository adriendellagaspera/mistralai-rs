# SDK build: responsibility map

The SDK consumer is Rust-only. The *private* build executable is a separate,
non-published Cargo workspace (`Cargo.toml`, `Cargo.lock`). It invokes only
pinned generic Rust CLI generators, never imports their implementations, and
does not add dependencies to the public SDK crate.

## The deterministic Rust build path — no Python

```text
provenance.lock.json + openapi/published.yaml
   │ src/sources.rs: pinned byte-level SHA-256, dialect and overlay assumptions
   ▼
src/main.rs: resolve immutable generator commits/trees; isolate inputs
   ▼
pinned openapi-to-rust: overlay + raw bindings (generate twice, check, dry-run)
   │ src/gates.rs: raw operation inventory and baseline parity
   ▼
pinned openapi-to-rust-bindings: normalized Bindings v3
   ▼
pinned rust-sdk-generator: canonical derive + 173-operation closed-world gate
   ▼
pinned rust-sdk-generator: frozen compatibility-definition.json
   │ src/gates.rs: exact published facade file inventory and bytes
   ▼
check: no checkout mutation  |  generate: verified raw + facade publication
```

Run from the repository root (or via Just):

| Command | Scope |
| --- | --- |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- check` | Full pinned build/coverage/facade parity with no Python and no checkout mutation. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- generate` | Same gates, then publish both validated outputs. No Python. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- raw` | Explicit source-update phase; publish reviewed raw bindings only. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- probe` | Inspect canonical derivation, optionally `--require-parity` or `--compatibility-definition PATH`. |
| `cargo test --locked --manifest-path sdk-build/Cargo.toml --all-targets` | Source-hash, overlay-assumption, coverage, parity and publication rollback tests. |

Rust source boundaries: `src/sources.rs` validates immutable Mistral
OpenAPI **bytes** against the pin before any generation; `src/main.rs` owns
the CLI, installed tool revisions, isolated workspace and publish transaction;
`src/gates.rs` owns consumer-specific normalized raw inventories, operation
coverage, overlays and exact facade comparison. No generic SDK parsing or
compiler logic is maintained here.

## Explicit source refresh and evidence — Python remains intentional

| Directory | Artifacts and owner | When it runs |
| --- | --- | --- |
| `openapi/` | `published.yaml`, `overlays/` and license are pinned inputs. `update.py` discovers/downloads the versioned upstream `mistralai/platform-docs-public/openapi.yaml` revision and changes the source pin explicitly. The build no longer calls a Python source checker. | `just sync-openapi` and scheduled source-update workflow, **not** routine `check`/`generate`. |
| `official-sdks/` | `harvest.py` analyzes the official Python AST and TypeScript sources; `update.py` coordinates `pin-latest`, `update`, `check`; `surface.json` is the reviewed input read by the Rust build. | `just check-source-evidence`, explicit source refresh, `just validate` and CI's independent source-evidence step. |
| `api-review/` | `check.py`, `test_check.py`, `review.json`: inspect actual public Rust changes against a PR base and run pinned compiler-aware semver checks. | CI's `api` job; Python test discovery in `just test-tooling`/`just validate`. Not part of deterministic generation. |

`provenance.lock.json` pins source checksum, tool revisions/trees,
official-SDK revisions and API semver checker; `openapi-to-rust.toml`,
`sdk-overrides.json`, `coverage-baseline.json` and
`compatibility-definition.json` are reviewed build inputs at the root of
`sdk-build/`. `openapi-to-rust-MIT.txt` preserves upstream attribution.

The docs-site URL `https://docs.mistral.ai/openapi.yaml` is **not** a byte mirror of\nthe versioned repository source. It serves a broader API catalog; the root\n`openapi.yaml` currently defines the explicitly pinned SDK scope. `update.py`\nnever silently switches to the docs-site catalog. See [#127](https://github.com/adriendellagaspera/mistralai-rs/issues/127)\nfor source investigation and [#16](https://github.com/adriendellagaspera/mistralai-rs/issues/16)\nfor future expansion of idiomatic API coverage.\n\n## Validation boundaries

`just check-generated` and `just generate` are Python-free. In contrast,
`just validate` intentionally combines deterministic Rust regeneration with
Python source-evidence and API-review/repository-policy tests. CI additionally
exercises `check` and `generate` with `python` and `python3` blocked in
`PATH`, negative source/coverage/tool-pin cases, the source-update dry run
and independent API compatibility checks. Source discovery is *not* part of
ordinary generation; its results are reviewed and pinned before build.

`openapi-to-rust` owns generic overlay application/raw Rust emission,
`openapi-to-rust-bindings` owns normalized bindings, and
`rust-sdk-generator` owns generic derivation/facade generation. This repo
supplies only Mistral-specific inputs, invariants and runtime conventions.
The historical `tooling/pipeline/compile_sdk.py` marker in generated Rust
is a byte-parity contract, not an active dependency. The published OpenAPI,
operation baseline, generated bindings/facade and public Cargo dependency
surface are unchanged by this reorganization.
