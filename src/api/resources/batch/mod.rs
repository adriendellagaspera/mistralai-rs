use crate::{ApiError, ClientConfig, HttpClient};

pub mod jobs;
pub use jobs::JobsClient;
pub struct BatchClient {
    pub http_client: HttpClient,
    pub jobs: JobsClient,
}

impl BatchClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            jobs: JobsClient::new(config.clone())?,
        })
    }
}
