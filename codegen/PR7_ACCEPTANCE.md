# PR #7 acceptance evidence

Scope: a reproducible, generic facade compiler exercised on Chat, OCR and Models;
not an ergonomic facade for all Mistral operations. Green CI is necessary but
does not establish architectural completeness.

## Implemented checks

- Structural `Option`/`Vec` inspection with the pinned Rust grammar, including
  nested generics, whitespace and rejection of malformed/injected types.
- Immutable typed model policies form a closed request/union/view union;
  stream policy and accessors are typed separately. Raw fields and parameters
  retain structural type nodes at the binding boundary.
- Central public symbol allocation rejects keywords/collisions with provenance.
  Protocol field keywords are escaped. Parameter types resolve to their actual
  `raw::client` or `raw::types` module, including nested generic arguments.
- Owned operation request objects, with private fields and fluent optional
  setters. A mutation test adds an optional query parameter and verifies that
  the resource signature remains unchanged and the declaration diff is additive.
- A versioned JSON Schema rejects unknown keys, wrong nested types and mutually
  exclusive model strategies before generation. Unsupported versions fail closed;
  there is deliberately no implicit manifest migration.
- All emitted files are syntax-checked and duplicate public declarations are
  rejected before writing generated Rust. This supplements, not replaces, cargo.
- An inventory covers every upstream operation, including unmapped operations.
  `candidate_unverified` is explicitly not a claim of overlay-only support.
- A deterministic declaration inventory supports conservative API review through
  `scripts/check_sdk_api.py --base COMMIT`. Changed/removed declarations fail;
  additions pass. CI additionally requests pinned cargo-semver-checks 0.50.0
  against the PR base once that base contains the facade baseline. Initial
  adoption requires explicit PR review; there is no preexisting facade contract
  on the base branch to preserve. The raw-root to `raw` namespace migration in
  this unreleased PR is intentionally incompatible.
- Streaming media, payload/envelope, wrapper ownership, raw stream lifetime/item
  and Boolean request overrides are validated against both source contracts.
- `codegen.py probe` generates disposable projections and compiles them offline.
  On the pinned spec, 63 additional projections compile; 19 shape candidates
  fail closed with diagnostic reasons. These do not become public API and this
  result proves mechanical generation, not polished product naming.

## Still required before architectural sign-off

- Validate the pinned compiler-aware compatibility tool and its supported
  rustdoc format locally; declaration snapshots alone are insufficient.
- Full pinned generation check, rustfmt, Clippy, Rust tests and doctests on the
  final commit, followed by a successful remote CI run.

## Alternatives: evidence, not a claimed benchmark

Reviewed 2026-09-13:

- [Smithy symbols](https://smithy.io/2.0/guides/building-codegen/decoupling-codegen-with-symbols.html)
  motivates separating schema identity from language names, dependencies and
  output paths. The current adapter adopts a deliberately bounded version:
  exact raw-module lookup and fail-closed public naming, not universal renaming.
- [Fern Rust quickstart](https://buildwithfern.com/learn/sdks/generators/rust/quickstart)
  documents a Rust target, but marks it Enterprise-only and requires login for
  generation. No Fern account was created, no spec uploaded and no commercial
  entitlement assumed. An executable comparison is **not performed**. Either
  authorized access is supplied or this evaluation is explicitly waived on
  deployment/reproducibility grounds; neither is an output-quality verdict.

Comparison protocol when authorized: use the same pinned Mistral spec; generate
Chat, OCR and Models; compile offline fixtures; assess request evolution, union
discriminators, nullable fields, stream ownership/cancellation, structured errors,
public API changes, deterministic regeneration and cost of adding one resource.
Record tool version/configuration and artifacts; do not rely on feature lists.
