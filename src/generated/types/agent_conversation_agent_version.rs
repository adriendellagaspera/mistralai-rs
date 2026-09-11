#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AgentConversationAgentVersion {
    String(String),
    Integer(i64),
}
