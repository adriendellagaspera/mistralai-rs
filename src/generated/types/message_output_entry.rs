#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageOutputEntry {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub content: MessageOutputEntryContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<MessageOutputEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<MessageOutputEntryRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageOutputEntryType>,
}
