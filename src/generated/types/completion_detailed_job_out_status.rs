#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionDetailedJobOutStatus {
    #[default]
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "VALIDATING")]
    Validating,
    #[serde(rename = "VALIDATED")]
    Validated,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "FAILED_VALIDATION")]
    FailedValidation,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "CANCELLATION_REQUESTED")]
    CancellationRequested,
}
