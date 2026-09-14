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
then applies the versioned semantic manifest. Every resource module and
`facade_types.rs` under `src/sdk/` is a generated artifact carrying an explicit
`@generated` marker. Only the generic error runtime is handwritten.

The IR supports required and optional operation parameters, a JSON body in any
raw argument position, named or inline-union responses, discriminated semantic
unions, recursively adapted simple unions, fluent request projections, response
views, and owned SSE streams. These are protocol-level concepts: no emitter is
specialized for a Mistral resource or schema name.

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

The current public projection covers Chat, OCR, Models, Embeddings, FIM,
Classifiers/Moderations, and the JSON metadata operations for Files. `src/sdk/coverage.json` is the machine-readable boundary
between mapped operations and the remaining raw-only surface.
