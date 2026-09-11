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
| ownership, borrowing, builders and Rust naming | this crate |
| uncommon or newly-added operations | `mistralai::raw` generated bindings |

The generated OpenAPI layer remains complete and reproducible. The idiomatic SDK
is a projection over it, not a second HTTP implementation.

## Drift policy

Facade request types serialize into the generated request models in tests. If an
upstream schema changes incompatibly, those tests fail instead of silently
changing the public Rust API. New operations remain immediately reachable through
`raw` even before an idiomatic resource method is added.

The Chat + OCR spike deliberately keeps the projection small. It exists to measure
how much semantic glue is required before generalizing the pattern to all Mistral
API families.
