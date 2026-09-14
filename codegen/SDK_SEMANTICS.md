# SDK semantic layer

The official OpenAPI document is the source of truth for the wire contract. It is
not, by itself, the desired Rust public API.

Mistral's official Python and TypeScript SDKs use a resource-oriented generated
surface (`chat.complete`, `chat.stream`, `ocr.process`, etc.) rather than exposing
OpenAPI `operationId` values directly. This spike mirrors that stable taxonomy
where it is observable in the public SDKs, while making Rust-specific choices for
ownership, builders, enums and error handling.

## Sources of truth

| Concern | Source |
| --- | --- |
| paths, verbs, payloads and response schemas | official Mistral OpenAPI |
| resource and method taxonomy | official Mistral Python/TypeScript SDKs |
| ownership, borrowing, builders and Rust naming | `codegen/sdk-semantics.json` and the facade generator |
| uncommon or newly-added operations | `mistralai::raw` generated bindings |

The generated OpenAPI layer remains complete and reproducible. The idiomatic SDK
is generated as a projection over it, not implemented as a second HTTP client.
`codegen/sdk_facade.py` reads the actual raw Rust structs and client methods,
then applies the versioned semantic manifest. The committed `src/sdk/mod.rs`,
`chat.rs`, and `ocr.rs` files are generated artifacts and carry an explicit
`@generated` marker. Only the generic error runtime is handwritten.

## Drift policy

Every codegen run regenerates the raw bindings first and the facade second.
Optional request fields are discovered from the raw Rust structs and receive
fluent setters automatically. The generator validates configured operation
signatures and raw types. A new required field or an incompatible signature
therefore fails closed with a semantic-review diagnostic instead of silently
changing the public Rust API. `codegen.py check` compares both layers
byte-for-byte. The nightly uses the same pipeline.

New operations remain immediately reachable through `raw` even before a new
resource mapping is deliberately added to the semantic manifest.

The Chat + OCR spike deliberately keeps the projection small. It exists to measure
how much semantic glue is required before generalizing the pattern to all Mistral
API families.
