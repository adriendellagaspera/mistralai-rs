//! API client and types for the Mistral AI API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AgentsClient, AudioClient, BatchClient, BetaClient, ChatClient, ClassifiersClient,
    EmbeddingsClient, EventsClient, FilesClient, FimClient, Mistral, ModelsClient, OcrClient,
    WorkflowsClient,
};
pub use types::*;
