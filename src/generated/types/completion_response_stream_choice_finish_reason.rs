#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum CompletionResponseStreamChoiceFinishReason {
    #[default]
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "null")]
    NullValue,
}
