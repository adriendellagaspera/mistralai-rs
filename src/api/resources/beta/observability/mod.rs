use crate::{ApiError, ClientConfig, HttpClient};

pub mod campaigns;
pub use campaigns::CampaignsClient;
pub mod chat_completion_events;
pub use chat_completion_events::ChatCompletionEventsClient;
pub mod datasets;
pub use datasets::DatasetsClient;
pub mod judges;
pub use judges::JudgesClient;
pub mod logs;
pub use logs::LogsClient;
pub mod spans;
pub use spans::SpansClient;
pub mod traces;
pub use traces::TracesClient;
pub struct ObservabilityClient {
    pub http_client: HttpClient,
    pub campaigns: CampaignsClient,
    pub chat_completion_events: ChatCompletionEventsClient,
    pub datasets: DatasetsClient,
    pub judges: JudgesClient,
    pub logs: LogsClient,
    pub spans: SpansClient,
    pub traces: TracesClient,
}

impl ObservabilityClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            campaigns: CampaignsClient::new(config.clone())?,
            chat_completion_events: ChatCompletionEventsClient::new(config.clone())?,
            datasets: DatasetsClient::new(config.clone())?,
            judges: JudgesClient::new(config.clone())?,
            logs: LogsClient::new(config.clone())?,
            spans: SpansClient::new(config.clone())?,
            traces: TracesClient::new(config.clone())?,
        })
    }
}
