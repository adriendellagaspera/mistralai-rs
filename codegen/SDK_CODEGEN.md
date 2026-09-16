# Semantic SDK compiler

The public SDK facade is compiled, not handwritten. `mistralai-rs` owns the
Mistral-specific inputs and repository checks; the generic compiler is the
separately versioned `openapi-to-rust-facade` tool pinned in `codegen.lock`.

## Pipeline

```text
Mistral OpenAPI ------------------------------\
                                              \
openapi-to-rust output -> pinned facade tool ---> FacadeIr -> Rust facade
                                              /
Mistral semantic policy ---------------------/

Mistral official SDK taxonomy -> auto-projection/planning
Mistral repository ------------> coverage/API/semver audits
```

The facade tool contains the generic `OpenApiToRustAdapter`, `RawIr`, OpenAPI
indexing, policy validation, facade lowering and Rust emission. Its adapter is
the only component that understands the current `openapi-to-rust` source-file
layout or parses generated Rust with tree-sitter. Rendering consumes only the
fully resolved `FacadeIr`.

`mistralai-rs` does not maintain a copied compiler implementation. The repository
keeps only Mistral-specific projection/taxonomy logic, generation orchestration,
coverage and public-API gates, plus a small compatibility bridge for historical
imports. `scripts/codegen.py` fetches the immutable facade-tool commit recorded in
`codegen.lock`, verifies its Git tree, and installs that checkout into the pinned
codegen Python environment.

The semantic overlay remains declarative. It assigns stable public names,
constructor field order, resource grouping, convenience access paths and
intentional stream semantics. It contains no Rust statements or method bodies.
OpenAPI remains the wire-contract authority.

## Raw binding contract

The facade tool's `RawIr` contains only the generated information facade
lowering needs:

- struct names and public fields/types;
- enum names, variants and unary payload types;
- type aliases;
- raw operations, ordered parameters, return type and successful payload type;
- ownership of generated symbols by the `types` or `client` module.

The tool ships a versioned `raw-ir.schema.json` and deterministic
`RawIr.to_dict()` / `from_dict()` round-trip. The sidecar contains JSON values and
Rust type strings, never tree-sitter nodes or source locations. A future
`openapi-to-rust` release can emit that contract directly and remove source
reparsing without changing facade lowering.

## Drift policy

Compatible changes are mechanical: an optional request property creates a
fluent setter and a new discriminated branch creates a public enum variant.
Ambiguous changes stop generation with a semantic-review error, including:

- OpenAPI and raw Rust field sets disagree;
- a required request field lacks constructor policy;
- an operation no longer accepts or returns the configured schema;
- an inferred union has no unique discriminator;
- a convenience factory branch needs more than one unexplained input.

`scripts/codegen.py check` regenerates raw bindings and the facade in isolation,
formats with the pinned Rust toolchain, and compares content and file sets. The
nightly update therefore refreshes the facade on every spec update and cannot
silently retain stale generated files.

## Extending coverage

Adding an operation composed only of already-supported primitives should need no
compiler change: policy/planning supplies names and selected conveniences. A new
generic shape belongs in `openapi-to-rust-facade`, with an unrelated fixture;
Mistral-specific taxonomy harvesting, source pins, automatic projection policy
and coverage goals remain here.

The extracted tool is independently tested with Menagerie and Library fixtures.
`coverage.json`, the projection probe, `api-surface.json` and pinned
`cargo-semver-checks` are consumer-side review gates and do not belong to the
generic compiler dependency graph.
