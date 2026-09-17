# Agent contract

This repository is the Mistral-specific consumer of the standalone Rust SDK
generator toolchain. Keep this file short: it indexes boundaries, commands and
executable contracts; implementation detail belongs beside the code that owns it.

## Repository map

- `src/generated/`: raw `openapi-to-rust` output; generated, never hand-maintained.
- `src/sdk/`: generated idiomatic Mistral facade and committed API/coverage inventories.
- `src/streaming.rs`: handwritten runtime integration for streaming responses.
- `tooling/pipeline/`: Mistral build orchestration and policy inputs.
- `tooling/sources/`: immutable upstream/tool provenance and tracked source inputs.
- `tooling/quality/`: repository-owned quality and compatibility gates.
- `tooling/tests/`: tests for the Mistral-specific build/tooling layer.

## Canonical local checks

Use the repository commands rather than reproducing CI logic by hand:

```sh
just policy
just format
just lint
just test-tooling
just check-generated
just test
just docs
just validate
```

API compatibility and the pinned dependency-security audit additionally run in CI;
the API check needs the PR base revision and the dependency audit owns its tool pin.

## Gated invariants

- [policy] `CLAUDE.md` MUST contain only `@AGENTS.md`, and root agent instruction files MUST stay within the combined 150-line budget.
- [policy] Nested `AGENTS.md` or `CLAUDE.md` files MUST NOT be added; put durable detail in code, tests, contributor docs, or an executable gate.
- [policy] Third-party GitHub Actions MUST use immutable full commit SHAs.
- [policy] Workflows MUST NOT use `pull_request_target`.
- [policy] Pull-request titles MUST follow the repository conventional title grammar.
- [generation] Files under `src/generated/` and generated `src/sdk/` output MUST NOT be edited by hand; change an owning input/tool and regenerate.
- [generation] Mistral OpenAPI, official-SDK taxonomy and generator inputs MUST remain pinned and reproducible through `tooling/sources/lock.json` and the build pipeline.
- [lint] Handwritten Rust MUST remain free of `unsafe`; lint exceptions MUST carry an explicit reason, with generated-code exceptions scoped at module boundaries.
- [dependencies] Locked Rust dependencies MUST pass the advisory, license, source and TLS-backend policy in `deny.toml`.
- [docs] Handwritten public Rust API MUST have rustdoc that builds warning-free; fallible public helpers MUST document their error contract.
- [tooling] Repository-owned Python build/source/quality behavior MUST have its tooling test suite executed in CI.
- [api] Changes to the committed public SDK surface MUST pass the repository API compatibility review against the PR base.
- [gate] Required CI jobs MUST converge on the single `gate` conclusion job before merge.

## Working guidance

Put a change at the narrowest owning boundary. Generic generator or
`openapi-to-rust-bindings` behavior belongs in `rust-sdk-generator`, not here.
Mistral naming/projection, source tracking, runtime integration and compatibility
policy belong here. Preserve deterministic generated output unless the issue
explicitly owns an output migration.

Comments should explain constraints, provenance, invariants or non-obvious reasons,
not narrate straightforward code. Prefer regression fixtures to prose reminders.
Normal tests use fixtures/local servers; live Mistral calls are explicit developer
operations and credentials or response dumps do not belong in the repository.
