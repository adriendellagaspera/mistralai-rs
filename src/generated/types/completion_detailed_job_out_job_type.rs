#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionDetailedJobOutJobType {
    #[default]
    #[serde(rename = "completion")]
    Completion,
}
