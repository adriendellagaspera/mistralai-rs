#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageOutputEvent {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    pub content: MessageOutputEventContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageOutputEventRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageOutputEventType>,
}
