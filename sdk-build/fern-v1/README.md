# Fern Rust SDK generator 1.x migration

This workspace evaluates and prepares the migration from the production Fern
0.48 Rust generator to Rust SDK Config generator 1.0.0. It is isolated from
production until the generated SDK and its repository adaptations are reviewed.

Pinned tools:
- Fern CLI: 5.228.1
- Rust SDK generator: 1.0.0

The build compiles the compatibility-filtered OpenAPI source, hoists nested
JSON Schema definitions, validates local references, and generates two
parameter styles (`wrapped` and `inline`). For both styles it applies the
strict, unit-tested compatibility patches, inventories public functions and
query types, then runs `cargo fmt --check`, `cargo check --all-features`,
Clippy, and all generated-crate tests. A shared Cargo target directory avoids
duplicated compilation work.

Known Fern v1.0.0 gaps are handled deterministically:
- `patch_generated_sdk.py` repairs five omitted fixed-string type declarations,
  multipart serialization for binary files and JSON collection fields, and an
  incorrect Serde default on a required enum.
- `patch_generated_streaming.py` restores `chat.complete_stream` and
  `fim.complete_stream`, which the generator currently omits despite the
  conditional-SSE metadata in the OpenAPI operation.

Each patch asserts the expected generator output and has focused unit tests.
Rust source modifications are formatted before compilation. If the generator
output changes, the migration fails loudly rather than applying a stale patch.

Run from the repository root:

```sh
npm install --global fern-api@5.228.1
python3 -m pip install PyYAML==6.0.3
bash sdk-build/fern-v1/build.sh
```

The CI artifact `fern-v1-sdk-migration-candidate` retains both generated
crates as compressed archives, the OpenAPI input, schema-resolution data,
public API inventories, generation logs, and Cargo diagnostics.

The two parameter styles currently produce the same public method and query
type inventories. The generated SDK candidate passes compilation, formatting,
Clippy, and tests in CI. This is not yet the production migration: endpoint
coverage, public API changes, downstream tests/examples, and final checked-in
source materialization still require review. The generated crate currently
contains no unit tests, so its test gate does not establish endpoint behavior.
