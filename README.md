# mistralai-rs

Unofficial asynchronous Rust SDK for Mistral AI, generated from a pinned OpenAPI source. This project is not affiliated with Mistral AI.

The package is `mistralai-sdk` and the library is `mistralai`. Install from Git:

```toml
[dependencies]
mistralai = { package = "mistralai-sdk", git = "https://github.com/adriendellagaspera/mistralai-rs" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Pin a Git `rev` for reproducible builds.

```rust,no_run
use mistralai::raw::types::ChatCompletionRequest;
use mistralai::{CompleteChatRequest, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let raw: ChatCompletionRequest = serde_json::from_value(serde_json::json!({
        "model": "mistral-small-latest",
        "messages": [{"role": "user", "content": "Say hello in French."}],
        "max_tokens": 64
    }))?;
    let response = mistral.chat().complete(CompleteChatRequest::from(raw)).await?;
    println!("{}", serde_json::to_string_pretty(response.raw())?);
    Ok(())
}
```

The client defaults to `https://api.mistral.ai`. It does not automatically retry billable requests. `with_base_url(...)` selects another compatible endpoint.

## API

The resource-oriented facade is in `mistralai`; the complete generated transport for the pinned source is in `mistralai::raw`. See [`src/generated/coverage.json`](src/generated/coverage.json) for raw operation coverage. The public facade is derived canonically from the pinned OpenAPI source; no legacy compatibility definition is applied by default. Streaming, multipart and binary download examples are in [`examples/`](examples/).

The published generation input is [`sdk-build/openapi/published.yaml`](sdk-build/openapi/published.yaml), pinned to `mistralai/platform-docs-public/public/openapi.yaml` with 288 source operations. The 0.4.0 cutover intentionally permits reviewed breaking API changes rather than carrying compatibility shims from the previous 0.x surface.

## Development

```sh
just generate
just check-generated
just validate
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for source updates and validation, and the [build responsibility map](sdk-build/README.md) for toolchain boundaries. Generated `src/generated/` and `src/sdk/` files are checked for deterministic regeneration; edit their owning inputs instead.

## License

Original contributions are MIT OR Apache-2.0. The upstream specification and generated material retain their applicable notices; see [NOTICE](NOTICE) and the license files in this repository.
