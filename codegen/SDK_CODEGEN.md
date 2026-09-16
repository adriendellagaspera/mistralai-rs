# SDK code generation

The public Rust SDK surface is compiled, not handwritten. `mistralai-rs` owns Mistral-specific policy, runtime integration and repository gates; generic code generation is supplied by independently pinned tools.

## Pipeline

```text
openapi-to-rust -> generated Rust -> openapi-to-rust-bindings -> Bindings
                                                              |
Mistral OpenAPI -------------------------------------> OpenApi |
Mistral semantic policy -----------------------------> Policy  |
Mistral runtime conventions -------------------------> Runtime |
                                                              v
                                                       rust-sdk-compiler
                                                              |
                                                              v
                                                        Compilation -> src/sdk

Official Python/TypeScript SDKs -> Mistral taxonomy / auto-projection
mistralai-rs                    -> coverage / API / semver audits
```

`rust-sdk-compiler` is backend-neutral and Rust-specific. It does not parse generated source and knows nothing about `openapi-to-rust`, tree-sitter or Mistral. Its public API is deliberately small: `OpenApi`, `Bindings`, `Policy`, `Runtime`, `Compilation`, `lower()` and `compile()`.

`openapi-to-rust-bindings` owns compatibility with the current raw generator output. `parse_bindings()` and `read_bindings()` normalize `types.rs` / `client.rs`, serde wire names, operation signatures, symbol paths and stream ABI into `Bindings`. Backend parsing does not belong in the compiler.

`scripts/codegen.py` fetches both tools at the exact commits recorded in `codegen.lock`, verifies their Git trees and installs them in an isolated environment. `codegen/sdk_pipeline.py` owns the Mistral-specific orchestration around those public APIs.

## Inputs

`OpenApi` is the HTTP/wire contract. `Bindings` describes the Rust surface that actually exists:

- structs, fields and types;
- enums, variants and wire names;
- aliases and resolved Rust symbol paths;
- operations, ordered parameters, return/success types and stream semantics;
- the generated client path and its construction/configuration methods.

`Policy` describes only the desired public SDK shape: names, resource hierarchy, constructors, accessors and intentional transport conveniences. It contains no Rust method bodies.

`Runtime` contains consumer-owned support paths such as `SdkError`, streaming helpers and the generated-file marker. Error conversion and stream support specific to this crate therefore remain in `mistralai-rs`.

The compiler ships a versioned `rust-bindings.schema.json` interchange contract. A backend producer may construct `Bindings` directly or serialize that schema. If `openapi-to-rust` eventually emits equivalent metadata itself, `openapi-to-rust-bindings` can shrink to validation/loading without changing the compiler or Mistral policy.

## Ownership

A change belongs in `rust-sdk-compiler` when it is a generic Rust SDK compilation capability and can be demonstrated with an unrelated fixture. A change belongs in `openapi-to-rust-bindings` when only the concrete raw-generator representation changed. Mistral taxonomy, official-SDK evidence, source pins, auto-projection decisions and coverage targets stay in this repository.

Repository audit artifacts (`coverage.json`, projection probes, `api-surface.json`, cargo-semver-checks) consume compiler output but are not compiler inputs.

## Drift policy

Generation fails closed when source contracts cannot be reconciled safely. Examples include:

- OpenAPI and Rust binding fields disagree;
- a required request field has no valid public projection;
- an operation no longer accepts or returns the configured schema;
- a union/discriminator cannot be resolved uniquely;
- an unsupported request or response transport appears.

Compatible additions are handled mechanically when an existing primitive already describes them. A new generic primitive is implemented once in the compiler with non-Mistral fixture coverage, then consumed here through policy/planning.

`scripts/codegen.py check` regenerates both raw bindings and the SDK surface in temporary directories, formats with the pinned Rust toolchain and compares the complete committed file sets byte-for-byte. No timestamp or mutable dependency is allowed to affect output.
