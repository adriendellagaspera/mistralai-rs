# Semantic SDK compiler

The public Rust SDK surface is compiled, not handwritten. `mistralai-rs` owns
Mistral-specific policy, runtime support and repository checks. Two separately
versioned tools are pinned in `codegen.lock`:

- `rust-sdk-compiler`: the backend-neutral compiler;
- `openapi-to-rust-bindings`: the compatibility layer that normalizes the current
  raw generator output into the compiler's `Bindings` contract.

## Pipeline

```text
openapi-to-rust output
        |
        v
openapi-to-rust-bindings
        |
        v
     Bindings --------------------\
                                   \
Mistral OpenAPI ------> OpenApi ----> rust-sdk-compiler -> Compilation
                                   /
Mistral policy --------> Policy ---/
Mistral runtime -------> Runtime --/

Mistral official SDK taxonomy -> auto-projection/planning
Mistral repository ------------> coverage/API/semver audits
```

`rust-sdk-compiler` knows nothing about `openapi-to-rust`, generated file names,
`HttpClient`, tree-sitter or Mistral. Its small public API is `OpenApi`,
`Bindings`, `Policy`, `Runtime`, `Compilation`, `lower()` and `compile()`.
`lower()` produces the resolved SDK IR; `compile()` additionally renders the
deterministic Rust file set.

`openapi-to-rust-bindings` owns every current backend convention: `types.rs`,
`client.rs`, `HttpClient`, generated module paths, serde enum renames and the raw
stream ABI. Its public API is functional: `parse_bindings()` and
`read_bindings()` return the compiler's public `Bindings` value. It imports no
compiler internals.

`mistralai-rs` does not maintain a copied compiler implementation. The repository
keeps Mistral-specific projection/taxonomy logic, generation orchestration,
runtime support, coverage and public-API gates, plus narrow local shims for
historical repository tests. `scripts/codegen.py` fetches both immutable tool
commits from `codegen.lock`, verifies their Git trees and installs them into the
pinned codegen environment.

The semantic policy remains declarative. It assigns stable public names,
constructor field order, resource grouping, convenience access paths and
intentional stream semantics. It contains no Rust statements or method bodies.
OpenAPI remains the wire-contract authority.

## Bindings contract

`Bindings` describes the Rust surface that actually exists and that the compiler
may target:

- structs and public fields/types;
- enums, variants, wire names and unary payloads;
- type aliases;
- raw operations, ordered parameters, return and successful payload types;
- normalized stream item/error/lifetime semantics;
- fully resolved Rust paths for generated symbols;
- the raw client path and its constructor/configuration methods.

The compiler ships the versioned `rust-bindings.schema.json` interchange format.
The representation contains JSON values and Rust type strings, never parser
nodes, source locations or generator-specific module labels. A producer can
construct `Bindings` directly or serialize the same schema.

If a future `openapi-to-rust` release emits this contract directly,
`openapi-to-rust-bindings` can collapse to validation/loading without any change
to `rust-sdk-compiler` or Mistral policy.

## Runtime ownership

The compiler does not own SDK-specific support code. `Runtime` tells emission
which consumer-owned error and streaming paths to reference. In this repository,
`src/sdk/error.rs`, `crate::streaming` and the historical generated-file marker
remain Mistral concerns. Raw network-error conversion therefore belongs here,
not in the generic compiler.

## Drift policy

Compatible changes are mechanical: an optional request property creates a
fluent setter and a new discriminated branch creates a public enum variant.
Ambiguous changes stop generation with a semantic-review error, including:

- OpenAPI and Rust binding field sets disagree;
- a required request field lacks constructor policy;
- an operation no longer accepts or returns the configured schema;
- an inferred union has no unique discriminator;
- a convenience factory branch needs more than one unexplained input.

`scripts/codegen.py check` regenerates raw bindings and the SDK surface in
isolation, formats with the pinned Rust toolchain, and compares content and file
sets. The nightly update therefore cannot silently retain stale generated files.

## Extending the compiler

Adding an operation composed only of already-supported primitives should need no
compiler change: policy/planning supplies names and selected conveniences. A new
generic compiler shape belongs in `rust-sdk-compiler` and must be demonstrated by
an unrelated fixture. Changes caused only by the concrete raw generator belong
in `openapi-to-rust-bindings`.

Mistral-specific taxonomy harvesting, source pins, automatic projection policy
and coverage goals remain in this repository. `coverage.json`, projection probes,
`api-surface.json` and pinned `cargo-semver-checks` are consumer-side review gates
and do not belong to either generic package.
