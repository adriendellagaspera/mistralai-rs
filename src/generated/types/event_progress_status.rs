#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EventProgressStatus {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
}
