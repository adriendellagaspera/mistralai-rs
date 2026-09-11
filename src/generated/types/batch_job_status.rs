#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BatchJobStatus {
    #[default]
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "TIMEOUT_EXCEEDED")]
    TimeoutExceeded,
    #[serde(rename = "CANCELLATION_REQUESTED")]
    CancellationRequested,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}
