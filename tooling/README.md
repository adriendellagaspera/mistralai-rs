# SDK tooling sidecar

This directory is development tooling, not part of the Rust SDK runtime.

- `sources/` tracks immutable OpenAPI, Python SDK and TypeScript SDK inputs and derives the compact Mistral product taxonomy.
- `pipeline/` adapts those inputs through `openapi-to-rust`, `openapi-to-rust-bindings` and `rust-sdk-generator` into committed Rust under `src/generated/` and `src/sdk/`.
- `quality/` owns deterministic regeneration, coverage probes, public API/semver review and update reporting.
- `tests/` exercises only this sidecar and its Mistral-specific contracts.

The OpenAPI remains the wire-contract source of truth. Official Python and TypeScript SDKs are evidence for product taxonomy and public surface only.
