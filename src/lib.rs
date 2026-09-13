//! Unofficial Mistral AI SDK generated from the official OpenAPI specification.
//!
//! The primary API is resource-oriented and intentionally hides mechanical
//! OpenAPI naming. The complete generated API remains available through `raw`.

// openapi-to-rust 0.16.0 emits these mechanical style patterns.
// Keep exceptions scoped to generated code; no compiler/correctness lint is
// disabled here. Evidence and removal criteria: codegen/EVALUATION.md.
#[allow(
    clippy::double_must_use,
    clippy::nonminimal_bool,
    clippy::redundant_field_names,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::collapsible_if,
    clippy::empty_docs
)]
mod generated;

/// Complete generated OpenAPI bindings and transport client.
pub mod raw {
    pub use crate::generated::client::HttpClient as Client;
    pub use crate::generated::types;
}

mod sdk;
pub mod streaming;

pub use sdk::{
    ApiError, ChatRequest, ChatResponse, ChatStream, ChatStreamChunk, Message, Mistral, OcrPage,
    OcrRequest, OcrResponse, SdkError, TransportError, TransportErrorKind,
};
pub use sdk::{chat, error, ocr};
