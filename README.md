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
use mistralai::{ChatRequest, Message, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mistral = Mistral::new(std::env::var("MISTRAL_API_KEY")?);
    let response = mistral
        .chat()
        .complete(
            ChatRequest::new("mistral-small-latest", [Message::user("Say hello in French.")])
                .max_tokens(64),
        )
        .await?;
    println!("{}", response.text().unwrap_or_default());
    Ok(())
}
```

The client defaults to `https://api.mistral.ai`. It does not automatically retry billable requests. `with_base_url(...)` selects another compatible endpoint.

## API

The resource-oriented facade is in `mistralai`; the complete generated transport for the pinned source is in `mistralai::raw`. See [`src/generated/coverage.json`](src/generated/coverage.json) for raw operation coverage and [`sdk-build/compatibility-definition.json`](sdk-build/compatibility-definition.json) for the published facade. Streaming, multipart and binary download examples are in [`examples/`](examples/).

The published generation input is [`sdk-build/openapi/published.yaml`](sdk-build/openapi/published.yaml). The separately pinned [`public-288.yaml`](sdk-build/openapi/public-288.yaml) is a migration candidate, not the published API. The remaining work and acceptance criteria are tracked in [#134](https://github.com/adriendellagaspera/mistralai-rs/issues/134), [#137](https://github.com/adriendellagaspera/mistralai-rs/issues/137) and [#138](https://github.com/adriendellagaspera/mistralai-rs/issues/138).

## Development

```sh
just generate
just check-generated
just validate
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for source updates and validation, and the [build responsibility map](sdk-build/README.md) for toolchain boundaries. Generated `src/generated/` and `src/sdk/` files are checked for deterministic regeneration; edit their owning inputs instead.

## License

Original contributions are MIT OR Apache-2.0. The upstream specification and generated material retain their applicable notices; see [NOTICE](NOTICE) and the license files in this repository.
