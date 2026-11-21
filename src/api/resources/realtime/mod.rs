use crate::{ApiError, ClientConfig, HttpClient};

pub mod sessions;
pub use sessions::SessionsClient2;
pub struct RealtimeClient {
    pub http_client: HttpClient,
    pub sessions: SessionsClient2,
}

impl RealtimeClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            sessions: SessionsClient2::new(config.clone())?,
        })
    }
}
