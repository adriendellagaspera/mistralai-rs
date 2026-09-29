//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **agents**
//! - **batch**
//! - **classifiers**
//! - **chat**
//! - **embeddings**
//! - **files**
//! - **fim**
//! - **models**
//! - **ocr**
//! - **workflows**
//! - **events**
//! - **Audio**
//! - **Beta**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod audio;
pub mod batch;
pub mod beta;
pub mod chat;
pub mod classifiers;
pub mod embeddings;
pub mod events;
pub mod files;
pub mod fim;
pub mod models;
pub mod ocr;
pub mod workflows;
pub struct ApiClient {
    pub config: ClientConfig,
    pub agents: AgentsClient,
    pub batch: BatchClient,
    pub classifiers: ClassifiersClient,
    pub chat: ChatClient,
    pub embeddings: EmbeddingsClient,
    pub files: FilesClient,
    pub fim: FimClient,
    pub models: ModelsClient,
    pub ocr: OcrClient,
    pub workflows: WorkflowsClient,
    pub events: EventsClient,
    pub audio: AudioClient,
    pub beta: BetaClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            agents: AgentsClient::new(config.clone())?,
            batch: BatchClient::new(config.clone())?,
            classifiers: ClassifiersClient::new(config.clone())?,
            chat: ChatClient::new(config.clone())?,
            embeddings: EmbeddingsClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            fim: FimClient::new(config.clone())?,
            models: ModelsClient::new(config.clone())?,
            ocr: OcrClient::new(config.clone())?,
            workflows: WorkflowsClient::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            audio: AudioClient::new(config.clone())?,
            beta: BetaClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use audio::AudioClient;
pub use batch::BatchClient;
pub use beta::BetaClient;
pub use chat::ChatClient;
pub use classifiers::ClassifiersClient;
pub use embeddings::EmbeddingsClient;
pub use events::EventsClient;
pub use files::FilesClient;
pub use fim::FimClient;
pub use models::ModelsClient;
pub use ocr::OcrClient;
pub use workflows::WorkflowsClient;
