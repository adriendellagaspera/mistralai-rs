# Agent contract

This repository publishes a Mistral-specific Rust SDK generated directly by Fern from a pinned OpenAPI source.

## Repository map

- `src/`: canonical Fern-generated Rust SDK, checked in for publication and never hand-edited.
- `sdk-build/openapi/`: pinned upstream OpenAPI source and source-update tooling.
- `sdk-build/fern/policy.yaml`: reviewed Mistral resource, method, type and representation policy.
- `sdk-build/fern/prepare_fern_source.py`: temporary input compatibility for identified Fern importer bugs.
- `sdk-build/fern/patch_fern_output.py`: narrow multipart+SSE output workaround for fern-api/fern#17928.
- `sdk-build/fern/build.sh`: compile policy, generate twice, verify closure/provenance/collisions and build generated Rust.
- `sdk-build/fern/check.sh`: require generated Rust and semantic inventory to match the committed production SDK.
- `sdk-build/official-sdks/`: independent pinned Python/TypeScript source evidence.
- `.github/scripts/`: repository policy, PR-title validation and update reporting.

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

Generation additionally requires Fern CLI 5.112.0, PyYAML 6.0.3 and the pinned SDK surface compiler from `adriendellagaspera/rust-sdk-generator@394d12ee893a256123545b75ba5e4a0f8f354ad7` at `.tools/sdk-surface`.

## Gated invariants

- [policy] `CLAUDE.md` MUST contain only `@AGENTS.md`, root agent instructions MUST fit the repository line budget, third-party actions MUST be SHA-pinned, and workflows MUST NOT use `pull_request_target`.
- [generation] Fern 0.48.0 MUST deterministically account for all 288 pinned operations and reproduce committed `src/`.
- [generation] Surface inventory, closure, source provenance and collisions MUST remain verified by the pinned policy compiler/verifier.
- [lint] Generated Rust MUST pass Clippy correctness and suspicious gates.
- [dependencies] Locked Rust dependencies MUST pass advisory, license, source and TLS policy; cargo-shear MUST not report unused direct dependencies.
- [docs] The public Rust documentation and publish dry-run MUST succeed.
- [tooling] Remaining Python build/update scripts and repository scripts MUST keep their tests green.
- [api] Breaking public changes MUST remain explicit in `sdk-build/fern/API_COMPATIBILITY.md`.
- [gate] Required CI jobs MUST converge on the single `gate` conclusion job.

## Working guidance

Fern is the sole production Rust generator. Do not restore `openapi-to-rust`, raw Bindings, the former raw-to-public converter, or a second Rust facade. Product policy belongs in `policy.yaml`; generic Fern defects belong upstream and may have only the smallest ticketed workaround here.

Comments should explain constraints, provenance or non-obvious reasons. Prefer executable gates and local HTTP fixtures to prose reminders. Live Mistral calls are explicit developer operations; credentials and response dumps do not belong in the repository.
