# Fern Rust SDK generator 1.x experiment

This isolated workspace tests the Rust SDK Config generator without changing the
production legacy `generators.yml` pipeline or committed SDK sources.

The build regenerates the exact compatibility-filtered OpenAPI source used by
production, then generates two Rust SDKs with Fern Rust generator 1.0.0:

- `client.requestParameterStyle: wrapped`
- `client.requestParameterStyle: inline`

Both generated crates are compiled. The script records the public
`*QueryRequest` types from each output and compares the two inventories.

Run from the repository root:

```sh
npm install --global fern-api@5.226.1
python3 -m pip install PyYAML==6.0.3
bash sdk-build/fern-v1/build.sh
```

Generated files are temporary and are not production SDK sources.
