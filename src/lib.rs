//! Unofficial Mistral AI SDK generated from the official OpenAPI specification.
//!
//! The primary API is resource-oriented and intentionally hides mechanical
//! OpenAPI naming. The complete generated API remains available through `raw`.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    clippy::allow_attributes_without_reason,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

#[allow(
    missing_docs,
    clippy::allow_attributes_without_reason,
    clippy::double_must_use,
    clippy::nonminimal_bool,
    clippy::redundant_field_names,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::collapsible_if,
    clippy::empty_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "openapi-to-rust output is generated and kept behind this module boundary"
)]
mod generated;

/// Complete generated OpenAPI bindings and transport client.
pub mod raw {
    pub use crate::generated::{client, types};
    pub use client::HttpClient as Client;
}

#[allow(
    missing_docs,
    clippy::allow_attributes_without_reason,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "the idiomatic SDK facade is generated and validated by generation/API gates"
)]
mod sdk;
pub mod streaming;

pub use sdk::*;
