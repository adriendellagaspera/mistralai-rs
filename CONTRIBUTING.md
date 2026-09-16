# Development

Install Git, Rustup, Python 3.11+ with `venv`, and Just. Run commands from the repository root. Linux and macOS are the intended codegen environments; consumers need only Rust.

```sh
just generate
just check-generated
bash scripts/validate.sh
```

Without Just, use `python3 scripts/codegen.py generate` and `python3 scripts/codegen.py check`. No xtask is needed. Codegen creates an isolated Python environment under `.tools/`; no global Python package installation is required.

`codegen.lock` pins the Rust toolchain, raw generator source commit and patch hash, `rust-sdk-compiler`, `openapi-to-rust-bindings`, their Git trees, and Python dependencies. `scripts/codegen.py` verifies those immutable inputs before generation. The raw generator is built with its committed lockfile; the two Python tools are installed from their exact pinned commits. `Cargo.lock` pins this SDK's validation dependencies.

## Updating the source specification

```sh
just update-spec
just generate
bash scripts/validate.sh
python3 scripts/update_report.py
git diff -- spec codegen.lock src/generated src/sdk
```

`update-spec` resolves the most recent commit touching `openapi.yaml`, fetches the spec at that immutable SHA, and only changes the checkout if its SHA-256 differs. It downloads upstream licensing at the same revision. Optional `GH_TOKEN` raises the GitHub API rate limit; no secret is required. A failed download does not replace the pinned spec. No endpoint credential is used.

Review API changes before committing. If new generated dependencies appear in `src/generated/REQUIRED_DEPS.toml`, reconcile them with `Cargo.toml` and update `Cargo.lock` explicitly. Generator and compiler upgrades are separate reviewed changes to `codegen.lock`, never automatic side effects of a spec update.

## Extending the SDK

There is no raw-operation allowlist: all upstream operations and schemas are generated. `src/generated/coverage.json` records the raw operation inventory. The idiomatic surface is separately tracked in `src/sdk/coverage.json`.

The generation boundaries are deliberate:

- `openapi-to-rust` owns raw Rust generation;
- `openapi-to-rust-bindings` normalizes that generated Rust into `Bindings`;
- `rust-sdk-compiler` compiles `OpenApi + Bindings + Policy + Runtime` into deterministic Rust SDK files;
- this repository owns Mistral taxonomy, auto-projection policy, runtime integration, coverage and API-review gates.

Generic compiler behavior belongs in `rust-sdk-compiler` and should be demonstrated with a non-Mistral fixture. A raw-generator compatibility fix belongs in `openapi-to-rust-bindings` when it only concerns the shape produced by `openapi-to-rust`. Mistral-specific naming or projection decisions stay here.

Do not edit `src/generated/` or generated files in `src/sdk/` by hand. Prefer generator configuration, then a narrowly scoped source patch or explicit preprocessing for raw generation. Source patches must apply with `git apply --check` to the pinned generator commit and have their SHA-256 updated in `codegen.lock`. Rustfmt remains the only post-generation Rust transform.

`codegen/README.md` maps the repository-owned generation files. The active Mistral semantic policy is `codegen/sdk-semantics.json`; generic compiler and adapter contracts live with their packages and tests. Preserve the vendored official spec exactly; preprocessing operates on a temporary generation copy and fails closed when its expected source shape changes.

## Tests and live requests

Normal tests use fixtures and local HTTP servers and require no API key or external API connection. Both chat examples are compiled in CI but never run. To make a live request explicitly, set `MISTRAL_API_KEY` and run `cargo run --example chat`. Do not commit credentials or live response dumps.

`scripts/validate.sh` checks deterministic regeneration, formatting, compilation, Clippy with warnings denied, Rust tests/docs, coverage probes and public API evolution. Do not weaken a failing gate merely to advance generated output; fix the owning boundary instead.

## Automation failures

The update workflow validates before writing its stable PR branch. If upstream generation, compilation, Clippy or tests fail, the run stays failed and no new candidate is pushed. Inspect the failing step and fix the relevant generator, bindings normalization, compiler capability, Mistral policy or test.

The workflow's hash comparison avoids commits for metadata-only upstream changes. A second run against the same pending upstream update reuses the same branch/PR. Do not enable automatic merge or add a PAT just to get a second CI run: the nightly already runs the same gates.

## Contributions

Unless explicitly stated otherwise, contributions are licensed under MIT OR Apache-2.0. Preserve all third-party license and attribution notices.
