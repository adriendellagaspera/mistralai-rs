//! Unofficial Mistral AI SDK, generated from the official OpenAPI specification.
//!
//! The public API is generated. See `codegen.lock` for provenance and the
//! repository README for supported operations and regeneration instructions.

// openapi-to-rust 0.16.0 emits these three mechanical style patterns.
// Keep exceptions scoped to generated code; no compiler/correctness lint is
// disabled here. Evidence and removal criteria: codegen/EVALUATION.md.
#[allow(
    clippy::double_must_use,
    clippy::nonminimal_bool,
    clippy::redundant_field_names
)]
pub mod generated;

pub use generated::client::HttpClient as Client;
pub use generated::types::*;
