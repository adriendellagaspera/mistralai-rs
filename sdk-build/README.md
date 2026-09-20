# SDK build ownership

`sdk-build/` contains only the Mistral-specific inputs, provenance, orchestration and verification of the published SDK. The standalone Rust generators own generic implementation. Python orchestration remains until #113; scripts are not retained merely to preserve a Python package.

## Entry points

| Caller | Purpose |
| --- | --- |
| `just generate` / `sdk-build/build.py generate` | Produce raw bindings and the pinned public facade after source, coverage and exact-parity checks. |
| `just check-generated` / CI generation job | Rebuild in isolation and compare committed output and official-SDK inventory. |
| `just sync-openapi`, `just sync-sdk-surface` / scheduled update | Review upstream OpenAPI and official-SDK pins, then regenerate. |
| `just test-tooling` / CI tooling job | Test the SDK coverage/parity gates and independently test repository scripts in `.github/scripts/`. |
| `just validate` | Run the repository policy, both Python test suites, generation and Rust checks. |
| CI `api` job | Review the public SDK API against the PR base and check compiler-aware semver. |
| `.github/workflows/update-sdk.yml` | Coordinate source updates; delegate PR reporting to `.github/scripts/update_report.py`. |

## Per-file owner, consumer and destination

All paths in the first column are relative to `sdk-build/`; “keep” means they remain at that path.

| File | Responsibility; concrete consumer | Destination |
| --- | --- | --- |
| `build.py` | Pinned orchestration, isolation, raw baseline, coverage and facade parity; Justfile, CI, scheduled workflow and parity tests. | Keep |
| `provenance.lock.json` | Single set of pinned revisions and checksums; build, source updates, SDK harvesting, semver gate, CI cache keys. | Keep |
| `openapi-to-rust.toml` | Mistral raw generator configuration and transport discriminators; invoked by build via pinned raw generator. | Keep |
| `openapi-to-rust-MIT.txt` | Upstream generator license attribution; NOTICE and README. | Keep |
| `sdk-overrides.json` | Reviewed Mistral response representation overrides; pinned SDK generator `derive` input. | Keep |
| `compatibility-definition.json` | Frozen reviewed public SDK definition; pinned SDK generator `generate` input via build. | Keep |
| `coverage-baseline.json` | Reviewed operation identities and allowed overrides; build and coverage gate. | Keep |
| `coverage_gate.py` | Consumer-specific operation coverage regression policy; build and coverage tests. | Keep |
| `test_coverage_gate.py` | Coverage gate unit tests; sdk-build unittest discovery. | Keep |
| `test_publish_parity.py` | Public facade and full-operation parity tests; sdk-build unittest discovery. | Keep (already renamed from `test_migration_parity.py` on main) |
| `check_api_compatibility.py` | SDK-specific API inventory and rustdoc semver review; CI `api` job. | Keep |
| `api-review.json` | Explicit reviewed changes against a particular PR base, consulted by the API gate only when needed. | Keep |
| `openapi/published.yaml` | Exact published Mistral OpenAPI snapshot; raw generation and snapshot check. | Keep unchanged |
| `openapi/LICENSE` | OpenAPI source attribution; README/NOTICE and source updater. | Keep unchanged |
| `openapi/overlays/rust-sdk.overlay.yaml` | Reviewed Mistral specification corrections; raw generator input. | Keep unchanged |
| `openapi/check_published.py` | Pinned source hash and overlay-assumption checks; build. | Keep |
| `openapi/update.py` | Verify published mirror and update Mistral OpenAPI pin/license; Justfile and scheduled update. | Keep |
| `official-sdks/harvest.py` | Extract public paths from pinned official Mistral Python/TypeScript SDKs; official-SDK updater. | Keep |
| `official-sdks/update.py` | Pin, regenerate or check official-SDK surface; Justfile, CI and scheduled update. | Keep |
| `official-sdks/surface.json` | Reviewed Mistral public-path evidence; pinned SDK compiler, updater and PR report. | Keep unchanged |
| `policy.py` | Root agent contract and GitHub workflow policy; Justfile and CI policy job. | Move to `.github/scripts/policy.py` |
| `pr_title.py` | Repository PR-title conventions; CI policy job. | Move to `.github/scripts/pr_title.py` |
| `update_report.py` | Scheduled-update PR reporting from Mistral source changes; update workflow and CONTRIBUTING. | Move to `.github/scripts/update_report.py` |
| `pyproject.toml`, `uv.lock` | Empty dependency metadata; no uv command or package consumer exists. | Delete |

The new `.github/scripts/test_repository_scripts.py` is owned by GitHub automation and executed separately from the SDK-build tests.

## Boundaries and byte parity

`openapi-to-rust` owns generic OpenAPI processing, overlays and raw Rust output; `openapi-to-rust-bindings` owns generic adaptation of raw output to normalized Bindings JSON; `rust-sdk-generator` owns generic derivation, definition validation and Rust SDK emission. This repository owns Mistral source tracking, configuration, taxonomy, reviewed overrides and compatibility, operation coverage, runtime inputs and the final consumer-specific parity checks. Nothing in this cleanup changes those standalone repositories or their pinned revisions.

The historical `tooling/pipeline/compile_sdk.py` marker in generated facade files and the raw source marker normalization in `build.py` are deliberate **byte-for-byte parity contracts**, not active dependencies. Preserve them until a separately reviewed generated-output change. No changes to `src/generated/`, `src/sdk/`, source pins or the public SDK are in scope.
