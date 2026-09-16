# Code generation

`main` owns only Mistral-specific generation inputs and orchestration. Generic tooling is consumed at immutable revisions from `codegen.lock`:

- `openapi-to-rust` generates `src/generated/`;
- `openapi-to-rust-bindings` normalizes generated Rust into `Bindings`;
- `rust-sdk-compiler` compiles `OpenApi + Bindings + Policy + Runtime` into `src/sdk/`.

Repository-owned code is limited to:

- `openapi-to-rust.toml`, `patches/` and `preprocess.py` for the Mistral raw-generation contract;
- `sdk-semantics.json`, `sdk-taxonomy.json` and `sdk-probe-baseline.json` for Mistral policy/evidence;
- `mistral/` for Mistral projection, orchestration and API-audit helpers;
- `sdk_facade.py` as the stable facade-generation entry point.

Generated Rust is committed and must not be edited by hand. `scripts/codegen.py check` regenerates both layers and compares file sets and bytes.

Generic compiler/adapter design and tests belong with their packages, not in this repository.
