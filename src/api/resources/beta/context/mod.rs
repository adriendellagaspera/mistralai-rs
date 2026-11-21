use crate::{ApiError, ClientConfig, HttpClient};

pub mod search_indexes;
pub use search_indexes::SearchIndexesClient;
pub struct ContextClient {
    pub http_client: HttpClient,
    pub search_indexes: SearchIndexesClient,
}

impl ContextClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            search_indexes: SearchIndexesClient::new(config.clone())?,
        })
    }
}
