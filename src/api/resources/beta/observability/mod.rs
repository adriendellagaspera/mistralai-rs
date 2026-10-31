use crate::{ApiError, ClientConfig, HttpClient};

pub mod datasets;
pub use datasets::DatasetsClient;
pub mod evaluations;
pub use evaluations::EvaluationsClient;
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
    pub datasets: DatasetsClient,
    pub evaluations: EvaluationsClient,
    pub judges: JudgesClient,
    pub logs: LogsClient,
    pub spans: SpansClient,
    pub traces: TracesClient,
}

impl ObservabilityClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            datasets: DatasetsClient::new(config.clone())?,
            evaluations: EvaluationsClient::new(config.clone())?,
            judges: JudgesClient::new(config.clone())?,
            logs: LogsClient::new(config.clone())?,
            spans: SpansClient::new(config.clone())?,
            traces: TracesClient::new(config.clone())?,
        })
    }
}
