#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ModerationLLMAction {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "block")]
    Block,
}
