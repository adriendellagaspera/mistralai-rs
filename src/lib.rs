//! # Mistral AI API SDK
//!
//! The official Rust SDK for the Mistral AI API.
//!
//! ## Getting Started
//!
//! ```rust
//! use adriendellagaspera_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .agents
//!         .agents_completion_v1agents_completions_post(
//!             &AgentsCompletionRequest {
//!                 agent_id: "agent_id".to_string(),
//!                 messages: vec![AgentsCompletionRequestMessagesItem::User {
//!                     data: UserMessage {
//!                         ..Default::default()
//!                     },
//!                 }],
//!                 frequency_penalty: None,
//!                 guardrails: None,
//!                 max_tokens: None,
//!                 metadata: None,
//!                 n: None,
//!                 parallel_tool_calls: None,
//!                 prediction: None,
//!                 presence_penalty: None,
//!                 prompt_cache_key: None,
//!                 prompt_mode: None,
//!                 random_seed: None,
//!                 reasoning_effort: None,
//!                 response_format: None,
//!                 service_tier: None,
//!                 stop: None,
//!                 stream: None,
//!                 tool_choice: None,
//!                 tools: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
