# Development

Production generation requires Rust 1.94, Python 3.11+ with `PyYAML==6.0.2`, Node/npm, Fern CLI `5.112.0`, and Just. SDK consumers need only Rust.

```sh
python3 -m pip install 'PyYAML==6.0.2'
npm install --global 'fern-api@5.112.0'
just generate
just check-generated
just validate
```

## Generation boundaries

`sdk-build/openapi/published.yaml` is the canonical 288-operation source. `sdk-build/fern/product-policy.json` owns Mistral product hierarchy, short verbs, parameter SDK names, and chat/FIM streaming extensions. `sdk-build/fern/compat.py` owns only temporary OpenAPI-importer workarounds for upstream Fern bugs. `sdk-build/fern/workarounds.py` owns only the multipart+SSE generated-output workaround.

Do not edit `src/` by hand. Do not add a generic Rust facade over Fern, reintroduce `openapi-to-rust`, `openapi-to-rust-bindings`, or canonical Bindings, or put Mistral logic in a generic Fern fork.

## Updating the OpenAPI source

```sh
just sync-openapi
just sync-sdk-surface
just validate
```

`openapi/update.py` updates the verified Mistral source pin and license material. `official-sdks/update.py` maintains independent Python/TypeScript evidence. Regeneration then runs Fern and must preserve 288/288 closure unless the source itself changes under explicit review.

A source update that changes operation count or public method/type inventory fails closed. Review `product-policy.json`, the historical transport assertions in `gates.py`, `public-inventory.json`, and `API_COMPATIBILITY.md` before accepting API changes.

## Fern workarounds

Every generic Fern workaround must have an upstream issue and a fail-closed target. When a pinned Fern release fixes an issue, delete the local workaround rather than making it more permissive. The current list is documented in `sdk-build/fern/README.md` and recorded in `public-inventory.json`.

## Publishing

The package is `mistralai-sdk`; the generated library is imported as `mistralai_sdk`. Before tagging a release, CI must be green and `cargo publish --dry-run --locked` must pass from the release commit. Tags matching `Cargo.toml` use the repository's trusted-publishing workflow.
