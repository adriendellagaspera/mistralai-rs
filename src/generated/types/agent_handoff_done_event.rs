#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentHandoffDoneEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    pub next_agent_id: String,
    pub next_agent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AgentHandoffDoneEventType>,
}
