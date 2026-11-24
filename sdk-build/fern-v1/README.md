# Fern Rust SDK generator v1 migration candidate

This isolated workspace tests the Rust SDK Config generator without changing the
production Fern 0.48 `generators.yml` pipeline. Production SDK sources are still
generated only by that pinned production pipeline; the v1 workspace is the migration candidate and is validated as part of its dedicated pull request.

The build regenerates the exact compatibility-filtered OpenAPI input used by
production, then generates two Rust SDKs with Fern Rust generator 1.0.0 in Fern
guest mode (remote execution, no account token):

- `client.requestParameterStyle: wrapped`
- `client.requestParameterStyle: inline`

For each mode, the script inventories public `*QueryRequest` declarations and
public Rust function signatures **before** running `cargo check`. It then
compares both inventories. This distinguishes type-name stability from changes
to generated client method signatures. A compilation failure therefore does
not prevent the second mode from being generated or the inventories from being
compared.

The `fern-v1-sdk-migration-candidate` CI artifact (7-day retention) contains the generated SDK trees and inventories,
their diffs, Fern generation logs, Cargo diagnostics, schema resolution, and
the exact OpenAPI input. Generated crate trees are not uploaded.

Run from the repository root:

```sh
npm install --global fern-api@5.228.1
python3 -m pip install PyYAML==6.0.3
bash sdk-build/fern-v1/build.sh
```


The v1 generation job runs on the dedicated `migration/fern-rust-v1` pull request and can also be dispatched manually with `run_fern_v1`. Ordinary pull requests do not consume Fern guest-generation sessions.

The v1 Rust generator currently flattens some nested resource groups, which can
introduce numeric method-name suffixes. Production Fern 0.48 preserves those
nested groups, so the v1-specific suffixes are not copied into the production
naming policy.

The migration gate requires the generated crate to pass formatting, compilation, Clippy, and all tests. Generated files are retained as CI artifacts while the migration is under review; they are not considered production sources until the migration is complete.
