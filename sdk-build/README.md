# SDK build: Fern production path

Fern is the sole production Rust generator.

```text
pinned Mistral OpenAPI (all operations)
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

`bash sdk-build/fern/generate.sh` materializes the production SDK. `bash sdk-build/fern/check.sh` regenerates twice, proves deterministic output, requires complete operation accounting, verifies semantic inventory and exact parity with committed `src/`.

The policy compiler/verifier lives in `sdk_surface/`. It consumes OpenAPI and Fern IR; it does not parse generated Rust and has no external project dependency.

## Temporary upstream Fern issues

- fern-api/fern#17928 — multipart/form-data + SSE Rust generation; generated-output patch.
- fern-api/fern#17929 — inline generated type collision; resolved by reviewed `x-fern-type-name` policy.
- fern-api/fern#17930 — intentional `allOf` property narrowing rejected; input rewrite.
- fern-api/fern#17931 — leading-hyphen enum default rejected; input rewrite.
- fern-api/fern#17932 — resource grouping requires explicit method extension with the pinned Fern importer.
- fern-api/fern#17941 — generated crate docs claim the SDK is official; output wording patch.

No `openapi-to-rust`, raw Bindings, raw-to-public conversion, backend-symbol projection, or generated facade participates in production generation.


## Official SDK parity evidence

The public Tier-1 OpenAPI contract and the private/official SDK surfaces must
remain separate. `official-sdks/surface.json` is the *matched naming evidence*
for published operations; it does **not** measure the remaining official-SDK-only
HTTP routes.

`official-sdks/parity-audit.json` and `official-sdks/PARITY_AUDIT.md` are
deterministic snapshots of **pinned** Python and TypeScript SDK source records.
They report the exact method/path identities absent from the published OpenAPI,
source aliases that could not be reconciled, cross-SDK path divergences and
known Python non-HTTP resource candidates. Shared routes are still *observations*
rather than trusted wire contracts. TypeScript non-HTTP coverage is incomplete.

```sh
python3 sdk-build/official-sdks/update.py update # refresh both snapshots
python3 sdk-build/official-sdks/update.py check  # fail on drift
```

The nightly source-update workflow runs these commands after Fern regeneration,
so an upstream pin update refreshes the taxonomy and Tier-2/Tier-3 evidence in
the **same** reviewable PR. The `tooling` CI job independently checks the
snapshots against the exact pinned SDK revisions.

Tier-2 contract acquisition and scope: [#17](https://github.com/adriendellagaspera/mistralai-rs/issues/17).
Non-HTTP protocols: [#18](https://github.com/adriendellagaspera/mistralai-rs/issues/18).
