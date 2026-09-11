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
`https://api.mistral.ai` and does not automatically retry billable requests.
`with_base_url(...)` allows another compatible endpoint or a local test server.
Do not send credentials to an endpoint you do not trust.

## API scope

All **173 operations** and **481 component schemas** from the pinned official
specification are included, including deprecated and beta endpoints. Four
additional generated methods select alternative response media, for **177
methods** in total. Names follow upstream `operationId`s.

| API families | Coverage |
| --- | --- |
| Chat, FIM, models, embeddings, classifiers, OCR | Generated typed requests/responses |
| Agents, conversations, connectors | Includes beta endpoints and conversation SSE |
| Files, batch, fine-tuning, libraries/documents/accesses | Includes multipart uploads and binary downloads |
| Audio transcription, speech, voices | Multipart arrays/files, transcription/speech SSE, WAV sample download |
| Observability: datasets, records, judges, campaigns, completion events | All operations declared upstream |
| Workflows: executions, runs, schedules, deployments, events, metrics, workers | All operations, including SSE feeds |

The exact machine-readable inventory is
[`src/generated/coverage.json`](src/generated/coverage.json). Generation fails
if an operation is missing or a generated method contains a configuration stub.
This measures **OpenAPI coverage**, not live-service verification: offline tests
exercise representative wire behavior and edge cases, and beta/deprecated APIs
retain upstream stability and availability constraints. No live tests run in CI.
Undocumented service features and media absent from the spec are not inferred.

### Streaming and uploads

Use the generated methods ending in `_stream` for chat, FIM and speech; native
conversation/transcription/workflow streaming methods retain their upstream
names. These return incremental byte streams. `streaming::json_events` decodes
JSON data into generated types, while `streaming::events` preserves SSE metadata.
For chat/FIM, deserialize the data as `CompletionChunk`, **not** `CompletionEvent`
(which describes the envelope). `Event::envelope` handles envelope schemas such
as `ConversationEvents`, `TranscriptionStreamEvents`, and `SpeechStreamEvents`.

```rust,ignore
let bytes = client.chat_completion_v1_chat_completions_post_stream(request).await?;
let events = mistralai::streaming::json_events::<_, _, mistralai::CompletionChunk>(bytes);
```

See [`examples/chat_stream.rs`](examples/chat_stream.rs) for a complete example
(`cargo run --example chat_stream`). JSON and SSE methods set a declared Boolean
`stream` field to match their response contract. SSE parsing is incremental,
handles `[DONE]`, and limits each buffered event to 1 MiB. It never reconnects
or replays billable requests automatically. Transport, JSON and size errors are
returned to the caller. Dropping the stream cancels consumption.

Multipart file fields accept `bytes::Bytes`. Use
`client.with_upload_filename("input.jsonl")` to set the filename sent with uploads
(the default is `upload`). This setting applies to file parts on that client;
clone it for concurrent uploads with different filenames. Audio arrays are sent
as repeated form fields, and absent/null optional parts are omitted.
Use `get_voice_sample_audio_v1_audio_voices_voice_id_sample_get_wav` for WAV bytes;
the original method retains the spec's JSON response variant.

This is a pre-1.0 SDK. Upstream schema fixes and generator upgrades can change
the generated public Rust API; automatic update PRs require human review and
are never merged or published automatically.

## Reproduce the SDK

Prerequisites: Git, Rustup, Python 3.11+ with `venv`, and [Just](https://github.com/casey/just).
Rust/rustfmt is pinned by `rust-toolchain.toml`. The first codegen run installs
the generator at an immutable source commit, verifies/applies a small source
patch, and compiles with `cargo install --locked` into `.tools/`. An isolated
Python environment installs pinned `ruamel.yaml` for YAML 1.2 preprocessing.

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
| `codegen.lock` | Upstream repository/path/commit/hash, generator version/source commit/patch hash, YAML parser and Rust versions |
| `spec/` | Unmodified official spec and upstream licensing |
| `codegen/` | Generator configuration, explicit preprocessing repairs, source patch and evaluation |
| `scripts/` | Acquisition, isolated generation, validation and update tooling |
| `src/generated/` | Committed generated Rust, dependency manifest and operation inventory |
| `src/lib.rs`, `src/streaming.rs`, `tests/`, `examples/` | Handwritten exports/SSE decoder, offline tests and opt-in examples |

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
