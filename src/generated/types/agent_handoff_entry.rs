#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentHandoffEntry {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub next_agent_id: String,
    pub next_agent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<AgentHandoffEntryObject>,
    pub previous_agent_id: String,
    pub previous_agent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AgentHandoffEntryType>,
}
