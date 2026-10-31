# mistralai-rs

[![OpenAPI compatibility check](https://github.com/adriendellagaspera/mistralai-rs/actions/workflows/monitor-public-openapi.yml/badge.svg?branch=main)](https://github.com/adriendellagaspera/mistralai-rs/actions/workflows/monitor-public-openapi.yml) [![Official SDK synchronization](https://github.com/adriendellagaspera/mistralai-rs/actions/workflows/update-sdk.yml/badge.svg?branch=main)](https://github.com/adriendellagaspera/mistralai-rs/actions/workflows/update-sdk.yml)

These badges reflect the latest scheduled checks on `main`, not a claim of API compatibility. Red means that upstream drift, an unavailable source, or a failed update/validation needs attention. The automation keeps [one synchronization PR](https://github.com/adriendellagaspera/mistralai-rs/pulls?q=is%3Apr+is%3Aopen+head%3Aautomation%2Fupdate-mistral-sources) current and includes a [source-difference report](sdk-build/monitoring/README.md). A passing check means the reviewed pins match the sampled upstream sources; it does not prove runtime behavior against the hosted API.

Unofficial asynchronous Rust SDK for Mistral AI, generated reproducibly with Fern from a pinned official OpenAPI source. This project is not affiliated with Mistral AI.

Install the `mistralai-sdk` package from crates.io; the Rust library is imported as `mistralai_sdk`:

```sh
cargo add mistralai-sdk@0.5
```

```rust,no_run
use mistralai_sdk::{ClientConfig, Mistral};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Mistral::new(ClientConfig {
        api_key: Some(std::env::var("MISTRAL_API_KEY")?),
        ..Default::default()
    })?;

    let _ = client;
    Ok(())
}
```

The canonical input is `sdk-build/openapi/published.yaml` with an explicitly reviewed operation inventory. Fern CLI 5.112.0 and `fernapi/fern-rust-sdk` 0.48.0 are the sole production Rust generator. Mistral surface policy is compiled to deterministic Fern extensions by the repository-local compiler/verifier in `sdk-build/sdk_surface/`.

The 0.5 Fern cutover intentionally accepts reviewed pre-1.0 breaking changes instead of preserving the previous raw/facade architecture. See `sdk-build/fern/API_COMPATIBILITY.md` and `sdk-build/fern/inventory.json`.

## Development

Install Fern 5.112.0 and PyYAML 6.0.3. All project-specific generation and verification tooling is versioned in this repository.

```sh
just generate
just check-generated
just validate
```

`check-generated` regenerates twice, proves deterministic output, accounts for all source operations, verifies surface closure/provenance/collisions, compiles the generated crate and compares it with committed `src/`.

See `CONTRIBUTING.md` and `sdk-build/README.md` for source updates and ownership boundaries.

## License

Original contributions are MIT OR Apache-2.0. Upstream specification and generated material retain their applicable notices; see `NOTICE`.
