# Agent contract

This repository publishes a Mistral-specific Rust SDK generated directly by Fern from a pinned OpenAPI source. Keep implementation detail beside the code that owns it.

## Repository map

- `src/`: canonical Fern-generated Rust SDK, checked in for publication and never hand-edited.
- `sdk-build/openapi/`: pinned canonical upstream OpenAPI source and source-update tooling.
- `sdk-build/fern/product-policy.json`: reviewed Mistral public hierarchy, naming, and streaming policy compiled to Fern OpenAPI extensions.
- `sdk-build/fern/compat.py`: temporary Fern OpenAPI-importer workarounds, each linked to an upstream Fern issue.
- `sdk-build/fern/workarounds.py`: the single generated-output workaround for Fern multipart+SSE.
- `sdk-build/fern/gates.py`: 288-operation closure, type-collision, transport, determinism, and public-inventory gates.
- `sdk-build/official-sdks/`: independent pinned Python/TypeScript surface evidence.
- `.github/scripts/`: repository policy, PR-title validation, and update reporting.

## Canonical local checks

```sh
just policy
just check-generated
just check-source-evidence
just format
just lint
just test-tooling
just test
just docs
just validate
```

Install Python 3.11+, `PyYAML==6.0.2`, Node/npm, Fern CLI `5.112.0`, Rust, and Just before regenerating.

## Gated invariants

- [policy] `CLAUDE.md` MUST contain only `@AGENTS.md`, and root agent instruction files MUST stay within the combined 150-line budget.
- [policy] Nested `AGENTS.md` or `CLAUDE.md` files MUST NOT be added.
- [policy] Third-party GitHub Actions MUST use immutable full commit SHAs, and workflows MUST NOT use `pull_request_target`.
- [generation] `src/` MUST equal deterministic Fern `0.48.0` output from the pinned 288-operation source plus reviewed product policy and explicitly linked temporary Fern workarounds.
- [generation] Every canonical OpenAPI operation MUST be found in emitted Rust by HTTP method/path; no operation may disappear silently.
- [lint] Generated and handwritten Rust MUST pass Clippy correctness and suspicious gates.
- [dependencies] Locked Rust dependencies MUST pass the advisory, license, source, and TLS-backend policy in `deny.toml`.
- [docs] The generated public Rust documentation MUST build successfully.
- [tooling] Remaining Python SDK-build and GitHub scripts MUST have their test suites executed in CI.
- [api] Public API changes MUST keep the machine-readable inventory current and the reviewed compatibility report explicit.
- [gate] Required CI jobs MUST converge on the single `gate` conclusion job before merge.

## Working guidance

Fern is the sole production Rust generator. Product hierarchy/naming belongs in OpenAPI extensions produced from `product-policy.json`. Generic Fern defects belong upstream and may have only the smallest temporary compatibility workaround here. Do not rebuild a generic Rust facade over Fern, and do not add Mistral-specific behavior to a Fern fork.

Comments should explain constraints, provenance, invariants, or non-obvious reasons. Prefer executable gates and local HTTP fixtures to prose reminders. Live Mistral calls are explicit developer operations; credentials and response dumps do not belong in the repository.
