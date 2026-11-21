use crate::{ApiError, ClientConfig, HttpClient};

pub mod agents;
pub use agents::AgentsClient2;
pub mod sessions;
pub use sessions::SessionsClient;
pub mod speech;
pub use speech::SpeechClient;
pub mod transcriptions;
pub use transcriptions::TranscriptionsClient;
pub mod voices;
pub use voices::VoicesClient;
pub struct AudioClient {
    pub http_client: HttpClient,
    pub agents: AgentsClient2,
    pub sessions: SessionsClient,
    pub speech: SpeechClient,
    pub transcriptions: TranscriptionsClient,
    pub voices: VoicesClient,
}

impl AudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            agents: AgentsClient2::new(config.clone())?,
            sessions: SessionsClient::new(config.clone())?,
            speech: SpeechClient::new(config.clone())?,
            transcriptions: TranscriptionsClient::new(config.clone())?,
            voices: VoicesClient::new(config.clone())?,
        })
    }
}
