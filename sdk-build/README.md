# SDK build: Fern production path

Fern is the sole production Rust generator.

```text
pinned Mistral OpenAPI (288 operations)
        +
reviewed Mistral product policy
        | x-fern-sdk-group-name / x-fern-sdk-method-name / x-fern-streaming
        +
temporary Fern input compatibility fixes
        |
        v
Fern CLI 5.112.0 + fernapi/fern-rust-sdk 0.48.0
        |
        v
small multipart+SSE workaround (#17928)
        |
        v
288/288 closure + collision + transport + determinism + inventory gates
        |
        v
checked-in src/ + publishable mistralai-sdk
```

The canonical source is `openapi/published.yaml`. The product policy is `fern/product-policy.json`. Generic Fern importer compatibility is isolated in `fern/compat.py`; generated-output compatibility is isolated in `fern/workarounds.py`. Neither layer is a second generator or a compatibility facade.

`python3 sdk-build/fern/build.py generate` writes the projected Fern input, generates the SDK, applies the one output workaround, executes production gates, refreshes `fern/public-inventory.json`, and replaces `src/`. `check` repeats generation twice, proves determinism, validates the inventory, and verifies exact parity with committed `src/`.

Production gates require all 288 source operations to be recoverable from emitted Rust by HTTP method/path, no public resource/method collision, no duplicate generated public type names, the known `JudgeOutput` collision to remain disambiguated, and the five historical transport/representation contracts to stay explicit.

## Temporary upstream Fern issues

- fern-api/fern#17928 — multipart/form-data + SSE Rust generation; the only generated-output shim.
- fern-api/fern#17929 — inline generated type collision with a top-level component.
- fern-api/fern#17930 — intentional `allOf` property narrowing rejected.
- fern-api/fern#17931 — leading-hyphen enum default rejected.
- fern-api/fern#17932 — `x-fern-sdk-group-name` ignored without an explicit SDK method name.

The first migration commit separately repointed the identical legacy `openapi-to-rust` commit from the personal fork to `gpu-cli/openapi-to-rust` and passed the complete old CI, including generation and Rust-build parity. The final Fern path contains no `openapi-to-rust` dependency at all.
