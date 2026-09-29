# mistralai-rs

Unofficial asynchronous Rust SDK for Mistral AI, generated reproducibly with Fern from a pinned official OpenAPI source. This project is not affiliated with Mistral AI.

Install the `mistralai-sdk` package from crates.io; the Rust library name is `mistralai_sdk`:

```sh
cargo add mistralai-sdk@0.4
```

```rust,no_run
use mistralai_sdk::{ApiClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new(ClientConfig {
        api_key: Some(std::env::var("MISTRAL_API_KEY")?),
        ..Default::default()
    })?;

    // Resources include client.chat, client.files, client.audio,
    // client.workflows, client.beta, and the remaining API groups.
    let _ = client;
    Ok(())
}
```

The canonical input is `sdk-build/openapi/published.yaml` with 288 operations. Fern CLI `5.112.0` and `fernapi/fern-rust-sdk` `0.48.0` are the only production Rust-generation path. Mistral product naming and streaming policy is compiled to Fern OpenAPI extensions; temporary Fern bugs are isolated separately and linked to upstream issues.

The 0.4 Fern cutover intentionally accepts reviewed pre-1.0 breaking changes instead of preserving the previous raw/facade architecture. See `sdk-build/fern/API_COMPATIBILITY.md` and the machine-readable `sdk-build/fern/public-inventory.json`.

## Development

```sh
python3 -m pip install 'PyYAML==6.0.2'
npm install --global 'fern-api@5.112.0'
just generate
just check-generated
just validate
```

`just check-generated` regenerates twice, proves deterministic output, accounts for all 288 source operations in emitted Rust, checks type collisions and the reviewed transport representations, and compares the result with committed `src/`.

See `CONTRIBUTING.md` and `sdk-build/README.md` for source updates, ownership boundaries, and release gates.

## License

Original contributions are MIT OR Apache-2.0. Upstream specification and generated material retain their applicable notices; see `NOTICE` and the license files in this repository.
