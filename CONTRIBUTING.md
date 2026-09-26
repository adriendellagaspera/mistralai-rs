# Development

Install Git, Rustup and Just for SDK regeneration. Python 3.11+ is required only for source harvesting, independent API review and repository-policy checks. Use a clean checkout and run commands from the repository root. The SDK consumer needs only Rust.

```sh
just generate
just check-generated
just validate
```

Without Just, run `cargo run --locked --manifest-path sdk-build/Cargo.toml -- generate` and `cargo run --locked --manifest-path sdk-build/Cargo.toml -- check`. The private Rust build executable fetches and installs the pinned Rust CLIs under `.tools/`; it verifies the pinned published-source SHA-256 and overlay assumptions directly in Rust. Neither command invokes Python.

`sdk-build/provenance.lock.json` pins the published Mistral OpenAPI, official SDK evidence, the raw generator, the canonical Rust SDK generator, the bindings adapter, and their immutable Git revisions. Root `Cargo.lock` pins the SDK's runtime and test dependencies; `sdk-build/Cargo.lock` independently pins the private build executable.

## Updating sources

```sh
just sync-openapi
just sync-sdk-surface
just validate
python3 .github/scripts/update_report.py
git diff -- sdk-build src/generated src/sdk
```

`.github/scripts/` owns repository policy, PR-title checks and the scheduled-update PR report; `sdk-build/` owns only Mistral SDK construction. The [SDK-build responsibility map](sdk-build/README.md) separates the Rust build path, pinned source collectors and independent API review.

`sdk-build/openapi/update.py` updates the verified official published mirror and source pin, and refreshes licenses. The private Rust builder validates the pinned source checksum, dialect and reviewed overlay assumptions without Python. `sdk-build/official-sdks/update.py` harvests pinned Python and TypeScript public paths; neither changes the OpenAPI wire contract. Optional `GH_TOKEN` is used only for GitHub API requests.

An upstream source update may require review of `sdk-build/coverage-baseline.json` and the five transport overrides in `sdk-build/sdk-overrides.json`. Never relax the strict coverage gate or change public methods silently to make an update pass. The complete canonical facade is derived from the pinned source.

## Build boundaries

The build runs the pinned `openapi-to-rust` raw generator and `openapi-to-rust-bindings` adapter, then uses the backend-neutral `rust-sdk-generator` in two independent modes. Canonical derivation must account for every source operation; generation from the reviewed compatibility definition must reproduce the committed public SDK exactly.

The derived SDK definition is generated from the pinned source, not maintained as a legacy compatibility contract. Generic compiler changes belong in `rust-sdk-generator`; wire-contract corrections belong in reviewed OpenAPI overlays. Do not maintain endpoint-specific generator heuristics in this repository.

Never edit `src/generated/` or generated Rust in `src/sdk/` by hand. Generated changes belong to pinned source/configuration updates and must pass determinism, coverage, exact facade parity, compilation, tests and compiler-aware semver review.

## Validation

`just generate` and `just check-generated` run without Python; `just check-source-evidence` separately validates the official SDK taxonomy using the Python collector. `just validate` deliberately includes both tracks, independent Python API-review/repository-policy tests, formatting, Clippy and Rust tests/docs. CI also checks public API compatibility against the PR base and dependency policy. Offline tests use fixtures/local HTTP servers; live Mistral requests are explicit opt-in only.

## Automated updates

The scheduled source-update workflow creates a candidate PR for changed source pins, including a blocked PR with the failed stage recorded when generation or validation fails. On a source incompatibility, it fails closed instead of silently changing the public SDK. Review source pins, generated changes, coverage and semver before merging.

## Publishing

The package name is `mistralai-sdk` and the library import is `mistralai`. The first crates.io release needs a manual upload by a crate owner: crates.io does not allow a trusted publisher to create a new crate. After the release commit passes CI, run `cargo publish --dry-run --locked` and `cargo publish --locked` from that commit using your own crates.io account. The first upload permanently reserves the name and version.

Then configure the crate's GitHub trusted publisher for repository `adriendellagaspera/mistralai-rs` and workflow file `publish.yml`. Later, push a tag matching the `Cargo.toml` version (`v0.4.1`, for example) to trigger [Publish crate](.github/workflows/publish.yml). Its preflight checks the registry and package, publishes with a short-lived credential, and creates a GitHub Release. Rerunning the workflow for an already published version skips the upload.
