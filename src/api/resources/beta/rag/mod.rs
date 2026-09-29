use crate::{ApiError, ClientConfig, HttpClient};

pub mod ingestion_pipeline_configurations;
pub use ingestion_pipeline_configurations::IngestionPipelineConfigurationsClient;
pub mod search_indexes;
pub use search_indexes::SearchIndexesClient;
pub struct RagClient {
    pub http_client: HttpClient,
    pub ingestion_pipeline_configurations: IngestionPipelineConfigurationsClient,
    pub search_indexes: SearchIndexesClient,
}

impl RagClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            ingestion_pipeline_configurations: IngestionPipelineConfigurationsClient::new(
                config.clone(),
            )?,
            search_indexes: SearchIndexesClient::new(config.clone())?,
        })
    }
}
