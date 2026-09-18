# mistralai-rs

An **unofficial**, async Rust SDK generated from Mistral AI's official OpenAPI specification. This project is not affiliated with or endorsed by Mistral AI.

The project is built around reproducible generation: the upstream specification, raw generator, generic SDK compiler and raw-binding compatibility package are all pinned to immutable revisions. Generated Rust is committed and CI verifies byte-for-byte regeneration.

## Install

The package is `mistralai-sdk`; its Rust library is `mistralai`. The crate is currently consumed from Git rather than crates.io:

```toml
[dependencies]
mistralai = { package = "mistralai-sdk", git = "https://github.com/adriendellagaspera/mistralai-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Pin a Git `rev` for reproducible consumer builds.

## Example

```rust,no_run
use mistralai::{ChatRequest, Message, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let response = mistral
        .chat()
        .complete(
            ChatRequest::new(
                "mistral-small-latest",
                [Message::user("Say hello in French.")],
            )
            .max_tokens(64),
        )
        .await?;

    println!("{}", response.text().unwrap_or_default());
    Ok(())
}
```

The client defaults to `https://api.mistral.ai`, does not automatically retry billable requests, and supports `with_base_url(...)` for another compatible endpoint or local test server.

## API layers

The complete generated transport is available under `mistralai::raw`. Every operation and schema from the pinned public OpenAPI is accounted for in [`src/generated/coverage.json`](src/generated/coverage.json).

The primary API is a resource-oriented Rust surface compiled from four explicit inputs:

```text
Mistral OpenAPI --------------------------------------> OpenApi
openapi-to-rust -> openapi-to-rust-bindings ---------> Bindings
Mistral semantic policy ------------------------------> Policy
Mistral runtime conventions --------------------------> Runtime
                                                           |
                                                           v
                                                    rust-sdk-generator
                                                           |
                                                           v
                                                        src/sdk
```

`rust-sdk-generator` is a separately versioned, backend-neutral Rust SDK compiler. `openapi-to-rust-bindings` is the separately versioned compatibility package that normalizes the current raw generator output into the compiler's `Bindings` contract. Both are pinned independently in `tooling/sources/lock.json`; neither implementation is copied into this repository.

`mistralai-rs` owns only Mistral-specific concerns around that generic toolchain: product taxonomy, semantic policy, auto-projection, runtime support, source tracking, coverage and public-API review gates.

The idiomatic inventory is recorded separately in [`src/sdk/coverage.json`](src/sdk/coverage.json). Operations not yet projected idiomatically remain available through `mistralai::raw`; generation never invents endpoint behavior to fill a gap.

## Streaming and binary responses

The Chat facade exposes an owned typed stream:

```rust,no_run
use futures_util::StreamExt;
use mistralai::{ChatRequest, Message, Mistral};

# async fn example(mistral: Mistral) -> Result<(), Box<dyn std::error::Error>> {
let mut stream = mistral
    .chat()
    .stream(ChatRequest::new(
        "mistral-small-latest",
        [Message::user("Say hello in French.")],
    ))
    .await?;

while let Some(chunk) = stream.next().await {
    if let Some(text) = chunk?.text() {
        print!("{text}");
    }
}
# Ok(())
# }
```

See [`examples/chat_stream.rs`](examples/chat_stream.rs) for a complete example. SSE parsing is incremental, handles `[DONE]`, bounds buffered event size, never reconnects automatically, and cancels response consumption when the stream is dropped.

Multipart file fields accept `bytes::Bytes`. Generated multipart operations expose additive `*_with_multipart_filenames(..., &[(field, filename)])` variants so filenames are scoped to each call and binary field. Audio arrays are encoded as repeated form fields, and optional absent/null parts are omitted. Binary download variants stream bytes without buffering the successful body.

## Reproduce the SDK

Prerequisites: Git, Rustup, Python 3.11+ with `venv`, and [Just](https://github.com/casey/just).

```sh
just generate
just check-generated
just validate
```

`tooling/sources/lock.json` pins:

- the official Mistral OpenAPI source commit and content hash;
- `openapi-to-rust` plus the reviewed local generator patch hash;
- `rust-sdk-generator` by version, commit and Git tree;
- `openapi-to-rust-bindings` by version, commit and Git tree;
- Rust and Python tooling versions used by generation.

On first use, `tooling/pipeline/build.py` installs the pinned tools into `.tools/`. It verifies their immutable revisions before use. Generation reads the vendored specification; it does not fetch a newer spec implicitly.

`just check-generated` regenerates raw bindings and the idiomatic SDK into fresh temporary directories, runs the raw generator's own check, formats with the pinned Rust toolchain and compares the complete file sets byte-for-byte. No timestamps enter generated output.

## Repository layout

| Location | Purpose |
| --- | --- |
| `tooling/sources/` | Immutable OpenAPI/Python/TypeScript source tracking, vendored OpenAPI and derived taxonomy |
| `tooling/pipeline/` | Mistral adaptation and orchestration across openapi-to-rust, Bindings and rust-sdk-generator |
| `tooling/quality/` | Determinism, coverage, semver/public API review and update reporting |
| `tooling/tests/` | Sidecar integration/policy tests |
| `src/generated/` | Committed raw generated Rust and raw operation inventory |
| `src/sdk/` | Committed idiomatic SDK surface plus Mistral-owned stable error runtime |
| `src/lib.rs`, `src/streaming.rs` | Public exports and Mistral-owned stream support |
| `tests/`, `examples/` | Offline behavior tests and opt-in examples |

See [`tooling/README.md`](tooling/README.md) for the ownership map. The active Mistral semantic policy is [`tooling/pipeline/semantics.json`](tooling/pipeline/semantics.json).

## Automation

CI validates deterministic generation, formatting, compilation, Clippy with warnings denied, Rust tests/docs, coverage inventories and public API evolution.

The scheduled **Update Mistral OpenAPI SDK** workflow checks the official specification and SDK evidence, regenerates and validates a candidate, then opens or updates a review PR. It never automatically merges or publishes an update.

`openapi-to-rust-bindings` owns compatibility tracking against `openapi-to-rust`; this repository consumes reviewed immutable package and generator revisions rather than maintaining that generic compatibility workflow locally.

## License and attribution

Original project contributions are **MIT OR Apache-2.0**. The official specification retains its **Apache-2.0** terms and generator-derived material retains applicable upstream notices. See [NOTICE](NOTICE), [LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE), [tooling/sources/openapi/LICENSE](tooling/sources/openapi/LICENSE) and [tooling/licenses/openapi-to-rust-MIT.txt](tooling/licenses/openapi-to-rust-MIT.txt).
