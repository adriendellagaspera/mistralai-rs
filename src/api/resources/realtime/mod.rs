use crate::{ApiError, ClientConfig, HttpClient};

pub mod sessions;
pub use sessions::SessionsClient;
pub struct RealtimeClient {
    pub http_client: HttpClient,
    pub sessions: SessionsClient,
}

impl RealtimeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
        })
    }
}
