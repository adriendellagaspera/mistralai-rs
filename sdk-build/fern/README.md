# Fern production build

Fern is the sole production Rust generator for the Tier-1 OpenAPI SDK.

Pipeline:

```text
pinned Mistral OpenAPI
  -> temporary, ticketed Fern source compatibility
  -> SDK surface policy
  -> deterministic Fern OpenAPI extensions
  -> Fern IR + Rust generation
  -> source/public inventory, closure, provenance and collision checks
  -> deterministic tree proof
  -> published src/
```

Pins are in `../provenance.lock.json`. The SDK surface compiler is pinned to
`adriendellagaspera/rust-sdk-generator@394d12ee893a256123545b75ba5e4a0f8f354ad7`.
It consumes OpenAPI/Fern IR; it does not parse generated Rust.

Temporary Fern workarounds are deliberately isolated:

- allOf property narrowing: https://github.com/fern-api/fern/issues/17930
- leading-hyphen enum default: https://github.com/fern-api/fern/issues/17931
- multipart + SSE Rust generation: https://github.com/fern-api/fern/issues/17928

The inline `JudgeOutput` collision is handled as reviewed type policy via
`x-fern-type-name`; the generic Fern defect is tracked at
https://github.com/fern-api/fern/issues/17929. Resource grouping currently
emits an explicit method extension as required by Fern 5.112.0; the importer
limitation is tracked at https://github.com/fern-api/fern/issues/17932.

`generate.sh` regenerates and materializes `src/`. `check.sh` regenerates
twice, proves deterministic output, verifies all 288 source operations, builds
the generated crate, and requires byte-equivalent committed Rust sources plus
the reviewed semantic inventory.
