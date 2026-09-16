# Semantic SDK compiler

The public SDK facade is compiled, not handwritten. Its semantic inputs are the
pinned OpenAPI document, a normalized view of the generated raw Rust bindings,
and `sdk-semantics.json` for product choices HTTP cannot express.

## Pipeline

```text
OpenAPI -----------------------> OpenApiIndex -----------\
                                                        \
openapi-to-rust output -> OpenApiToRustAdapter -> RawIr ---> validation/lowering -> FacadeIr -> sdk_emit
                                                        /
explicit semantic overlay -----------------------------/
```

`OpenApiToRustAdapter` is the only component that understands the current
`openapi-to-rust` source-file layout or parses generated source files directly
with tree-sitter. Everything after that boundary consumes `RawIr`; rendering
consumes only the fully resolved `FacadeIr`. `sdk_codegen.py` is retained only
as a compatibility surface for historical scripts/tests and delegates raw
loading to the same adapter.

The generic compiler (`sdk_compiler.py`) compiles an explicit policy. Official
SDK taxonomy discovery (`sdk_autoproject.py`) and repository audits such as
coverage, public-surface comparison and semver checks remain outside its
dependency graph.

The overlay is deliberately declarative. It assigns stable public names,
constructor field order, resource grouping, convenience access paths and
intentional stream semantics. It contains no Rust statements or method bodies.
Discriminated unions are read from OpenAPI; when no explicit discriminator is
present, inference is allowed only when every branch exposes one unambiguous
const-valued property.

## Raw binding contract

`RawIr` contains only the generated information facade lowering needs:

- struct names and public fields/types;
- enum names, variants and unary payload types;
- type aliases;
- raw operations, ordered parameters, return type and successful payload type;
- ownership of generated symbols by the `types` or `client` module.

`raw-ir.schema.json` defines a candidate deterministic machine-readable sidecar
for that contract. `RawIr.to_dict()` and `RawIr.from_dict()` round-trip the same
information. The in-memory representation may parse Rust type strings into the
generic `RustType` algebra for structural reasoning; the sidecar itself contains
only JSON values and type strings, never tree-sitter nodes or source locations.

An upstream `openapi-to-rust` improvement could emit this sidecar alongside
`types.rs` and `client.rs`. Once that output is available and versioned, the
facade tool can load it directly and remove source-file reparsing without
changing facade lowering. The proposed upstream contract is intentionally
limited to the fields above; HTTP/OpenAPI semantics remain owned by OpenAPI
rather than being duplicated in the sidecar.

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
generic shape requires one IR/lowering/emission capability plus an unrelated
fixture test rather than an endpoint-specific template.

The Menagerie fixture is the primary domain-independent proof. Mistral-specific
taxonomy harvesting, source pins, automatic projection policy and coverage goals
remain repository concerns rather than compiler semantics.

`sdk-semantics.schema.json` rejects unknown or ill-typed policy. `coverage.json`
inventories upstream operations. `codegen.py probe` compiles disposable
candidate projections offline, and `api-surface.json` plus pinned
cargo-semver-checks review public API evolution. These checks consume compiler
outputs; the compiler does not depend on them.
