# Development

Install Rustup, Python 3.11+ and Just. Run commands from the repository root.
Linux and macOS are the intended codegen environments; consumers need only Rust.

```sh
just generate
just check-generated
bash scripts/validate.sh
```

Without Just, use `python3 scripts/codegen.py generate` and
`python3 scripts/codegen.py check`. No Python packages or xtask are required.
The Rust toolchain, rustfmt and generator are pinned. The generator installs
into `.tools/` with its published dependency lockfile (`cargo install --locked
--version =0.16.0`). `Cargo.lock` pins this SDK's validation dependencies.

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

1. Add an exact operation selector (`METHOD /path` or upstream operation ID) to
   `codegen/openapi-to-rust.toml`. Keep `prune_models = true` to retain the full
   transitive model closure of selected operations.
2. Regenerate, inspect `REQUIRED_DEPS.toml`, compile and review the new Rust API.
3. Add offline serialization and HTTP behavior tests for the new operation,
   including its unions, optional values and errors where relevant.
4. Update the documented scope and run the complete validation script.

Do not edit `src/generated/`. Prefer a generator configuration correction. If
the generator needs a fix, report/fix it upstream and pin a released version.
Any future spec preprocessing must preserve `spec/openapi.yaml` unchanged,
live in the codegen tooling, and have explicit regression fixtures. Broad
handwritten schemas or post-generation string replacements are not the design.

The initial generation contains one tested preprocessing correction documented
in `codegen/EVALUATION.md`: an undefined `data` required field is removed from
the chat response schema. No benchmark or `agent-ir` dependency belongs in this
generic SDK.

## Tests and live requests

Normal tests use fixtures and local HTTP servers, and require no API key or
external API connection. The chat example is compiled in CI but never run.
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
