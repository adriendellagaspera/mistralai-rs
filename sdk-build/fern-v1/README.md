# Fern Rust SDK generator v1 migration candidate

This isolated workspace tests the Rust SDK Config generator without changing the
production Fern 0.48 `generators.yml` pipeline. Production SDK sources are still
generated only by that pinned production pipeline; the v1 workspace is the migration candidate and is validated as part of its dedicated pull request.

The build regenerates the exact compatibility-filtered OpenAPI input used by
production, hoists response-local JSON Schema definitions into OpenAPI components,
normalizes the disjoint NDJSON summary frames to `oneOf`, then generates two Rust SDKs with Fern Rust generator 1.0.0 in Fern
guest mode (remote execution, no account token):

- `client.requestParameterStyle: wrapped`
- `client.requestParameterStyle: inline`

For each mode, the script inventories public `*QueryRequest` declarations and
public Rust function signatures **before** running `cargo check`. It then
compares both inventories. This distinguishes type-name stability from changes
to generated client method signatures. A compilation failure therefore does
not prevent the second mode from being generated or the inventories from being
compared.

The `fern-v1-sdk-migration-candidate` CI artifact (7-day retention) contains compressed generated SDK trees and inventories,
their diffs, Fern generation logs, Cargo diagnostics, schema resolution, and
the exact OpenAPI input. The generated crate trees are included as compressed archives for inspection.

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


Fern v1.0.0 currently emits several Rust defects for this API surface. The
build applies `patch_generated_sdk.py` to each freshly generated tree before
inventories and Cargo checks. The patch is deliberately narrow and fails on
unexpected generator output:

- Adds public `String` aliases for five fixed-string schema types that Fern
  references but does not emit.
- Serializes multipart collection/map fields as JSON and uploads `File`
  values as binary multipart parts rather than text.
- Removes an erroneous `serde(default)` from the required
  `ConversationMessages.messages` enum field.

The patch has isolated unit tests and is part of the tooling test job. These
workarounds are migration blockers to track upstream; they do not modify the
OpenAPI contract or hand-edit generated files outside the reproducible build.
