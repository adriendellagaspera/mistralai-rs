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
    dead_code,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    clippy::allow_attributes_without_reason,
    clippy::clone_on_copy,
    clippy::double_must_use,
    clippy::nonminimal_bool,
    clippy::redundant_field_names,
    clippy::too_many_arguments,
    clippy::match_single_binding,
    clippy::collapsible_if,
    clippy::empty_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::unnecessary_to_owned,
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
    rustdoc::broken_intra_doc_links,
    clippy::allow_attributes_without_reason,
    clippy::large_enum_variant,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::redundant_field_names,
    clippy::useless_conversion,
    clippy::wrong_self_convention,
    reason = "the idiomatic SDK facade is generated and validated by generation/API gates"
)]
mod sdk;
pub mod streaming;

pub use sdk::*;
