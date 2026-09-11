#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChatCompletionChoiceFinishReason {
    #[default]
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "model_length")]
    ModelLength,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
