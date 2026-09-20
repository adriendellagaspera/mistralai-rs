# Development

Install Git, Rustup, Python 3.11+ and Just. Use a clean checkout and run commands from the repository root. The SDK consumer needs only Rust.

```sh
just generate
just check-generated
just validate
```

Without Just, run `python3 sdk-build/build.py generate` and `python3 sdk-build/build.py check`. The build fetches and installs the pinned Rust tools under `.tools/`; it does not require global Python packages.

`sdk-build/provenance.lock.json` pins the published Mistral OpenAPI, official SDK evidence, the raw generator, the canonical Rust SDK generator, the bindings adapter, and their immutable Git revisions. `Cargo.lock` pins the SDK's runtime and test dependencies.

## Updating sources

```sh
just sync-openapi
just sync-sdk-surface
just validate
python3 .github/scripts/update_report.py
git diff -- sdk-build src/generated src/sdk
```

`.github/scripts/` owns repository policy, PR-title checks and the scheduled-update PR report; `sdk-build/` owns only Mistral SDK construction. The [SDK-build ownership map](sdk-build/README.md) lists every input, active caller and validation command.

`sdk-build/openapi/update.py` verifies the official published mirror against the immutable upstream OpenAPI and refreshes licenses. `sdk-build/official-sdks/update.py` harvests pinned Python and TypeScript public paths; neither changes the OpenAPI wire contract. Optional `GH_TOKEN` is used only for GitHub API requests.

An upstream source update may require review of `sdk-build/coverage-baseline.json` or `sdk-build/compatibility-definition.json`. Never relax the strict compatibility gate or change public methods silently to make an update pass. Uncovered new operations remain available through `mistralai::raw` until a separately reviewed SDK surface change.

## Build boundaries

The build runs the pinned `openapi-to-rust` raw generator and `openapi-to-rust-bindings` adapter, then uses the backend-neutral `rust-sdk-generator` in two independent modes. Canonical derivation must account for every source operation; generation from the reviewed compatibility definition must reproduce the committed public SDK exactly.

The frozen compatibility definition is an explicit public API contract, not a copy of generated Rust source. It can be changed only through an intentional public API review. Generic compiler changes belong in `rust-sdk-generator`; wire-contract corrections belong in reviewed OpenAPI overlays. Do not maintain endpoint-specific generator heuristics in this repository.

Never edit `src/generated/` or generated Rust in `src/sdk/` by hand. Generated changes belong to pinned source/configuration updates and must pass determinism, coverage, exact facade parity, compilation, tests and compiler-aware semver review.

## Validation

`just validate` runs pinned regeneration, official SDK evidence checks, formatting, Clippy, Rust tests and docs, and the sdk-build unit tests. CI also checks public API compatibility against the PR base and dependency policy. Offline tests use fixtures/local HTTP servers; live Mistral requests are explicit opt-in only.

## Automated updates

The scheduled source-update workflow creates a candidate PR only after generation and validation succeed. On a source incompatibility, it fails closed instead of silently changing the public SDK. Review source pins, generated changes, coverage and semver before merging. No workflow automatically publishes a release.
