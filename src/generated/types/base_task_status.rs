#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BaseTaskStatus {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "TERMINATED")]
    Terminated,
    #[serde(rename = "CONTINUED_AS_NEW")]
    ContinuedAsNew,
    #[serde(rename = "TIMED_OUT")]
    TimedOut,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
