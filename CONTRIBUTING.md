# Development

Install Git, Rustup, Python 3.11+ with `venv`, and Just. Run commands from the repository root.
Linux and macOS are the intended codegen environments; consumers need only Rust.

```sh
just generate
just check-generated
bash scripts/validate.sh
```

Without Just, use `python3 scripts/codegen.py generate` and
`python3 scripts/codegen.py check`. No xtask is needed. The tool installs
`ruamel.yaml` at the version in `codegen.lock` into an isolated environment in
`.tools/`; no global Python package installation is needed.

The Rust toolchain, rustfmt, generator source commit, generator patch SHA-256,
and YAML parser version are pinned. Codegen fetches the immutable generator
commit, checks/applies `codegen/patches/generator.patch`, and builds it with its
committed dependency lockfile (`cargo install --locked --path ...`). The install
cache includes both source commit and patch hash. `Cargo.lock` pins this SDK's
validation dependencies. Tools can be installed online once and reused offline.

## Updating the source specification

```sh
just update-spec
just generate
bash scripts/validate.sh
python3 scripts/update_report.py
git diff -- spec codegen.lock src/generated
```

`update-spec` resolves the most recent commit touching `openapi.yaml`, fetches
the spec at that immutable SHA, and only changes the checkout if its SHA-256
differs. It downloads upstream licensing at the same revision. Optional
`GH_TOKEN` raises the GitHub API rate limit; no secret is required. A failed
download does not replace the pinned spec. No endpoint credential is used.

Review API changes before committing. If new generated dependencies appear in
`src/generated/REQUIRED_DEPS.toml`, reconcile them with `Cargo.toml` and update
`Cargo.lock` explicitly. The nightly uses `--locked` and fails rather than
silently choosing new dependencies. Generator upgrades are separate reviewed
changes to `codegen.lock`, never an automatic side effect of a spec update.

## Extending the API

There is no operation allowlist: all upstream operations and schemas are
included (`prune_models = false`). New upstream operations are picked up on
regeneration. `src/generated/coverage.json` records method, path, operation ID,
Rust method, tags, and success media for each operation. Coverage validation
rejects missing methods and runtime stubs before replacing committed output.

Review new generated dependencies, request encodings, response media and tests.
Add offline HTTP/serialization fixtures for new behavior. Stream variants for
mixed JSON/SSE endpoints and the WAV voice-sample variant are explicit aliases
in `codegen/preprocess.py`; the generator removes fragments from request URLs.
The new methods are generated from the same upstream schemas.

Do not edit `src/generated/`. Prefer generator configuration, then a narrowly
scoped source patch or explicit preprocessing. Source patches must apply with
`git apply --check` to the pinned commit and have their SHA-256 updated in
`codegen.lock`. They change the generator, not its output. Rustfmt remains the
only post-generation Rust transform. Submit/removal-track fixes upstream when
possible; a future released generator should replace these local patches.

See `codegen/EVALUATION.md` for the three upstream schema contradictions and
multipart/streaming generator fixes. Preserve the original spec exactly.
No benchmark or `agent-ir` dependency belongs in this generic SDK.

## Tests and live requests

Normal tests use fixtures and local HTTP servers, and require no API key or
external API connection. Both chat examples are compiled in CI but never run.
To make a live request explicitly, set `MISTRAL_API_KEY` and run
`cargo run --example chat`. Do not commit credentials or live response dumps.

## Automation failures

The update workflow validates before writing its stable PR branch. If upstream
generation, compilation, Clippy or tests fail, the run stays failed and no new
candidate is pushed. Inspect the failing step, reproduce with `just update-spec`
locally, and fix the generator configuration or tests as appropriate. Existing
update PRs still require review: successful codegen does not prove wire-level
compatibility for every upstream change.

The workflow's hash comparison avoids commits for metadata-only upstream
changes. A second run against the same pending upstream update reuses the same
branch/PR; merging it makes subsequent runs no-ops. Do not enable automatic merge
or add a PAT just to get a second CI run: the nightly already runs the same gates.

## Contributions

Unless explicitly stated otherwise, contributions are licensed under MIT OR
Apache-2.0. Preserve all third-party license and attribution notices.
