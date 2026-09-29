use crate::{ApiError, ClientConfig, HttpClient};

pub mod chat_completion_events;
pub use chat_completion_events::ChatCompletionEventsClient;
pub mod judges;
pub use judges::JudgesClient;
pub mod campaigns;
pub use campaigns::CampaignsClient;
pub mod datasets;
pub use datasets::DatasetsClient;
pub mod logs;
pub use logs::LogsClient;
pub mod traces;
pub use traces::TracesClient;
pub mod spans;
pub use spans::SpansClient;
pub struct ObservabilityClient {
    pub http_client: HttpClient,
    pub chat_completion_events: ChatCompletionEventsClient,
    pub judges: JudgesClient,
    pub campaigns: CampaignsClient,
    pub datasets: DatasetsClient,
    pub logs: LogsClient,
    pub traces: TracesClient,
    pub spans: SpansClient,
}

impl ObservabilityClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            chat_completion_events: ChatCompletionEventsClient::new(config.clone())?,
            judges: JudgesClient::new(config.clone())?,
            campaigns: CampaignsClient::new(config.clone())?,
            datasets: DatasetsClient::new(config.clone())?,
            logs: LogsClient::new(config.clone())?,
            traces: TracesClient::new(config.clone())?,
            spans: SpansClient::new(config.clone())?,
        })
    }
}
