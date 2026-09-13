# Semantic SDK compiler

The public SDK facade is compiled, not handwritten. Its inputs are:

1. the pinned, unmodified Mistral OpenAPI document for wire semantics;
2. the generated raw Rust syntax tree for exact symbol reconciliation;
3. `sdk-semantics.json` for the small amount of product design HTTP cannot carry.

This follows the architecture used by current SDK generators such as
[Speakeasy](https://www.speakeasy.com/docs/sdks/customize/methods),
[Smithy](https://smithy.io/2.0/guides/building-codegen/index.html), and
[Fern](https://buildwithfern.com/learn/sdks/overview/how-it-works): a semantic
model separates protocol parsing from language-specific emission.

## Pipeline

```text
OpenAPI ─────────────┐
                    ├─ validation ─ typed SDK IR ─ generic Rust emitters
raw Rust AST ────────┤
semantic overlay ────┘
```

`sdk_codegen.py` contains no Chat, OCR, Models, or Mistral backend. It indexes
Rust with tree-sitter and compiles every configured domain through the same
model, union, operation, resource, stream, accessor, and client emitters.

The overlay is deliberately declarative. It assigns stable public names,
constructor field order, resource grouping, convenience access paths, and
intentional stream semantics. It contains no Rust statements or method bodies.
Discriminated unions are read from OpenAPI. When the document omits an explicit
discriminator, the compiler can infer one only when every branch has the same
unique const-valued property.

## Drift policy

Compatible changes are mechanical: an optional request property creates a
fluent setter and a new discriminated branch creates a public enum variant.
Ambiguous changes stop generation with a semantic-review error, including:

- OpenAPI and raw Rust field sets disagree;
- a required request field lacks constructor policy;
- an operation no longer accepts or returns the configured schema;
- an inferred union does not have one unambiguous discriminator;
- a convenience factory branch needs more than one unexplained input.

`scripts/codegen.py check` regenerates both raw and facade trees in isolation,
formats with the pinned Rust toolchain, and compares content and file sets. The
nightly update therefore refreshes the facade on every spec update and cannot
silently retain stale generated files.

## Extending coverage

Adding a resource using already-supported shapes normally requires only:

- a resource name and operation mapping;
- public wrapper names for selected request/response schemas;
- optional semantic conveniences such as a constructor order or accessor path.

No Python emitter is added per endpoint. A genuinely new OpenAPI shape requires
one generic IR capability plus an unrelated fixture test, rather than a
service-specific template. The test suite uses a fictional animal API and also
generates the real Models resource to enforce both sides of that rule.
