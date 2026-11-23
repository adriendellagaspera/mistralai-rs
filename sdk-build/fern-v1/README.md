# Fern Rust SDK generator 1.x experiment

This isolated workspace tests the Rust SDK Config generator without changing the
production Fern 0.48 `generators.yml` pipeline. Production SDK sources are still
generated only by that pinned production pipeline; the v1 workspace is diagnostic.

The build regenerates the exact compatibility-filtered OpenAPI input used by
production, then generates two Rust SDKs with Fern Rust generator 1.0.0 in Fern
guest mode (remote execution, no account token):

- `client.requestParameterStyle: wrapped`
- `client.requestParameterStyle: inline`

For each mode, the script inventories public `*QueryRequest` declarations and
public Rust function signatures before running `cargo fmt`, `cargo check`,
Clippy, and the generated crate's full test targets. It repeats each style to prove deterministic tree output, compares wrapped and
inline inventories, and emits diffs against the checked-in Fern 0.48 public
query types, endpoint method names, and function signatures. Those diffs are diagnostic because v1 changes
resource grouping and request APIs. A compilation failure does not prevent the
other checks or the second parameter style from being evaluated.

The `fern-v1-sdk-audit` CI artifact (7-day retention) contains the inventories,
their diffs, per-style and repeat generated-tree digests, Fern generation logs, Cargo
format/check/Clippy/test diagnostics, production-vs-v1 inventory diffs, schema
resolution, and the exact OpenAPI input. Generated crate trees are not uploaded.

Run from the repository root:

```sh
npm install --global fern-api@5.228.1
python3 -m pip install PyYAML==6.0.3
bash sdk-build/fern-v1/build.sh
```


The v1 CI job remains opt-in because Fern guest mode has a daily session quota.
It can be dispatched manually with `run_fern_v1`; the dedicated evaluation PR
also enables it explicitly. Ordinary pull-request CI does not consume guest
sessions.

The v1 Rust generator currently flattens some nested resource groups, which can
introduce numeric method-name suffixes. Production Fern 0.48 preserves those
nested groups, so the v1-specific suffixes are not copied into the production
naming policy.

Generated files are temporary and are not production SDK sources.
