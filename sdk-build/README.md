# SDK build: Fern production path

Fern is the sole production Rust generator.

```text
pinned Mistral OpenAPI (288 operations)
        +
small reviewed surface policy
        |
        v
repository-local surface policy compiler/verifier
        |
        v
deterministic Fern extensions + Fern IR verification
        |
        v
Fern CLI 5.112.0 + fernapi/fern-rust-sdk 0.48.0
        |
        v
ticketed Fern output workarounds (#17928, #17941)
        |
        v
closure + provenance + collisions + determinism + Rust build
        |
        v
checked-in src/ + publishable mistralai-sdk
```

The canonical source is `openapi/published.yaml`. The Mistral-specific policy is `fern/policy.yaml`. `fern/prepare_fern_source.py` contains temporary importer compatibility only, while `fern/patch_fern_output.py` contains narrow, ticketed generated-output patches.

`bash sdk-build/fern/generate.sh` materializes the production SDK. `bash sdk-build/fern/check.sh` regenerates twice, proves deterministic output, requires 288/288 accounting, verifies semantic inventory and exact parity with committed `src/`.

The policy compiler/verifier lives in `sdk_surface/`. It consumes OpenAPI and Fern IR; it does not parse generated Rust and has no external project dependency.

## Temporary upstream Fern issues

- fern-api/fern#17928 — multipart/form-data + SSE Rust generation; generated-output patch.
- fern-api/fern#17929 — inline generated type collision; resolved by reviewed `x-fern-type-name` policy.
- fern-api/fern#17930 — intentional `allOf` property narrowing rejected; input rewrite.
- fern-api/fern#17931 — leading-hyphen enum default rejected; input rewrite.
- fern-api/fern#17932 — resource grouping requires explicit method extension with the pinned Fern importer.
- fern-api/fern#17941 — generated crate docs claim the SDK is official; output wording patch.

No `openapi-to-rust`, raw Bindings, raw-to-public conversion, backend-symbol projection, or generated facade participates in production generation.
