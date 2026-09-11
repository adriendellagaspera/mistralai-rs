#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolMessage {
    pub content: Option<ToolMessageContent>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ToolMessageRole>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub tool_call_id: Option<Option<String>>,
}
