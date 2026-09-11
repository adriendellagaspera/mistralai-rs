#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AssistantMessage {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub content: Option<Option<AssistantMessageContent>>,
    ///Set this to `true` when adding an assistant message as prefix to condition the model response. The role of the prefix message is to force the model to start its answer by the content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AssistantMessageRole>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tool_calls: Option<Option<Vec<ToolCall>>>,
}
