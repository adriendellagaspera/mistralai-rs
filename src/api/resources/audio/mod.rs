use crate::{ApiError, ClientConfig, HttpClient};

pub mod speech;
pub use speech::SpeechClient;
pub mod transcriptions;
pub use transcriptions::TranscriptionsClient;
pub mod voices;
pub use voices::VoicesClient;
pub struct AudioClient {
    pub http_client: HttpClient,
    pub speech: SpeechClient,
    pub transcriptions: TranscriptionsClient,
    pub voices: VoicesClient,
}

impl AudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            speech: SpeechClient::new(config.clone())?,
            transcriptions: TranscriptionsClient::new(config.clone())?,
            voices: VoicesClient::new(config.clone())?,
        })
    }
}
