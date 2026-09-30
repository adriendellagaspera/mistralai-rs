# Development

Production generation requires Rust 1.94, Python 3.11+ with `PyYAML==6.0.3`, Node/npm, Fern CLI `5.112.0`, and Just.

```sh
npm install --global fern-api@5.112.0
python3 -m pip install PyYAML==6.0.3

just generate
just check-generated
just validate
```

SDK consumers need only Rust.

## Generation boundaries

`sdk-build/openapi/published.yaml` is the canonical 288-operation source. `sdk-build/fern/policy.yaml` owns Mistral resource/method/type identity and representation policy. The repository-local compiler in `sdk-build/sdk_surface/` turns that policy into deterministic Fern extensions and verifies the resulting Fern IR.

`prepare_fern_source.py` owns only temporary input compatibility for identified Fern importer bugs. `patch_fern_output.py` owns only the multipart+SSE generated-output workaround tracked by fern-api/fern#17928.

Do not edit `src/` by hand. Do not add a generic Rust facade over Fern, reintroduce `openapi-to-rust`, `openapi-to-rust-bindings`, canonical Bindings, or backend-symbol parsing.

## Updating sources

```sh
just sync-openapi
just sync-sdk-surface
just validate
```

`openapi/update.py` updates the verified Mistral source pin. `official-sdks/update.py` maintains independent Python/TypeScript evidence. Regeneration must preserve 288/288 closure unless the pinned source itself changes under explicit review.

## Fern workarounds

Every generic Fern workaround must have an upstream issue and a fail-closed target. The current list is documented in `sdk-build/fern/README.md`.

## Publishing

The package is `mistralai-sdk`; the Fern-native library import is `mistralai_sdk`. Before tagging, CI must be green and `cargo publish --dry-run --locked` must pass. Tags matching `Cargo.toml` use the repository trusted-publishing workflow.
