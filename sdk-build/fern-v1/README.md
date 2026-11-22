# Fern Rust SDK generator 1.x experiment

This isolated workspace tests the Rust SDK Config generator without changing the
production legacy `generators.yml` pipeline or committed SDK sources.

The build regenerates the exact compatibility-filtered OpenAPI input used by
production, then generates two Rust SDKs with Fern Rust generator 1.0.0 in Fern
guest mode (remote execution, no account token):

- `client.requestParameterStyle: wrapped`
- `client.requestParameterStyle: inline`

For each mode, the script inventories public `*QueryRequest` declarations
**before** running `cargo check`. A compilation failure therefore does not
prevent the second mode from being generated or the inventories from being
compared. The build retains Fern generation logs, Cargo diagnostics, both
generated crates, and the type inventories as the `fern-v1-sdk-audit` CI
artifact (7-day retention), including when compilation fails.

Run from the repository root:

```sh
npm install --global fern-api@5.226.1
python3 -m pip install PyYAML==6.0.3
bash sdk-build/fern-v1/build.sh
```

Generated files are temporary and are not production SDK sources.
