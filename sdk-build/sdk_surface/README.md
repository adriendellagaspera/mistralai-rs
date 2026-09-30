# SDK surface contract

This directory contains the small repository-local policy/proof layer used before and after Fern generation.

- `sdk_surface.py compile` resolves `sdk-build/fern/policy.yaml` against stable OpenAPI identities and emits deterministic Fern extensions.
- `sdk_surface.py verify` checks source-operation accounting, public method inventory, named-type closure, provenance, collisions and optional semantic compatibility against Fern IR.
- `sdk_surface.py digest` hashes the publishable generated tree while excluding Fern metadata.

The tool does not generate Rust, implement HTTP semantics, or parse generated Rust symbols. Fern remains the sole production Rust generator.

This code was extracted from the minimal surface-contract implementation proven in `rust-sdk-generator#240` so `mistralai-rs` remains fully reproducible without an external project dependency.
