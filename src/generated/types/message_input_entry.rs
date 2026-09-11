///Representation of an input message inside the conversation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageInputEntry {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub content: MessageInputEntryContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<MessageInputEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
    pub role: MessageInputEntryRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MessageInputEntryType>,
}
