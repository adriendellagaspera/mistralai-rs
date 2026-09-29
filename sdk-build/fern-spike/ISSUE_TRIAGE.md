# rust-sdk-generator issue triage after Fern/Mistral 288

This is the proposed disposition if #234 lands on Outcome A.

| Issue | Proposed disposition | Rationale |
| --- | --- | --- |
| #136 Compare open-source Rust SDK generators | Close as superseded / not planned | The architectural question is now answered by the concrete Fern/Mistral spike. Continuing a broad generator survey no longer changes the selected direction. |
| #137 Progenitor adapter | Close as not planned | A second backend adapter to canonical Bindings is contrary to the Fern-primary simplification. |
| #139 Local-first GitHub Actions regeneration workflow | Retain, re-scope | Still valuable, but should orchestrate Fern + surface-policy/proof checks, not raw backend + adapter + facade emission. |
| #144 Panic/lint audit | Defer / replace after migration | It targets compiler/lowering modules expected to be deleted or substantially reduced. Do not spend migration effort polishing code scheduled for removal; audit the surviving policy/proof compiler afterwards. |
| #172 Optional nullable referenced JSON bodies | Close as obsolete | Fern accepts the full Mistral corpus without this custom request-proof machinery. Generic OpenAPI semantics belong upstream Fern. |
| #185 Duplicate scalar enum wire values | Close as obsolete | Fern accepts the representative Mistral shapes; no custom root structural proof is needed in the target architecture. |
| #189 Typed SSE named oneOf | Close as obsolete | Streaming decoding is Fern generator/runtime responsibility. The Mistral corpus compiles through Fern; any residual generic SSE defect belongs upstream. |
| #197 Anonymous owned stream transports in Bindings | Close as obsolete | Canonical raw transport Bindings are not part of the target production architecture. |
| #200 LangGraph Agent Protocol E2E | Retain, re-scope | Valuable as the independent second fixture: Fern generation + surface policy + coverage/closure/provenance + native/wasm/runtime proof. Remove openapi-to-rust/Bindings-specific acceptance criteria. |
| #202 Remove openapi-to-rust-bindings | Close as superseded | The target removes the whole old backend/adapter path rather than waiting for more openapi-to-rust metadata. |
| #205 Remove openapi-to-rust-bindings | Close as superseded | Same as #202; migration should be tracked by a new Fern-primary architecture issue. |
| #232 Resource-local facade type placement | Close as obsolete | Fern already emits resource-owned modules and nested client hierarchies. There should be no custom Rust facade emitter to fix. |
| #233 Concise semantic public type names | Retain, re-scope | Stable semantic naming remains product surface policy, but implementation should become backend-independent naming policy / Fern overlay + verification, not a Rust facade name allocator. Generic missing naming controls should be upstreamed to Fern. |
| #234 Fern/Mistral 288 decision | Close completed after follow-ups exist | Outcome A is supported once the deterministic gate is recorded and migration/upstream follow-ups are created. |

## Open PR #231

`#231 fix: require constructible public request models` is tied to the current facade architecture. Treat it only as a bounded correctness fix needed to keep current main healthy during migration. Do not use it as a reason to expand the old projection/emission architecture.

## Follow-up work under Outcome A

1. Define a backend-independent SDK surface-policy manifest keyed by stable OpenAPI operation/schema identity.
2. Compile that policy to Fern-compatible overlays/extensions.
3. Consume Fern output/IR only as needed to build publication proof:
   - exact operation accounting;
   - public path/type inventory;
   - closure;
   - provenance;
   - deterministic output;
   - semver/review gates.
4. Migrate `mistralai-rs` generation to Fern.
5. Re-run #200 as the independent non-Mistral proof target.
6. Remove `openapi-to-rust`, `openapi-to-rust-bindings`, canonical raw Bindings, raw/public conversion, and facade emission once the two fixtures prove the replacement path.
7. Rename/reframe the project once generation is no longer its main responsibility.
