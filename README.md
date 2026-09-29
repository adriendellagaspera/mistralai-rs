# mistralai-rs

Unofficial asynchronous Rust SDK for Mistral AI, generated reproducibly with Fern from a pinned official OpenAPI source. This project is not affiliated with Mistral AI.

Install the `mistralai-sdk` package from crates.io; the Rust library is imported as `mistralai_sdk`:

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

    let _ = client;
    Ok(())
}
```

The canonical input is `sdk-build/openapi/published.yaml` with 288 operations. Fern CLI 5.112.0 and `fernapi/fern-rust-sdk` 0.48.0 are the sole production Rust generator. Mistral surface policy is compiled to deterministic Fern extensions by the policy compiler/verifier from `rust-sdk-generator#240`.

The 0.4 Fern cutover intentionally accepts reviewed pre-1.0 breaking changes instead of preserving the previous raw/facade architecture. See `sdk-build/fern/API_COMPATIBILITY.md` and `sdk-build/fern/inventory.json`.

## Development

Install Fern 5.112.0 and PyYAML 6.0.3, and check out `adriendellagaspera/rust-sdk-generator@394d12ee893a256123545b75ba5e4a0f8f354ad7` as `.tools/sdk-surface`.

```sh
just generate
just check-generated
just validate
```

`check-generated` regenerates twice, proves deterministic output, accounts for all 288 source operations, verifies surface closure/provenance/collisions, compiles the generated crate and compares it with committed `src/`.

See `CONTRIBUTING.md` and `sdk-build/README.md` for source updates and ownership boundaries.

## License

Original contributions are MIT OR Apache-2.0. Upstream specification and generated material retain their applicable notices; see `NOTICE`.
