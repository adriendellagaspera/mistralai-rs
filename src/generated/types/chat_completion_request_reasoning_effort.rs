///Controls the reasoning effort level for reasoning models. "high" enables comprehensive reasoning traces, "none" disables reasoning effort.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChatCompletionRequestReasoningEffort {
    #[default]
    #[serde(rename = "high")]
    High,
    #[serde(rename = "none")]
    None,
}
