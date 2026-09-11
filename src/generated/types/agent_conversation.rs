#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgentConversation {
    pub agent_id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_version: Option<Option<AgentConversationAgentVersion>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    ///Description of the what the conversation is about.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    pub id: String,
    ///Custom metadata for the conversation.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<MetadataDict>>,
    ///Name given to the conversation.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentConversationObject>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
