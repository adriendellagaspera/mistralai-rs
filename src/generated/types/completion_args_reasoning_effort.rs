#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionArgsReasoningEffort {
    #[default]
    #[serde(rename = "high")]
    High,
    #[serde(rename = "none")]
    None,
}
