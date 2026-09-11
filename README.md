# mistralai-rs

An **unofficial**, async Rust SDK generated from Mistral AI's official OpenAPI
3.1 specification. This project is not affiliated with or endorsed by Mistral AI.

The differentiator is maintenance: the official upstream spec is vendored at an
immutable commit, generation is reproducible, generated Rust is committed, and a
nightly workflow proposes validated SDK updates through a single pull request.
Consumers do not need Python, Just, or the generator to build the crate.

## Install

The package is `mistralai-sdk`; its Rust library is `mistralai`.
`mistralai` is already occupied on crates.io. This bootstrap is distributed from
Git; it has not been published to crates.io:

```toml
[dependencies]
mistralai = { package = "mistralai-sdk", git = "https://github.com/adriendellagaspera/mistralai-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
serde_json = "1"
```

Pin a Git `rev` for reproducible consumer builds. No dependency on `agent-ir` or
any benchmark project exists in this SDK.

## Chat completion

```rust,no_run
use mistralai::{ChatCompletionRequest, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().with_api_key(std::env::var("MISTRAL_API_KEY")?);
    let request: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": "mistral-small-latest",
        "messages": [{"role": "user", "content": "Say hello in French."}],
        "stream": false,
        "max_tokens": 64
    }))?;
    let response = client.chat_completion_v1_chat_completions_post(request).await?;
    println!("{}", response.usage.prompt_tokens);
    println!("{}", response.usage.completion_tokens);
    Ok(())
}
```

The request and response are generated Rust types. JSON is only used above as a
concise way to construct the request; there is no handwritten schema or client
facade. `Client` is a re-export of the generated `HttpClient`.

Run the live example explicitly (this incurs normal Mistral API usage):

```sh
export MISTRAL_API_KEY='your-key'
cargo run --example chat
```

`MISTRAL_MODEL` optionally changes the example's model. The client defaults to
`https://api.mistral.ai`, with a 120-second timeout and no automatic retries of
billable requests. `with_base_url(...)` allows another compatible endpoint or a
local test server. Do not send credentials to an endpoint you do not trust.

## API scope

Initial scope: non-streaming chat completions, list models, retrieve a model,
and all reachable request/response types, including tool calls, structured
outputs and token usage. Operation names follow upstream `operationId`s.

Streaming is not supported by the initial client: keep `stream` false or omit
it. The official spec describes streaming in prose, but the JSON completion
method does not decode SSE. Other API families are outside this initial scope.
See [the generator evaluation](codegen/EVALUATION.md) and
[CONTRIBUTING.md](CONTRIBUTING.md) for extending the selection.

This is a pre-1.0 SDK. Upstream schema fixes and generator upgrades can change
the generated public Rust API; automatic update PRs require human review and
are never merged or published automatically.

## Reproduce the SDK

Prerequisites: Rustup, Python 3.11+, and [Just](https://github.com/casey/just).
Rust/rustfmt is pinned by `rust-toolchain.toml`. The first codegen run installs
the exact generator version with `cargo install --locked` into `.tools/`.

```sh
just generate
just check-generated
cargo test
cargo clippy -- -D warnings
```

No Mistral API key is needed. Generation reads the vendored spec and verifies
its SHA-256; it never fetches a newer spec implicitly. After initial tool and
dependency installation, regeneration can run offline.

`just check-generated` generates into a fresh temporary directory, runs the
generator's own `--check` in a second process, applies pinned rustfmt, and
compares the complete file set and bytes with the committed SDK. It leaves the
checkout unchanged, detects added/deleted files, and does not rely on `git diff`
ignoring untracked files. No timestamps enter the generated output.

## Source and automation

| Location | Purpose |
| --- | --- |
| `codegen.lock` | Upstream repository/path/commit/hash, generator version, Rust version |
| `spec/` | Unmodified official spec and upstream licensing |
| `codegen/` | Generator configuration, one fail-closed preprocessing fix, and evaluation |
| `scripts/` | Acquisition, isolated generation, validation and update tooling |
| `src/generated/` | Committed generated Rust and `REQUIRED_DEPS.toml` |
| `src/lib.rs`, `tests/`, `examples/` | Handwritten exports, offline tests and opt-in example |

The source of truth is
[`mistralai/platform-docs-public/openapi.yaml`](https://github.com/mistralai/platform-docs-public/blob/main/openapi.yaml).
Resolve the exact immutable source through `codegen.lock`.

Two permanent workflows:

- **CI** validates push/PR changes: generation consistency, formatting,
  compilation, Clippy with warnings denied, Rust tests and codegen/update tests.
- **Update Mistral OpenAPI SDK** runs daily at **03:23 UTC** and supports
  `workflow_dispatch`. It compares the spec's content hash, updates provenance,
  regenerates, validates, and creates/updates
  `automation/update-mistral-openapi`. Identical spec bytes are a no-op even if
  upstream has a different commit. The PR contains old/new commits and hashes,
  generator version, operation additions/removals, a diff summary and checks.

Enable **Settings → Actions → General → Workflow permissions → Allow GitHub
Actions to create and approve pull requests** for the updater to create PRs.
The workflow only creates PRs; it does not approve or merge them. No PAT or
GitHub App is needed. Schedules become active after the workflow reaches the
default branch; GitHub may delay scheduled runs or disable schedules after
extended public-repository inactivity.

`GITHUB_TOKEN` pushes do not trigger another push workflow. GitHub's current
behavior can also leave automation-created PR workflows awaiting approval.
Therefore the updater performs **all validation itself before creating or
updating the PR**, and never assumes that CI will run again. If branch rules
require separate PR checks, approve those workflow runs before merging.
See [GitHub's event-trigger documentation](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow).

## License and attribution

Original project contributions: **MIT OR Apache-2.0**. The official specification
retains its **Apache-2.0** terms, including applicable obligations for generated
derivatives. Generator runtime/template material retains applicable MIT notices.
See [NOTICE](NOTICE), [LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE), [spec/LICENSE](spec/LICENSE) and
[codegen/GENERATOR-LICENSE](codegen/GENERATOR-LICENSE).
