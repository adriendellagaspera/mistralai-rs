#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ProcessStatus {
    #[default]
    #[serde(rename = "self_managed")]
    SelfManaged,
    #[serde(rename = "missing_content")]
    MissingContent,
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "waiting_for_capacity")]
    WaitingForCapacity,
}
