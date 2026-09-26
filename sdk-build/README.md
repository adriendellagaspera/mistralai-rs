# SDK build: responsibility map

The public crate is generated reproducibly from the pinned Mistral OpenAPI source. The private `sdk-build` workspace invokes pinned generic Rust generators and does not add build-time dependencies to SDK consumers.

## Canonical build path

```text
provenance.lock.json + openapi/published.yaml (288 operations)
   │ byte-level SHA-256 + reviewed overlay assumptions
   ▼
openapi-to-rust
   │ deterministic raw bindings + exact source coverage
   ▼
openapi-to-rust-bindings (Bindings v3)
   ▼
rust-sdk-generator
   │ canonical derivation: 283 derived + 5 reviewed transport/representation overrides
   ▼
src/generated + src/sdk
```

`check` regenerates everything in isolation and requires exact parity with the checked-in raw bindings and facade. `generate` runs the same gates and atomically publishes the canonical outputs. No legacy compatibility definition participates in the canonical build.

| Command | Scope |
| --- | --- |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- check` | Verify pinned source, generation, coverage and public facade parity without mutating the checkout. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- generate` | Run the same gates and publish canonical raw + facade outputs atomically. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- raw` | Explicitly refresh the reviewed raw baseline before a full generation. |
| `cargo run --quiet --locked --manifest-path sdk-build/Cargo.toml -- probe` | Inspect canonical derivation without publishing. |

The active source is `mistralai/platform-docs-public/public/openapi.yaml`, vendored as `openapi/published.yaml` and pinned in `provenance.lock.json`. Source discovery remains an explicit Python maintenance action via `openapi/update.py`; routine generation is Rust-only and network-independent apart from installing immutable pinned tools when absent.

`coverage-baseline.json` is fail-closed: all 288 source operations must be accounted for, no rejected operation is allowed, and only the five reviewed overrides may be classified as overridden. Those overrides model response/transport representations; they are not compatibility shims.

## API review and 0.x semver

`api-review/check.py` compares the actual Rust sources against the PR base and always runs the pinned `cargo-semver-checks`. Breaking changes normally fail the gate. A reviewed 0.x migration may set `allow_breaking_changes` in `api-review/review.json`; the semver report is still produced, but known breaking changes do not block that reviewed release.

`openapi-to-rust` owns generic OpenAPI-to-Rust emission, `openapi-to-rust-bindings` owns normalized bindings, and `rust-sdk-generator` owns generic SDK derivation/facade generation. This repository owns only Mistral-specific pins, overlays, transport overrides, runtime conventions and publication gates.
